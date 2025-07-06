use bevy::prelude::*;
use rand::{Rng, SeedableRng};

use crate::GameState;

pub enum Stop {
    Town { distance: f32 },
    InitialStop,
}

#[derive(Resource)]
pub struct GameWorld {
    rng: rand_chacha::ChaCha8Rng,
}

#[derive(Resource)]
pub struct NextStop(Stop);
#[derive(Resource)]
pub struct CurrentStop(Option<Stop>);

pub fn world_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Loading), generate_world);
}

fn generate_world(mut commands: Commands) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(10);
    println!("Random f32: {}", rng.random_range(3000.0..=7000.0));

    commands.insert_resource(CurrentStop(Some(Stop::InitialStop)));
    commands.insert_resource(NextStop(Stop::Town {
        distance: rng.random::<f32>(),
    }));

    commands.insert_resource(GameWorld { rng });
}
