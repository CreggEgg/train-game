use bevy::prelude::*;

use crate::{
    GameState,
    train_plugin::{CAR_SIZE, MaxPixelHeightOfTrain, TrainStats},
};

#[derive(Default, Resource)]
struct CameraSpeeds {
    vx: f32,
    vy: f32,
}

pub fn camera_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_camera);
    app.init_resource::<CameraSpeeds>();
    app.add_systems(Update, move_camera.run_if(in_state(GameState::InGame)));
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
    ));
}

const CAMERA_MOVE_SPEED: f32 = 120.0;
const CAMERA_ACCELERATION: f32 = 360.0;

fn move_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    train_stats: Res<TrainStats>,
    height: Res<MaxPixelHeightOfTrain>,
    mut camera_speeds: ResMut<CameraSpeeds>,
) {
    let train_length = train_stats.length as f32 * CAR_SIZE;

    let mut acelx = 0.0f32;
    let mut acely = 0.0f32;

    if keys.pressed(KeyCode::ArrowLeft) {
        acelx -= 1.0;
    }

    if keys.pressed(KeyCode::ArrowRight) {
        acelx += 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        acelx -= 1.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        acelx += 1.0;
    }

    if keys.pressed(KeyCode::ArrowDown) {
        acely -= 1.0;
    }

    if keys.pressed(KeyCode::ArrowUp) {
        acely += 1.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        acely -= 1.0;
    }

    if keys.pressed(KeyCode::KeyW) {
        acely += 1.0;
    }

    if camera_speeds.vx.partial_cmp(&0.0) != acelx.partial_cmp(&0.0) {
        camera_speeds.vx = acelx * CAMERA_MOVE_SPEED;
    }

    let acelx = acelx * CAMERA_ACCELERATION * time.delta_secs();

    camera_speeds.vx += acelx * CAMERA_ACCELERATION * time.delta_secs();

    if camera_speeds.vy.partial_cmp(&0.0) != acely.partial_cmp(&0.0) {
        camera_speeds.vy = acely * CAMERA_MOVE_SPEED;
    }

    let acely = acely * CAMERA_ACCELERATION * time.delta_secs();

    camera_speeds.vy += acely;

    let mut trans /* rights */ = camera.single_mut().unwrap();

    trans.translation.x += camera_speeds.vx * time.delta_secs() + 0.5 * acelx * time.delta_secs();

    trans.translation.x = trans.translation.x.clamp(0.0, train_length);

    trans.translation.y += camera_speeds.vy * time.delta_secs() + 0.5 * acely * time.delta_secs();

    trans.translation.y = trans
        .translation
        .y
        .clamp(-500.0 + 50.0, height.height + 500.0)
}
