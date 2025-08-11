use std::time::Duration;

use bevy::prelude::*;

use crate::{
    GameState, ImageAssets, InGameState, MainGameObject, animations::Animation,
    train_plugin::MaxPixelHeightOfTrain,
};

#[derive(Component, Clone)]
pub struct Roost {
    pub birds: Vec<Bird>,
}

#[derive(Component, Clone)]
pub struct Bird {
    pub out: bool,
}

impl Default for Roost {
    fn default() -> Self {
        Self { birds: vec![] }
    }
}

#[derive(Event)]
enum BirdEvent {
    SendOut { bird: usize, roost: Entity },
    Return { bird: usize, roost: Entity },
}

pub fn roost_menu(builder: &mut ChildSpawnerCommands, roost: &Roost, building: Entity) {
    // Children::spawn(SpawnIter(
    //     roost
    //         .birds
    //         .iter()
    //         .cloned()
    //         .map(|bird| Text::new("This is a bird")),
    // ))
    for (i, bird) in roost.birds.iter().enumerate() {
        builder
            .spawn(Node {
                ..Default::default()
            })
            .with_children(|parent| {
                let parent_id = parent.target_entity();
                if bird.out {
                    parent.spawn(out_button());
                } else {
                    parent.spawn(send_out_button()).observe(
                        move |_trigger: Trigger<Pointer<Pressed>>,
                              mut commands: Commands,
                              mut send_out_events: EventWriter<BirdEvent>| {
                            send_out_events.write(BirdEvent::SendOut {
                                bird: i,
                                roost: building,
                            });
                            commands
                                .entity(parent_id)
                                .despawn_related::<Children>()
                                .with_child(out_button());
                        },
                    );
                }
            });
    }
}
fn send_out_button() -> impl Bundle + use<> {
    (
        TextColor::BLACK,
        Text::new("This bird in"),
        BackgroundColor(Color::srgb(1.0, 0.0, 0.)),
        Pickable::default(),
    )
}

fn out_button() -> impl Bundle + use<> {
    (TextColor::BLACK, Text::new("This bird is out"))
}

pub fn bird_plane_plugin(app: &mut App) {
    app.add_event::<BirdEvent>().add_systems(
        Update,
        (send_birds_out, update_birds)
            .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running))),
    );
}

fn send_birds_out(
    mut ev: EventReader<BirdEvent>,
    mut roosts: Query<(&GlobalTransform, &mut Roost)>,
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
) {
    for ev in ev.read() {
        match ev {
            BirdEvent::SendOut {
                bird: bird_idx,
                roost: roost_entity,
            } => {
                let (transform, mut roost) = roosts.get_mut(*roost_entity).unwrap();
                let bird = roost.birds.get_mut(*bird_idx).unwrap();
                bird.out = true;
                commands.spawn(spawn_bird(
                    bird,
                    transform.translation().xy() + Vec2::Y * 40.0,
                    transform.translation().xy() + Vec2::Y * 30.0,
                    &image_assets,
                    *roost_entity,
                    *bird_idx,
                    Duration::ZERO,
                ));
            }
            BirdEvent::Return { bird, roost } => {
                let (_, mut roost) = roosts.get_mut(*roost).unwrap();
                let bird = roost.birds.get_mut(*bird).unwrap();
                bird.out = false;
            }
        };
    }
}

#[derive(Component)]
pub struct BirdTimer(pub Timer);
#[derive(Component)]
pub struct BirdReturnData {
    pub location: Vec2,
    pub roost: Entity,
    pub bird: usize,
}

fn update_birds(
    mut birds: Query<(Entity, &mut Transform, &mut BirdTimer, &BirdReturnData), With<Bird>>,
    height: Res<MaxPixelHeightOfTrain>,
    mut commands: Commands,
    time: Res<Time>,
    mut ev: EventWriter<BirdEvent>,
) {
    for (bird_entity, mut bird, mut timer, bird_return) in &mut birds {
        if bird.translation.y >= (height.height + 100.0).max(1000.0) && !timer.0.finished() {
            timer.0.tick(time.delta());
        } else {
            bird.translation.y +=
                time.delta_secs() * 100.0 * if timer.0.finished() { -1.0 } else { 1.0 };
            if bird.translation.y <= bird_return.location.y {
                commands.entity(bird_entity).despawn();
                ev.write(BirdEvent::Return {
                    bird: bird_return.bird,
                    roost: bird_return.roost,
                });
            }
        }
    }
}

pub fn spawn_bird(
    bird: &Bird,
    translation: Vec2,
    return_translation: Vec2,
    image_assets: &ImageAssets,
    roost_entity: Entity,
    bird_index: usize,
    time_progress: Duration,
) -> impl Bundle {
    (
        MainGameObject,
        bird.clone(),
        Transform::from_translation(
            translation.extend(4.0), /* transform.translation() + Vec3::Y * 40.0 */
        ),
        Sprite::from_image(image_assets.bird_plane_away_1.clone()),
        BirdTimer(
            Timer::new(Duration::from_secs_f32(3.0), TimerMode::Once)
                .tick(time_progress)
                .clone(),
        ),
        BirdReturnData {
            location: return_translation, /* transform.translation().xy() + Vec2::Y * 30.0 */
            roost: roost_entity,
            bird: bird_index,
        },
        Animation(
            vec![
                image_assets.bird_plane_away_1.clone(),
                image_assets.bird_plane_away_2.clone(),
                image_assets.bird_plane_away_3.clone(),
                image_assets.bird_plane_away_4.clone(),
                image_assets.bird_plane_away_5.clone(),
                image_assets.bird_plane_away_6.clone(),
                image_assets.bird_plane_away_7.clone(),
                image_assets.bird_plane_away_8.clone(),
                image_assets.bird_plane_away_9.clone(),
                image_assets.bird_plane_away_10.clone(),
                image_assets.bird_plane_away_11.clone(),
                image_assets.bird_plane_away_12.clone(),
                image_assets.bird_plane_away_13.clone(),
                image_assets.bird_plane_away_14.clone(),
                image_assets.bird_plane_away_15.clone(),
                image_assets.bird_plane_away_16.clone(),
                image_assets.bird_plane_away_17.clone(),
                image_assets.bird_plane_away_18.clone(),
                image_assets.bird_plane_away_19.clone(),
                image_assets.bird_plane_away_20.clone(),
                image_assets.bird_plane_away_21.clone(),
                image_assets.bird_plane_away_22.clone(),
                image_assets.bird_plane_away_23.clone(),
                image_assets.bird_plane_away_24.clone(),
                image_assets.bird_plane_away_25.clone(),
                image_assets.bird_plane_away_26.clone(),
                image_assets.bird_plane_away_27.clone(),
                image_assets.bird_plane_away_28.clone(),
                image_assets.bird_plane_away_29.clone(),
                image_assets.bird_plane_away_30.clone(),
                image_assets.bird_plane_away_31.clone(),
                image_assets.bird_plane_away_32.clone(),
                image_assets.bird_plane_away_33.clone(),
                image_assets.bird_plane_away_34.clone(),
            ],
            0,
        ),
    )
}
