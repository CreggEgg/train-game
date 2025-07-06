use bevy::prelude::*;
use rand::{Rng, SeedableRng};

use crate::{GameState, train_plugin::Train};

#[derive(Clone)]
pub enum Stop {
    Town,
    InitialStop,
}

#[derive(Resource)]
pub struct GameWorld {
    rng: rand_chacha::ChaCha8Rng,
}

#[derive(Resource)]
pub struct NextStop {
    pub stop: Stop,
    pub distance: f32,
}
#[derive(Resource)]
pub struct CurrentStop(pub Option<Stop>);

#[derive(Event)]
pub struct GenerateNextStop;

pub fn world_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Loading), generate_world)
        .add_observer(
            |_trigger: Trigger<GenerateNextStop>,
             mut next_stop: ResMut<NextStop>,
             mut game_world: ResMut<GameWorld>,
             train: Query<&Train>| {
                *next_stop =
                    generate_next_stop(&mut game_world.rng, train.single().unwrap().distance);
            },
        );
}

fn generate_world(mut commands: Commands) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);

    commands.insert_resource(CurrentStop(Some(Stop::InitialStop)));
    commands.insert_resource(generate_next_stop(&mut rng, 0.));

    commands.insert_resource(GameWorld { rng });
}

fn generate_next_stop(rng: &mut impl Rng, current_distance: f32) -> NextStop {
    let distance = rng.random_range(
        30.0..=70.0, /*units now in meters but i made these very small to make it easy to test*/
    ) + current_distance;
    info!("Random f32: {}", distance);

    NextStop {
        stop: Stop::Town,
        distance,
    }
}
