use bevy::prelude::*;

use crate::{
    GameState, ImageAssets,
    world_plugin::{CurrentStop, GenerateNextStop, NextStop},
};

mod train_speed_ui;

#[derive(Resource)]
pub struct TrainStats {
    length: usize,
    acceleration: f32,
    max_velocity: f32,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum TrainState {
    #[default]
    Stopped,
    Advancing,
}

pub fn train_plugin(app: &mut App) {
    app.insert_resource(TrainStats {
        length: 1,
        acceleration: 1.0,
        max_velocity: 27.0,
    })
    .add_event::<AdvanceEvent>()
    .init_state::<TrainState>()
    .add_systems(OnEnter(GameState::InGame), spawn_train)
    .add_systems(OnEnter(GameState::InGame), train_speed_ui::make_ui)
    .add_systems(
        FixedPostUpdate,
        train_speed_ui::update_train_speed.run_if(in_state(GameState::InGame)),
    )
    .add_systems(
        FixedUpdate,
        (
            start_advancing.run_if(in_state(TrainState::Stopped)),
            move_train.run_if(in_state(TrainState::Advancing)),
        ),
    );
}

const CAR_SIZE: f32 = 140.0;

#[derive(Component)]
pub struct Locomotive;
#[derive(Component)]
pub struct TrainCar;
#[derive(Component)]
pub struct Train {
    pub distance: f32,
    pub velocity: f32,
}

#[derive(Event)]
pub struct AdvanceEvent;

fn spawn_train(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    train_stats: Res<TrainStats>,
) {
    commands
        .spawn((
            Visibility::default(),
            Transform::default(),
            Train {
                distance: 0.,
                velocity: 0.,
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_image(image_assets.train_locomotive.clone()),
                Name::new("Locomotive"),
                Locomotive,
            ));
            for i in 0..train_stats.length {
                parent.spawn((
                    Sprite::from_image(image_assets.train_car.clone()),
                    Name::new(format!("Car{i}")),
                    TrainCar,
                    Transform::from_xyz(CAR_SIZE * (i as f32 + 1.), 0., 0.),
                ));
            }
        });
}

fn start_advancing(
    mut ev: EventReader<AdvanceEvent>,
    mut next_state: ResMut<NextState<TrainState>>,
    mut current_stop: ResMut<CurrentStop>,
) {
    for _ in ev.read() {
        info!("Starting to advance!");
        next_state.set(TrainState::Advancing);

        current_stop.0 = None;
    }
}

fn move_train(
    mut train: Query<&mut Train>,
    train_stats: Res<TrainStats>,
    mut current_stop: ResMut<CurrentStop>,
    next_stop: Res<NextStop>,
    mut next_state: ResMut<NextState<TrainState>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let mut train = train.single_mut().unwrap();
    train.velocity += train_stats.acceleration * time.delta_secs();

    train.velocity = train.velocity.min(train_stats.max_velocity);

    train.distance += train.velocity * time.delta_secs();
    info!("Distance: {}", train.distance);

    if next_stop.distance - train.distance < 0.1 {
        info!(
            "{} - {} = {}",
            next_stop.distance,
            train.distance,
            next_stop.distance - train.distance
        );
        info!("Stopping");
        next_state.set(TrainState::Stopped);
        train.velocity = 0.0;

        train.velocity = 0.0;

        current_stop.0 = Some(next_stop.stop.clone());

        commands.trigger(GenerateNextStop);
    }
}
