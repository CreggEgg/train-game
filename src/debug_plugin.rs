use bevy::prelude::*;

use crate::GameState;

pub fn debug_plugin(mut app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), skip_main_menu);
}

fn skip_main_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Loading);
}
