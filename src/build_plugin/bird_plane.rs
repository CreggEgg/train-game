use bevy::{ecs::spawn::SpawnIter, image, prelude::*};

use crate::{GameState, ImageAssets, InGameState, animations::Animation};

#[derive(Component, Clone)]
pub struct Roost {
    birds: Vec<Bird>,
}

#[derive(Component, Clone)]
struct Bird {
    out: bool,
}

impl Default for Roost {
    fn default() -> Self {
        Self {
            birds: vec![Bird { out: false }],
        }
    }
}

#[derive(Event)]
struct SendOut {
    bird: usize,
    roost: Entity,
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
            .spawn(
                (Node {
                    ..Default::default()
                }),
            )
            .with_children(|parent| {
                let parent_id = parent.target_entity();
                if bird.out {
                    parent.spawn(out_button());
                } else {
                    parent.spawn(send_out_button()).observe(
                        move |trigger: Trigger<Pointer<Pressed>>,
                              mut commands: Commands,
                              mut send_out_events: EventWriter<SendOut>| {
                            send_out_events.write(SendOut {
                                bird: i,
                                roost: building,
                            });
                            commands
                                .entity(parent_id.clone())
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
    app.add_event::<SendOut>().add_systems(
        Update,
        (send_birds_out, update_birds)
            .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running))),
    );
}

fn send_birds_out(
    mut ev: EventReader<SendOut>,
    mut roosts: Query<(&GlobalTransform, &mut Roost)>,
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
) {
    for ev in ev.read() {
        let SendOut { bird, roost } = ev;
        let (transform, roost) = roosts.get_mut(*roost).unwrap();
        let bird = roost.birds.get(*bird).unwrap();
        commands.spawn((
            bird.clone(),
            Transform::from_translation(transform.translation()),
            Sprite::from_image(image_assets.bird_plane_away_1.clone()),
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
        ));
    }
}

fn update_birds(mut birds: Query<&mut Transform, With<Bird>>, time: Res<Time>) {
    for mut bird in &mut birds {
        bird.translation.y += time.delta_secs() * 100.0;
    }
}
