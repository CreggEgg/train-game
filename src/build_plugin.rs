use core::f32;

use bevy::{math::FloatPow, prelude::*, window::PrimaryWindow};

use crate::{GameState, ImageAssets, InGameState, train_plugin::TrainCar};

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum BuildState {
    Building,
    #[default]
    NotBuilding,
}

#[derive(Component)]
pub struct BuildLocation(pub Vec2);

#[derive(Component)]
struct GhostBuilding;

pub fn build_plugin(app: &mut App) {
    app.init_state::<BuildState>()
        .add_event::<BuildEvent>()
        .add_systems(
            Update,
            construct_buildings.run_if(
                in_state(GameState::InGame)
                    .and(in_state(InGameState::Running))
                    .and(in_state(BuildState::Building)),
            ),
        )
        .add_systems(FixedUpdate, on_build.run_if(in_state(BuildState::Building)))
        .add_systems(OnEnter(GameState::InGame), spawn_ghost);
}

fn spawn_ghost(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        GhostBuilding,
        Sprite::from_image(image_assets.farm.clone()),
        Transform::from_xyz(0., 0., 5.0),
    ));
}

pub const MAX_CONSTRUCTION_SNAPPING: f32 = 40.0;

#[derive(Event)]
pub struct BuildEvent {
    child_of: Entity,
    offset: Vec2,
}

fn construct_buildings(
    window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    mut ghost: Query<(&mut Sprite, &mut Transform), With<GhostBuilding>>,
    build_locations: Query<(Entity, &BuildLocation, &GlobalTransform, &ChildOf)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut ev: EventWriter<BuildEvent>,
    mut commands: Commands,
) {
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };
    if let Some(position) = window
        .cursor_position()
        .and_then(|position| camera.viewport_to_world_2d(camera_transform, position).ok())
    {
        let (mut ghost_sprite, mut ghost_transform) = ghost.single_mut().unwrap();

        let mut closest: Option<(f32, Entity, &BuildLocation, &GlobalTransform, &ChildOf)> = None;
        for (build_entity, build_location, build_transform, build_parent) in build_locations {
            let closest_distance = closest
                .map(|(distance, _, _, _, _)| distance)
                .unwrap_or(MAX_CONSTRUCTION_SNAPPING.squared());
            let distance = (position - (build_transform.translation().xy() + build_location.0))
                .length_squared();
            if distance < closest_distance {
                closest = Some((
                    distance,
                    build_entity,
                    build_location,
                    build_transform,
                    build_parent,
                ));
            }
        }
        if let Some((_, build_entity, build_location, build_transform, build_parent)) = closest {
            ghost_sprite.color = Color::srgb(0.0, 1., 0.);
            ghost_transform.translation =
                build_transform.translation() + build_location.0.extend(5.0);
            if buttons.just_pressed(MouseButton::Left) {
                commands.entity(build_entity).despawn();
                ev.write(BuildEvent {
                    child_of: build_parent.0,
                    offset: build_location.0,
                });
            }
        } else {
            ghost_sprite.color = Color::srgb(1.0, 0., 0.);
            ghost_transform.translation = position.extend(5.);
        }
    }
}

fn on_build(
    mut ev: EventReader<BuildEvent>,
    parents: Query<Entity, With<Transform>>,
    image_assets: Res<ImageAssets>,
    mut commands: Commands,
) {
    for BuildEvent { child_of, offset } in ev.read() {
        let parent = parents.get(*child_of).unwrap();
        let building = commands
            .spawn((
                Sprite::from_image(image_assets.farm.clone()),
                Transform::from_translation(offset.extend(4.0)),
                // children![(BuildLocation(Vec2::new(0., 40.)), Transform::default())],
            ))
            .id();
        commands.entity(parent).add_child(building);
    }
}
