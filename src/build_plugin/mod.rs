use core::f32;
use std::time::Duration;

use bevy::{math::FloatPow, platform::collections::HashMap, prelude::*, window::PrimaryWindow};
use building_menus::BuildingInspected;

use crate::{
    GameState, ImageAssets, InGameState,
    resources_plugin::{Inventory, Item},
    train_plugin::TrainState,
    ui_state::InMenu,
};

// #[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
// pub enum BuildState {
//     Building,
//     #[default]
//     NotBuilding,
// }
mod building_menus;

#[derive(Resource, Clone, Copy)]
pub enum BuildingType {
    Housing,
    Farm,
    Storage,
}

impl BuildingType {
    fn get_texture(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            BuildingType::Housing => image_assets.housing.clone(),
            BuildingType::Farm => image_assets.farm.clone(),
            _ => image_assets.debug_building.clone(),
        }
    }
    fn get_build_locations(&self) -> Vec<Vec2> {
        match self {
            BuildingType::Housing => vec![Vec2::new(0., 40.)],
            BuildingType::Farm => vec![],
            BuildingType::Storage => vec![Vec2::new(0., 40.)],
        }
    }

    fn iterator() -> impl Iterator<Item = Self> {
        [Self::Housing, Self::Farm, Self::Storage].into_iter()
    }

    fn name(&self) -> &'static str {
        match self {
            BuildingType::Housing => "Housing",
            BuildingType::Farm => "Farm",
            BuildingType::Storage => "Storage",
        }
    }

    fn get_resource_production(&self) -> Option<ResourceProduction> {
        match self {
            BuildingType::Housing => None,
            BuildingType::Farm => Some(ResourceProduction(
                Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
                Item::Food,
            )),
            BuildingType::Storage => None,
        }
    }
}

#[derive(Component)]
pub struct BuildLocation(pub Vec2);

#[derive(Component)]
struct GhostBuilding;

#[derive(Component)]
pub struct Building(BuildingType);

#[derive(Component)]
pub struct ResourceProduction(pub Timer, pub Item);

pub fn build_plugin(app: &mut App) {
    app //.init_state::<BuildState>()
        .insert_resource(BuildingType::Farm)
        .add_event::<BuildEvent>()
        .add_plugins(building_menus::building_menus_plugin)
        .add_systems(
            Update,
            (construct_buildings, change_selected_building).run_if(
                in_state(GameState::InGame)
                    .and(in_state(InGameState::Running))
                    .and(in_state(InMenu::BuildMenu)),
            ),
        )
        .add_systems(
            FixedUpdate,
            update_ghost.run_if(
                in_state(InMenu::BuildMenu)
                    .and(in_state(GameState::InGame))
                    .and(resource_exists::<BuildingType>)
                    .and(resource_changed::<BuildingType>),
            ),
        )
        .add_systems(
            OnEnter(InMenu::BuildMenu),
            |mut ghost: Query<&mut Visibility, With<BuildMenuItem>>| {
                for mut build_menu_item in &mut ghost {
                    *build_menu_item = Visibility::Visible;
                }
            },
        )
        .add_systems(
            OnExit(InMenu::BuildMenu),
            |mut ghost: Query<&mut Visibility, With<BuildMenuItem>>| {
                for mut build_menu_item in &mut ghost {
                    *build_menu_item = Visibility::Hidden;
                }
            },
        )
        .add_systems(FixedUpdate, on_build.run_if(in_state(InMenu::BuildMenu)))
        .add_systems(
            OnEnter(GameState::InGame),
            (spawn_ghost, spawn_blueprint_window),
        )
        .add_systems(
            FixedUpdate,
            produce_resources.run_if(
                in_state(GameState::InGame)
                    .and(in_state(InGameState::Running))
                    .and(in_state(TrainState::Advancing)),
            ),
        );
}

#[derive(Component)]
struct BuildMenuItem;

fn spawn_ghost(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        Visibility::Hidden,
        BuildMenuItem,
        GhostBuilding,
        Sprite::from_image(image_assets.farm.clone()),
        Transform::from_xyz(0., 0., 5.0),
    ));
}

fn update_ghost(
    mut ghost: Query<&mut Sprite, With<GhostBuilding>>,
    building_type: Res<BuildingType>,
    image_assets: Res<ImageAssets>,
) {
    let mut ghost = ghost.single_mut().unwrap();
    ghost.image = building_type.get_texture(&image_assets);
}

#[derive(Component)]
struct BluePrintButton(BuildingType);

fn spawn_blueprint_window(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::splat(80), 1, 1, None, Some(UVec2::new(200, 60)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let texture_atlas = TextureAtlas::from(texture_atlas_handle);
    commands
        .spawn((
            Visibility::Hidden,
            BuildMenuItem,
            Node {
                top: Val::Vh(5.0),
                right: Val::Px(0.),
                display: Display::Flex,
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::End,
                align_items: AlignItems::FlexEnd,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::top(Val::Px(10.0)),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            for building_type in BuildingType::iterator() {
                parent.spawn((
                    Node {
                        width: Val::Px(142.0),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    children![
                        (
                            ImageNode::from_atlas_image(
                                building_type.get_texture(&*image_assets),
                                texture_atlas.clone(),
                            ),
                            Node {
                                width: Val::Px(142.0),
                                height: Val::Px(142.0),
                                bottom: Val::Px(0.0),

                                ..Default::default()
                            },
                            BluePrintButton(building_type),
                            Button,
                        ),
                        (
                            Node {
                                width: Val::Percent(100.0),
                                display: Display::Flex,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,

                                ..default()
                            },
                            children![Text::new(building_type.name()),],
                        ),
                    ],
                ));
            }
        });
}

fn change_selected_building(
    interaction_query: Query<
        (&Interaction, &BluePrintButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut current_building: ResMut<BuildingType>,
) {
    for (interaction, BluePrintButton(building_type)) in &interaction_query {
        if *interaction == Interaction::Pressed {
            *current_building = *building_type;
        }
    }
}

pub const MAX_CONSTRUCTION_SNAPPING: f32 = 40.0;

#[derive(Event)]
pub struct BuildEvent {
    child_of: Entity,
    offset: Vec2,
    building_type: BuildingType,
}

fn construct_buildings(
    window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    mut ghost: Query<(&mut Sprite, &mut Transform), With<GhostBuilding>>,
    build_locations: Query<(Entity, &BuildLocation, &GlobalTransform, &ChildOf)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut ev: EventWriter<BuildEvent>,
    mut commands: Commands,
    building_type: Res<BuildingType>,
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
                    building_type: *building_type,
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
    for BuildEvent {
        child_of,
        offset,
        building_type,
    } in ev.read()
    {
        let parent = parents.get(*child_of).unwrap();
        let mut building = commands.spawn((
            Sprite::from_image(building_type.get_texture(&image_assets)),
            Transform::from_translation(offset.extend(4.0)),
            Building(*building_type),
            // children![(BuildLocation(Vec2::new(0., 40.)), Transform::default())],
            //
            Pickable::default(),
        ));
        if let Some(resource_production) = building_type.get_resource_production() {
            building.insert(resource_production);
        }
        building.with_children(|parent| {
            for build_location in building_type.get_build_locations() {
                parent.spawn((BuildLocation(build_location), Transform::default()));
            }
        });
        match building_type {
            BuildingType::Storage => {
                building.insert(Inventory::default());
            }
            _ => {}
        }

        let building_id = building.id();
        building.observe(
            move |mut trigger: Trigger<Pointer<Click>>,
                  mut selected_building: ResMut<BuildingInspected>,
                  mut menu_state: ResMut<NextState<InMenu>>,
                  current_menu_state: Res<State<InMenu>>| {
                if let InMenu::None = **current_menu_state {
                    println!("got click");
                    selected_building.0 = Some(building_id);
                    menu_state.set(InMenu::BuildingMenu);
                    trigger.propagate(false);
                }
            },
        );
        commands.entity(parent).add_child(building_id);
    }
}

fn produce_resources(
    mut buildings: Query<&mut ResourceProduction>,
    mut inventories: Query<&mut Inventory>,
    time: Res<Time>,
) {
    let mut produced_items = HashMap::new();
    for mut building in &mut buildings {
        if building.0.tick(time.delta()).just_finished() {
            *produced_items.entry(building.1.clone()).or_insert(0) += 1;
        }
    }
    for (item, amount) in produced_items {
        for mut inventory in &mut inventories {
            *inventory.items.entry(item.clone()).or_insert(0) += amount;
            break;
        }
    }
}
