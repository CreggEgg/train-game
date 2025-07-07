use std::ops::DerefMut;

use bevy::prelude::*;

use crate::{
    GameState,
    build_plugin::{BuildLocation, MAX_CONSTRUCTION_SNAPPING},
    goblins::Goblin,
};

const SKIP_MAIN_MENU: bool = true;
const LOG_DISTANCE: bool = true;
const BUILD_LOCATION_GIZMO: bool = true;
const ZOOM_CAMERA_OUT: bool = true;
const GOBLIN_KILL: bool = true;

pub fn debug_plugin(app: &mut App) {
    if SKIP_MAIN_MENU {
        app.add_systems(OnEnter(GameState::MainMenu), skip_main_menu);
    }
    if BUILD_LOCATION_GIZMO {
        app.add_systems(Update, build_location_gizmo);
    }
    if ZOOM_CAMERA_OUT {
        app.add_systems(Update, zoom_camera_out);
    }
    if GOBLIN_KILL {
        app.add_systems(Update, kill_goblins);
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

fn zoom_camera_out(keys: Res<ButtonInput<KeyCode>>, mut camera: Query<&mut Projection>) {
    if keys.just_pressed(KeyCode::KeyO) {
        if let Ok(a) = &mut camera.single_mut() {
            match a.deref_mut() {
                Projection::Orthographic(orthographic_projection) => {
                    orthographic_projection.scale = 100.0
                }
                _ => panic!("different camera projection don't know what to do with"),
            }
        }
    }

    if keys.just_released(KeyCode::KeyO) {
        if let Ok(a) = &mut camera.single_mut() {
            match a.deref_mut() {
                Projection::Orthographic(orthographic_projection) => {
                    orthographic_projection.scale = 10.0
                }
                _ => panic!("different camera projection don't know what to do with"),
            }
        }
    }
}

fn kill_goblins(
    keys: Res<ButtonInput<KeyCode>>,
    goblins: Query<Entity, With<Goblin>>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyK) {
        for g in goblins {
            commands.entity(g).despawn();
        }
    }
}
