use bevy::prelude::*;
use rand::{
    Rng, SeedableRng,
    seq::{IndexedMutRandom, IndexedRandom},
};

use crate::{
    GameState, ImageAssets, InGameState, MainGameObject,
    resources_plugin::{Inventory, Item},
    train_plugin::{Train, TrainLength, TrainState},
    ui_state::InMenu,
    world_plugin::goblin_spawner::{GoblinSpawner, GoblinType, spawn_goblins},
};

mod goblin_spawner;
mod progress_bar_plugin;
pub mod stop_plugin;

#[derive(Component)]
#[require(Pickable::default())]
pub struct WorldClickable;

#[derive(Clone)]
pub enum Stop {
    Town,
    Mine { minecarts: Vec<Minecart> },
    GoblinAttack { waves: Vec<Vec<GoblinType>> },
    Initial,
}

#[derive(Component, Clone)]
pub struct Minecart {
    resource_type: Item,
    resource_amount: usize,
    clicked: bool,
    offset: Vec3,
}

#[derive(Clone)]
pub struct NumberedStop(pub Stop, pub usize);

impl Stop {
    fn spawn_stop(&self, mut commands: Commands, distance: f32, image_assets: Res<ImageAssets>) {
        match self {
            Stop::Town => {
                commands
                    .spawn((
                        NextStopImage,
                        Transform::from_xyz(-distance * METERS_PER_UNIT, 0., -10.),
                        WorldObject(distance),
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                Sprite::from_image(image_assets.stop_bg.clone()),
                                Transform::from_xyz(0., 0., -25.0),
                                WorldClickable
                            ))
                            .observe(
                                |_trigger: Trigger<Pointer<Pressed>>,
                                train_state: Res<State<TrainState>>,
                                 menu_state: Res<State<InMenu>>, mut next_state: ResMut<NextState<InMenu>>| {
                                    if *menu_state == InMenu::None && *train_state == TrainState::Stopped {
                                        next_state.set(InMenu::StopMenu);
                                    }
                                },
                            );
                        parent.spawn((
                            Sprite::from_image(image_assets.stop_fg.clone()),
                            Transform::from_xyz(0., 0., 25.0),
                        ));
                    });
            }
            Stop::Mine { minecarts } => {
                commands.spawn((
                    NextStopImage,
                    Transform::from_xyz(-distance * METERS_PER_UNIT, 0., -10.),
                    WorldObject(distance),
                ))
                .with_children(|parent| {
                    for minecart in minecarts.iter() {
                        let minecart_image: Handle<Image> =
                        match minecart.resource_type {
                            Item::Metal => { image_assets.minecart_metal.clone() }
                            Item::Wood => { image_assets.minecart_wood.clone() }
                            Item::Stone => { image_assets.minecart_stone.clone() }
                            _ => { image_assets.minecart_empty.clone() }
                        };

                        parent
                            .spawn((
                                Sprite::from_image(minecart_image.clone()),
                                Transform::from_xyz(minecart.offset.x, minecart.offset.y, minecart.offset.z),
                                WorldClickable,
                                minecart.clone(),
                            ))
                            .observe(
                                |trigger: Trigger<Pointer<Pressed>>,
                                 mut minecart_query: Query<(Entity, &mut Minecart, &mut Sprite)>,
                                 image_assets: Res<ImageAssets>,
                                 mut inventories: Query<&mut Inventory>, | {
                                    for (entity, mut minecart, mut sprite) in &mut minecart_query {
                                        if entity == trigger.target && !minecart.clicked {
                                            minecart.clicked = true;
                                            for mut inventory in &mut inventories {
                                                *inventory
                                                    .items
                                                    .entry(minecart.resource_type.clone())
                                                    .or_insert(0) += minecart.resource_amount;
                                                break;
                                            }
                                            sprite.image = image_assets.minecart_empty.clone();
                                        }
                                    }
                                },
                            );
                    }
                    parent.spawn((
                        Sprite::from_image(image_assets.mine_stop.clone()),
                        Transform::from_xyz(0., 130., -25.0),
                    ));
                    });
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
            Stop::Mine { .. } => generate_mine_name(rng),
            Stop::GoblinAttack { .. } => "Goblin Ambush".into(),
        }
    }

    fn generate_random<R: Rng>(rng: &mut R, current_stop: &CurrentStop) -> Self {
        let mut stops: [(&mut dyn FnMut(&mut R) -> Stop, u32); 3] = [
            (&mut |_| Stop::Town, 3),
            (
                &mut |rng| Stop::Mine {
                    minecarts: generate_minecarts(rng),
                },
                2,
            ),
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

const FIRST_HALVES: &[&str] = &[
    "Snod",
    "Bell",
    "South",
    "Hamburger",
    "East West",
    "Hamburger Schlamburger",
    "King",
    "Lang",
    "Pen",
    "Lynn",
];
const SECOND_HALVES: &[&str] = &[
    " Upon Trent",
    "sbury",
    "ceston",
    "chester",
    " Schlamburger",
    " Hamburger Schlamburger",
    "phalia",
    "ington",
    " Springs",
    " Hill",
    "worth",
    "ston",
    "port",
    "shire",
    "ford",
    " Ham",
    "bane",
    "sylvania",
];

const THIRD_HALVES: &[&str] = &[
    " Mines",
    " Quarry",
    " Cave",
    " Mineshaft",
    " Pit",
    " Dig",
    " Burrow",
    " Cavern",
    " Chamber",
    " Deposit",
    " Lode",
];

fn generate_town_name(rng: &mut impl Rng) -> String {
    let mut out = String::new();
    out.push_str(FIRST_HALVES.choose(rng).unwrap());
    out.push_str(SECOND_HALVES.choose(rng).unwrap());
    out
}

fn generate_mine_name(rng: &mut impl Rng) -> String {
    let mut out = String::new();
    out.push_str(FIRST_HALVES.choose(rng).unwrap());
    out.push_str(SECOND_HALVES.choose(rng).unwrap());
    out.push_str(THIRD_HALVES.choose(rng).unwrap());
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

const MINECART_ITEMS: &[(Item, usize)] = &[
    (Item::Wood, 1),
    (Item::Clay, 2),
    (Item::Brick, 1),
    (Item::Stone, 3),
    (Item::Metal, 2),
];

fn generate_minecarts(rng: &mut impl Rng) -> Vec<Minecart> {
    let mut minecarts: Vec<Minecart> = Vec::new();
    let minecart_count = rng.random_range(2..=5);

    let mut minecart_offsets: Vec<Vec3> = vec![
        vec3(0., 135., -20.),
        vec3(-65., 185., -21.),
        vec3(155., 195., -21.),
        vec3(-120., 150., -20.),
        vec3(-155., 195., -21.),
        vec3(65., 185., -21.),
        vec3(120., 150., -20.),
    ];

    for _i in 0..minecart_count {
        let minecart_item = MINECART_ITEMS
            .choose_weighted(rng, |(_, w)| *w)
            .unwrap()
            .0
            .clone();

        let chosen_pos = rng.random_range(0..minecart_offsets.len());
        minecarts.push(Minecart {
            resource_type: minecart_item.clone(),
            resource_amount: rng.random_range(45..=75),
            clicked: false,
            offset: minecart_offsets[chosen_pos],
        });
        minecart_offsets.swap_remove(chosen_pos);
    }
    minecarts
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
    app.add_plugins((
        stop_plugin::stop_plugin,
        progress_bar_plugin::progress_bar_plugin,
    ))
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
    train_length: Res<TrainLength>,
    mut commands: Commands,
) {
    for mut obj in &mut objs {
        let newx = (train.single().unwrap().distance - obj.1.0) * METERS_PER_UNIT;

        // info!("{newx}");

        // 2000 is to not despawn it immediatily after it gets past the train,
        // and also give some le way so it isn't too easy to see the despawned area
        obj.0.translation.x = newx;

        if newx > (train_length.train_size() + 4000.0) {
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
            MainGameObject,
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
