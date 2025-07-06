use bevy::prelude::*;

use crate::GameState;

const SKIP_MAIN_MENU: bool = true;
const LOG_DISTANCE: bool = true;

pub fn debug_plugin(app: &mut App) {
    if SKIP_MAIN_MENU {
        app.add_systems(OnEnter(GameState::MainMenu), skip_main_menu);
    }
}

fn skip_main_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Loading);
}
