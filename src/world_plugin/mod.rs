use bevy::prelude::*;
use rand::{Rng, SeedableRng};

use crate::{GameState, ImageAssets, InGameState, train_plugin::Train};

mod stop_plugin;

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
    pub spawned: bool,
}
#[derive(Resource)]
pub struct CurrentStop(pub Option<Stop>);

#[derive(Event)]
pub struct GenerateNextStop;

pub fn world_plugin(app: &mut App) {
    app /* .add_plugins(stop_plugin::stop_plugin) */
        .add_systems(OnEnter(GameState::Loading), generate_world)
        .add_systems(
            FixedUpdate,
            ((move_world_objects, spawn_stop_assets, loop_rails)
                .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running)))),
        )
        .add_systems(OnEnter(GameState::InGame), spawn_rails)
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
        spawned: false,
    }
}

#[derive(Component)]
struct WorldObject(f32);

#[derive(Component)]
struct NextStopImage;

const METERS_PER_UNIT: f32 = 100.0;

fn spawn_stop_assets(
    mut commands: Commands,
    train: Query<&Train>,
    mut next_stop: ResMut<NextStop>,
    image_assets: Res<ImageAssets>,
) {
    let train = train.single().unwrap();
    let horizontal_distance = 100.0;
    if !next_stop.spawned
        && next_stop.distance - train.distance < horizontal_distance * METERS_PER_UNIT
    {
        next_stop.spawned = true;
        commands.spawn((
            NextStopImage,
            Transform::from_xyz(-next_stop.distance * METERS_PER_UNIT, 0., -10.),
            WorldObject(next_stop.distance),
            children![
                (
                    Sprite::from_image(image_assets.stop_bg.clone()),
                    Transform::from_xyz(0., 0., -25.0)
                ),
                (
                    Sprite::from_image(image_assets.stop_fg.clone()),
                    Transform::from_xyz(0., 0., 25.0)
                )
            ],
        ));
    }
}

fn move_world_objects(
    mut objs: Query<(&mut Transform, &WorldObject)>,
    train: Query<&Train>,
    time: Res<Time>,
) {
    for mut obj in &mut objs {
        obj.0.translation.x = -(obj.1.0 - train.single().unwrap().distance) * METERS_PER_UNIT;
        // info!(
        //     "Error: {:.2} | Actual distance: {:.2}",
        //     obj.0.translation.x.abs()
        //         - ((obj.1.0 - train.single().unwrap().distance) / METERS_PER_UNIT),
        //     ((obj.1.0 - train.single().unwrap().distance) / METERS_PER_UNIT)
        // );
    }
}

#[derive(Component)]
struct Rail;

const RAIL_WIDTH: f32 = 480.0 / METERS_PER_UNIT;
const NUM_RAILS: usize = 8;

fn spawn_rails(mut commands: Commands, image_assets: Res<ImageAssets>) {
    for i in 0..NUM_RAILS {
        commands.spawn((
            Sprite::from_image(image_assets.rail.clone()),
            Transform::default(),
            WorldObject((i as f32 - 4.) * RAIL_WIDTH),
            Rail,
        ));
    }
}

fn loop_rails(mut rails: Query<(&mut WorldObject, &Rail)>, train: Query<&Train>) {
    for (mut world_object, rail) in &mut rails {
        if world_object.0 - train.single().unwrap().distance < -25.0 {
            world_object.0 += RAIL_WIDTH * NUM_RAILS as f32;
        }
    }
}
