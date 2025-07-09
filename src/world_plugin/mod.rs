use bevy::prelude::*;
use rand::{
    Rng, SeedableRng,
    seq::{IndexedMutRandom, IndexedRandom},
};

use crate::{
    GameState, ImageAssets, InGameState,
    train_plugin::{Train, TrainState, TrainStats},
    world_plugin::goblin_spawner::{GoblinSpawner, GoblinType, spawn_goblins},
};

mod goblin_spawner;
pub mod stop_plugin;

#[derive(Clone)]
pub enum Stop {
    Town,
    GoblinAttack { waves: Vec<Vec<GoblinType>> },
    Initial,
}

#[derive(Clone)]
pub struct NumberedStop(pub Stop, pub usize);

impl Stop {
    fn spawn_stop(&self, mut commands: Commands, distance: f32, image_assets: Res<ImageAssets>) {
        match self {
            Stop::Town => {
                commands.spawn((
                    NextStopImage,
                    Transform::from_xyz(-distance * METERS_PER_UNIT, 0., -10.),
                    WorldObject(distance),
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
            Stop::Initial => {}
            Stop::GoblinAttack { waves } => {
                commands.spawn((
                    NextStopImage,
                    Transform::from_xyz(-distance * METERS_PER_UNIT, 0., -10.),
                    WorldObject(distance),
                    children![
                        (
                            Sprite::from_image(image_assets.goblin_stop_bg.clone()),
                            Transform::from_xyz(0., 0., -25.0)
                        ),
                        (
                            Sprite::from_image(image_assets.goblin_stop_fg.clone()),
                            Transform::from_xyz(0., 0., 25.0)
                        ),
                        (GoblinSpawner::new(waves.clone()), Transform::default()),
                    ],
                ));
            }
        }
    }
    fn generate_name(&self, rng: &mut impl Rng) -> String {
        match self {
            Stop::Town | Self::Initial => generate_town_name(rng),
            Stop::GoblinAttack { waves } => "Goblin Ambush".into(),
        }
    }

    fn generate_random<R: Rng>(rng: &mut R, current_stop: &CurrentStop) -> Self {
        let mut stops: [(&mut dyn FnMut(&mut R) -> Stop, u32); 2] = [
            (&mut |_| Stop::Town, 5),
            (
                &mut |rng| Stop::GoblinAttack {
                    waves: generate_waves(rng),
                },
                1,
            ),
        ];

        if let Some(NumberedStop(Stop::GoblinAttack { waves: _ }, _)) = current_stop.0 {
            Stop::Town
        } else {
            stops.choose_weighted_mut(rng, |(_, w)| *w).unwrap().0(rng)
        }
    }
}

const FIRST_HALVES: &[&'static str] = &[
    "Snod",
    "Bell",
    "South",
    "Hamburger",
    "East West",
    "Hamburger Schlamburger",
];
const SECOND_HALVES: &[&'static str] = &[
    " Upon Trent",
    "sbury",
    "ceston",
    "chester",
    " Schlamburger",
    " Hamburger Schlamburger",
    "phalia",
];

fn generate_town_name(rng: &mut impl Rng) -> String {
    let mut out = String::new();
    out.push_str(FIRST_HALVES.choose(rng).unwrap());
    out.push_str(SECOND_HALVES.choose(rng).unwrap());
    out
}

fn generate_waves(rng: &mut impl Rng) -> Vec<Vec<GoblinType>> {
    let waves = rng.random_range(1..=10);

    (0..waves)
        .map(|_| {
            let num = rng.random_range(1..=10);

            (0..num).map(|_| GoblinType::Basic).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Resource)]
pub struct GameWorld {
    rng: rand_chacha::ChaCha8Rng,
}

#[derive(Resource)]
pub struct NextStop {
    pub stop: NumberedStop,
    pub distance: f32,
    pub spawned: bool,
    pub name: String,
}
#[derive(Resource)]
pub struct CurrentStop(pub Option<NumberedStop>);

#[derive(Event)]
pub struct GenerateNextStop;

pub fn world_plugin(app: &mut App) {
    app.add_plugins(stop_plugin::stop_plugin)
        .add_systems(OnEnter(GameState::Loading), generate_world)
        .add_systems(
            FixedUpdate,
            (move_world_objects, spawn_stop_assets, loop_rails)
                .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running))),
        )
        .add_systems(OnEnter(GameState::InGame), spawn_rails)
        .add_systems(
            FixedUpdate,
            spawn_goblins.run_if(
                in_state(GameState::InGame)
                    .and(in_state(InGameState::Running))
                    .and(in_state(TrainState::Stopped)),
            ),
        )
        .add_observer(
            |_trigger: Trigger<GenerateNextStop>,
             mut next_stop: ResMut<NextStop>,
             current_stop: Res<CurrentStop>,
             mut game_world: ResMut<GameWorld>,
             train: Query<&Train>| {
                *next_stop = generate_next_stop(
                    &mut game_world.rng,
                    train.single().unwrap().distance,
                    &current_stop,
                );
            },
        );
}

fn generate_world(mut commands: Commands) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(460);

    commands.insert_resource(CurrentStop(Some(NumberedStop(Stop::Initial, 0))));
    commands.insert_resource(generate_next_stop(&mut rng, 0., &CurrentStop(None)));

    commands.insert_resource(GameWorld { rng });
}

fn generate_next_stop(
    rng: &mut impl Rng,
    current_distance: f32,
    current_stop: &CurrentStop,
) -> NextStop {
    let distance = rng.random_range(
        60.0..=140.0, /*units now in meters but i made these very small to make it easy to test*/
    ) + current_distance;
    info!("Random f32: {}", distance);
    let stop = Stop::generate_random(rng, current_stop);

    NextStop {
        name: stop.generate_name(rng),
        stop: NumberedStop(
            stop,
            current_stop
                .0
                .clone()
                .map(|it| {
                    if let Stop::GoblinAttack { .. } = it.0 {
                        it.1
                    } else {
                        it.1 + 1
                    } //ensure contracts dont expire on goblin stops
                })
                .unwrap_or(1),
        ),
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
    commands: Commands,
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

        next_stop
            .stop
            .0
            .spawn_stop(commands, next_stop.distance, image_assets);
    }
}

fn move_world_objects(
    mut objs: Query<(&mut Transform, &WorldObject, Entity)>,
    train: Query<&Train>,
    train_stats: Res<TrainStats>,
    mut commands: Commands,
) {
    for mut obj in &mut objs {
        let newx = (train.single().unwrap().distance - obj.1.0) * METERS_PER_UNIT;

        // info!("{newx}");

        // 2000 is to not despawn it immediatily after it gets past the train,
        // and also give some le way so it isn't too easy to see the despawned area
        obj.0.translation.x = newx;

        if newx > (train_stats.train_size() + 4000.0) {
            commands.entity(obj.2).despawn();
            info!("despawning world object");
        }

        // info!(
        //     "Error: {:.2} | Actual distance: {:.2}",
        //     obj.0.translation.x.abs()
        //         - ((obj.1.0 - train.single().unwrap().distance) / METERS_PER_UNIT),
        //     ((obj.1.0 - train.single().unwrap().distance) / METERS_PER_UNIT)
        // );
    }

    // info!("world objects {}", objs.iter().len());
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
            children![(
                Sprite::from_image(image_assets.ground.clone()),
                Transform::from_xyz(0., 0., -100.0)
            ),],
        ));
    }
}

fn loop_rails(mut rails: Query<(&mut WorldObject, &Rail)>, train: Query<&Train>) {
    for (mut world_object, _) in &mut rails {
        if world_object.0 - train.single().unwrap().distance < -25.0 {
            world_object.0 += RAIL_WIDTH * NUM_RAILS as f32;
        }
    }
}
