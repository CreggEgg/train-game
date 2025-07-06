use bevy::prelude::*;

use crate::{
    GameState,
    build_plugin::{BuildLocation, MAX_CONSTRUCTION_SNAPPING},
};

const SKIP_MAIN_MENU: bool = true;
const LOG_DISTANCE: bool = true;
const BUILD_LOCATION_GIZMO: bool = true;

pub fn debug_plugin(app: &mut App) {
    if SKIP_MAIN_MENU {
        app.add_systems(OnEnter(GameState::MainMenu), skip_main_menu);
    }
    if BUILD_LOCATION_GIZMO {
        app.add_systems(Update, build_location_gizmo);
    }
}

fn build_location_gizmo(
    mut gizmos: Gizmos,
    build_locations: Query<(&BuildLocation, &GlobalTransform)>,
) {
    for (build_location, parent_transform) in build_locations {
        gizmos.circle_2d(
            parent_transform.translation().xy() + build_location.0,
            MAX_CONSTRUCTION_SNAPPING,
            Color::srgb(1.0, 0., 0.),
        );
    }
}

fn skip_main_menu(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Loading);
}
