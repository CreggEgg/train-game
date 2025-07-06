use bevy::prelude::*;

use crate::GameState;

pub enum Stop {
    Town,
}

#[derive(Resource)]
pub struct GameWorld {
    stops: Vec<Stop>,
}

pub fn world_plugin(mut app: &mut App) {
    app.add_systems(OnEnter(GameState::Loading), generate_world);
}

fn generate_world() {}
