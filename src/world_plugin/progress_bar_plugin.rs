use std::f32::consts::PI;

use bevy::{ecs::entity, prelude::*, render::{camera::Viewport, view::visibility}};
use lerp;

use crate::{
    GameState, ImageAssets, InGameState, MainGameObject,
    train_plugin::Train,
    world_plugin::{CurrentStop, NextStop, NumberedStop, Stop},
};

fn reset_resources(mut anim_timer: ResMut<AnimTimer>, mut last_stop_dist: ResMut<LastStopDist>) {
    *anim_timer = AnimTimer {
        time: 20.,
        right_slide_amount: 0.,
        swapped: false,
    };
    *last_stop_dist = LastStopDist(0.);
}

pub fn progress_bar_plugin(app: &mut App) {
    app.insert_resource(AnimTimer {
        time: 20.,
        right_slide_amount: 0.,
        swapped: false,
    })
    .insert_resource(LastStopDist(0.))
    .add_systems(OnEnter(GameState::InGame), spawn_progress_bar)
    .add_systems(OnEnter(GameState::MainMenu), reset_resources)
    .add_systems(
        Update,
        (
            (update_last_stop_dist, start_anim_timer).run_if(
                resource_exists::<CurrentStop>.and(
                    resource_changed::<CurrentStop>
                        .and(in_state(GameState::InGame))
                        .and(in_state(InGameState::Running)),
                ),
            ),
            update_progress_bar
                .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running))),
            animate_progress_bar
                .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running))),
        ),
    );
}

#[derive(Component)]
struct MapPin {
    right: bool,
}

#[derive(Component)]
struct MapLocomotive;

#[derive(Resource)]
struct AnimTimer {
    time: f32,
    right_slide_amount: f32,
    swapped: bool,
}

#[derive(Resource)]
struct LastStopDist(f32);


const MAP_PIN_OFFSET_RIGHT: f32 = -12.;
const MAP_PIN_OFFSET_LEFT: f32 = -28.;
const MAP_LOCO_OFFSET_RIGHT: f32 = -69.1;
const MAP_LOCO_OFFSET_LEFT: f32 = -84.5;


fn spawn_progress_bar(mut commands: Commands, image_assets: Res<ImageAssets>, window_query: Query<&Window>) {
    let window = window_query.single().unwrap();
    let bar_width = window.resolution.width() * 0.6;
    //println!("{}", bar_width);

    commands.spawn((
        MainGameObject,
        Node {
            width: Val::Vw(60.0),
            height: Val::Px(14.5),
            bottom: Val::Vh(2.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::Center,
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        BorderRadius::MAX,
        BackgroundColor(Color::srgba(0.7, 0.7, 0.7, 0.55)),
        children![
            (
                Node {
                    width: Val::Px(512.0 * 0.08),
                    height: Val::Px(512.0 * 0.08),
                    bottom: Val::Percent(47.),
                    //right: Val::Percent(96.4),
                    right: Val::Px(bar_width + MAP_PIN_OFFSET_LEFT),
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ImageNode::new(image_assets.map_pin.clone())
                    .with_color(Color::linear_rgba(0.45, 0.45, 0.45, 0.65)),
                MapPin { right: false },
            ),
            (
                Node {
                    width: Val::Px(512.0 * 0.08),
                    height: Val::Px(512.0 * 0.08),
                    bottom: Val::Percent(47.),
                    //right: Val::Percent(-1.6),
                    right: Val::Px(MAP_PIN_OFFSET_RIGHT),
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ImageNode::new(image_assets.map_pin.clone())
                    .with_color(Color::linear_rgba(0.45, 0.45, 0.45, 0.65)),
                MapPin { right: true },
            ),
            (
                Node {
                    width: Val::Px(480.0 * 0.3),
                    height: Val::Px(270.0 * 0.3),
                    bottom: Val::Percent(-125.),
                    //right: Val::Percent(89),
                    right: Val::Px(MAP_LOCO_OFFSET_RIGHT),
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ImageNode::new(image_assets.train_locomotive.clone())
                    .with_color(Color::linear_rgba(0.75, 0.75, 0.75, 0.85)),
                MapLocomotive,
            ),
        ],
    ));
}

fn update_progress_bar(
    last_stop_dist: Res<LastStopDist>,
    next_stop: Res<NextStop>,
    train_query: Query<&Train>,
    mut map_loco_query: Query<&mut Node, (With<MapLocomotive>, Without<MapPin>)>,
    mut map_pin_query: Query<(&mut Node, &mut MapPin), Without<MapLocomotive>>,
    anim_timer: Res<AnimTimer>,
    window_query: Query<&Window>,
) {
    let window = window_query.single().unwrap();
    let bar_width = window.resolution.width() * 0.6;

    if !train_query.is_empty() {
        let width_travel: f32 = (bar_width + MAP_LOCO_OFFSET_LEFT) - MAP_LOCO_OFFSET_RIGHT;
        //println!("{}", width_travel);

        let train_dist: f32 = train_query.single().unwrap().distance;
        let last_dist: f32 = last_stop_dist.0;
        let next_dist: f32 = next_stop.distance;
        let train_progress: f32 = (train_dist - last_dist) / (next_dist - last_dist);

        for mut map_loco in &mut map_loco_query {
            map_loco.right = Val::Px(
                MAP_LOCO_OFFSET_RIGHT.lerp((bar_width + MAP_LOCO_OFFSET_LEFT), train_progress)
                    - ((sinerp((anim_timer.right_slide_amount + width_travel) / width_travel) * width_travel) - width_travel),
            );
            // map_loco.right = Val::Percent(
            //     full_right.lerp(full_left, train_progress)
            //         - ((sinerp((anim_timer.right_slide_amount + 98.) / 98.) * 98.) - 98.),
            // );
        }
    }

    if anim_timer.time > 2.0 {
        for mut map_pin in &mut map_pin_query {
            if map_pin.1.right {
                map_pin.0.width = Val::Px(512.0 * 0.08);
                map_pin.0.height = Val::Px(512.0 * 0.08);
                map_pin.0.bottom = Val::Percent(47.0);
                map_pin.0.right = Val::Px(MAP_PIN_OFFSET_RIGHT);
            }
            else {
                map_pin.0.width = Val::Px(512.0 * 0.08);
                map_pin.0.height = Val::Px(512.0 * 0.08);
                map_pin.0.bottom = Val::Percent(47.0);
                map_pin.0.right = Val::Px(bar_width + MAP_PIN_OFFSET_LEFT);
            }
        }
    }

    if anim_timer.time > 2.0 {
        for mut map_pin in &mut map_pin_query {
            if map_pin.1.right {
                map_pin.0.width = Val::Px(512.0 * 0.08);
                map_pin.0.height = Val::Px(512.0 * 0.08);
                map_pin.0.bottom = Val::Percent(47.0);
                map_pin.0.right = Val::Px(MAP_PIN_OFFSET_RIGHT);
            }
            else {
                map_pin.0.width = Val::Px(512.0 * 0.08);
                map_pin.0.height = Val::Px(512.0 * 0.08);
                map_pin.0.bottom = Val::Percent(47.0);
                map_pin.0.right = Val::Px(bar_width + MAP_PIN_OFFSET_LEFT);
            }
        }
    }
}

fn start_anim_timer(mut anim_timer: ResMut<AnimTimer>, current_stop: Res<CurrentStop>, window_query: Query<&Window>) {
    if matches!(current_stop.0, Some(NumberedStop(Stop::Initial, _)) | None) {
        return;
    }
    println!("start animating progress bar");
    let window = window_query.single().unwrap();
    let bar_width = window.resolution.width() * 0.6;
    let width_travel: f32 = (bar_width + MAP_LOCO_OFFSET_LEFT) - MAP_LOCO_OFFSET_RIGHT;

    anim_timer.time = 0.;
    anim_timer.right_slide_amount = -width_travel;
    anim_timer.swapped = false;
}

fn animate_progress_bar(
    mut anim_timer: ResMut<AnimTimer>,
    time: Res<Time>,
    mut map_pin_query: Query<(&mut Node, &mut MapPin), Without<MapLocomotive>>,
    window_query: Query<&Window>,
) {
    if anim_timer.time > 2.0 {
        return;
    }

    anim_timer.time += time.delta_secs() * 1.25;

    let window = window_query.single().unwrap();
    let bar_width = window.resolution.width() * 0.6;

    if anim_timer.time > 0. && anim_timer.time <= 0.5 {
        let full_width: f32 = 512.0 * 0.08;
        for mut map_pin in &mut map_pin_query {
            if map_pin.1.right {
                map_pin.0.width = Val::Px(full_width * sinerp((0.5 - anim_timer.time) * 2.));
                map_pin.0.height = Val::Px(full_width * sinerp((0.5 - anim_timer.time) * 2.));
                map_pin.0.bottom = Val::Percent(47.0.lerp(70., sinerp(anim_timer.time * 2.)));
                map_pin.0.right = Val::Px(MAP_PIN_OFFSET_RIGHT.lerp(MAP_PIN_OFFSET_RIGHT*(-1.1/1.6), sinerp(anim_timer.time * 2.)));
            }
        }
    }

    if anim_timer.time > 0.5 && anim_timer.time <= 1.5 {
        let width_travel: f32 = (bar_width + MAP_LOCO_OFFSET_LEFT) - MAP_LOCO_OFFSET_RIGHT;
        anim_timer.right_slide_amount = (-width_travel).lerp(0., (anim_timer.time - 0.5) / 1.);

        for mut map_pin in &mut map_pin_query {
            if !map_pin.1.right {
                map_pin.0.right =
                    Val::Px((bar_width + MAP_PIN_OFFSET_LEFT).lerp(MAP_PIN_OFFSET_RIGHT, sinerp((anim_timer.time - 0.5) / 1.)));
            }
        }
    }
    if anim_timer.time <= 1.5 {
        anim_timer.swapped = false;
    }
    if anim_timer.time > 1.5 && !anim_timer.swapped {
        for mut map_pin in &mut map_pin_query {
            map_pin.1.right = !map_pin.1.right;
        }
        anim_timer.swapped = true;
    }
    if anim_timer.time > 1.5 && anim_timer.time <= 2.0 {
        let full_width: f32 = 512.0 * 0.08;
        for mut map_pin in &mut map_pin_query {
            if !map_pin.1.right {
                map_pin.0.width = Val::Px(full_width * sinerp((anim_timer.time - 1.5) * 2.));
                map_pin.0.height = Val::Px(full_width * sinerp((anim_timer.time - 1.5) * 2.));
                map_pin.0.bottom =
                    Val::Percent(70.0.lerp(47., sinerp((anim_timer.time - 1.5) * 2.)));
                map_pin.0.right =
                    Val::Px(((bar_width + MAP_PIN_OFFSET_LEFT)*99.1/96.4).lerp(bar_width + MAP_PIN_OFFSET_LEFT, sinerp((anim_timer.time - 1.5) * 2.)));
            }
        }
    }
}

fn sinerp(x: f32) -> f32 {
    -(ops::cos(PI * x) - 1.) / 2.
}

fn update_last_stop_dist(mut last_stop_dist: ResMut<LastStopDist>, train_query: Query<&Train>) {
    if !train_query.is_empty() {
        last_stop_dist.0 = train_query.single().unwrap().distance;
    }
}
