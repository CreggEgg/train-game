use bevy::prelude::*;

use crate::GameState;

pub fn camera_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        IsDefaultUiCamera,
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::FixedVertical {
                viewport_height: 100.0,
            },
            scale: 10.0,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0., 300.0, 0.),
    ));
}
