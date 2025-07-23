use bevy::{math::FloatPow, prelude::*};

use crate::{
    GameState, ImageAssets, MainGameObject,
    build_plugin::{BuildLocation, Building},
    world_plugin::{CurrentStop, GenerateNextStop, NextStop, Stop},
};

mod fuel_display;
mod train_speed_ui;

#[derive(Resource, Default)]
pub struct MaxPixelHeightOfTrain {
    pub height: f32,
}

#[derive(Resource)]
pub struct TrainLength(pub usize);

#[derive(Resource)]
pub struct TrainFuel(pub f32);

#[derive(Resource)]
pub struct TrainStats {
    // pub length: usize,
    pub acceleration: f32,
    pub max_velocity: f32,
}

impl TrainLength {
    pub fn train_size(&self) -> f32 {
        (self.0 as f32) * CAR_SIZE
    }
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum TrainState {
    #[default]
    Stopped,
    Advancing,
    Arriving,
}

#[derive(Event)]
pub struct RunOutOfFuelEvent;

fn reset_resources(
    mut train_stats: ResMut<TrainStats>,
    mut train_length: ResMut<TrainLength>,
    mut train_fuel: ResMut<TrainFuel>,
    mut max_pixel_height_of_train: ResMut<MaxPixelHeightOfTrain>,
    mut train_state: ResMut<NextState<TrainState>>,
) {
    *train_stats = TrainStats {
        acceleration: 1.0,
        max_velocity: 27.0,
    };
    *train_length = TrainLength(1);
    *train_fuel = TrainFuel(1000.0);
    *max_pixel_height_of_train = MaxPixelHeightOfTrain::default();
    train_state.set(TrainState::default());
}

pub fn train_plugin(app: &mut App) {
    app.add_plugins(fuel_display::fuel_display_plugin)
        .insert_resource(TrainStats {
            acceleration: 1.0,
            max_velocity: 27.0,
        })
        .insert_resource(TrainLength(1))
        .insert_resource(TrainFuel(1000.0))
        .add_event::<AdvanceEvent>()
        .add_event::<StopEvent>()
        .add_event::<RunOutOfFuelEvent>()
        .init_state::<TrainState>()
        .add_systems(OnEnter(GameState::InGame), spawn_train)
        .add_systems(OnEnter(GameState::MainMenu), reset_resources)
        .add_systems(Update, update_train.run_if(resource_changed::<TrainLength>))
        .add_systems(
            Update,
            handle_run_out_of_fuel.run_if(resource_changed::<TrainFuel>),
        )
        .init_resource::<MaxPixelHeightOfTrain>()
        .add_systems(OnEnter(GameState::InGame), train_speed_ui::make_ui)
        .add_systems(
            FixedPostUpdate,
            train_speed_ui::update_train_speed.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            FixedUpdate,
            (
                start_advancing
                    .run_if(in_state(TrainState::Stopped).and(in_state(GameState::InGame))),
                move_train.run_if(
                    (in_state(TrainState::Advancing).or(in_state(TrainState::Arriving)))
                        .and(in_state(GameState::InGame)),
                ),
            ),
        )
        .add_systems(
            FixedUpdate,
            update_train_height.run_if(in_state(GameState::InGame)),
        );
}

pub const CAR_SIZE: f32 = 144.0;

#[derive(Component)]
pub struct Locomotive;
#[derive(Component)]
pub struct Caboose;
#[derive(Component)]
pub struct TrainCar;
#[derive(Component)]
pub struct Train {
    pub distance: f32,
    pub velocity: f32,
}

#[derive(Event)]
pub struct AdvanceEvent;

#[derive(Event)]
pub struct StopEvent {
    stop: Stop,
    name: String,
}

fn spawn_train(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    train_length: Res<TrainLength>,
) {
    commands
        .spawn((
            Visibility::default(),
            Transform::default(),
            Train {
                distance: 0.,
                velocity: 0.,
            },
            MainGameObject,
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_image(image_assets.train_locomotive.clone()),
                Name::new("Locomotive"),
                Locomotive,
            ));
            for i in 0..train_length.0 {
                parent.spawn((
                    Sprite::from_image(image_assets.train_car.clone()),
                    Name::new(format!("Car{i}")),
                    TrainCar,
                    Transform::from_xyz(CAR_SIZE * (i as f32 + 1.), 0., 0.),
                    children![
                        (BuildLocation(Vec2::new(-30.0, 0.0)), Transform::default()),
                        (BuildLocation(Vec2::new(30.0, 0.0)), Transform::default())
                    ],
                ));
            }
            parent.spawn((
                Sprite::from_image(image_assets.train_caboose.clone()),
                Name::new("Caboose"),
                Caboose,
                Transform::from_xyz(CAR_SIZE * (train_length.0 as f32 + 1.), 0., 0.),
            ));
        });
}

fn update_train(
    train_length: Res<TrainLength>,
    mut commands: Commands,
    mut caboose: Single<&mut Transform, With<Caboose>>,
    train: Single<Entity, With<Train>>,
    image_assets: Res<ImageAssets>,
) {
    println!("train updated");
    caboose.translation.x = CAR_SIZE * (train_length.0 as f32 + 1.);
    let i = train_length.0 - 1;
    commands.entity(*train).with_child((
        Sprite::from_image(image_assets.train_car.clone()),
        Name::new(format!("Car{i}")),
        TrainCar,
        Transform::from_xyz(CAR_SIZE * (i as f32 + 1.), 0., 0.),
        children![
            (BuildLocation(Vec2::new(-30.0, 0.0)), Transform::default()),
            (BuildLocation(Vec2::new(30.0, 0.0)), Transform::default())
        ],
    ));
}

fn start_advancing(
    mut ev: EventReader<AdvanceEvent>,
    mut next_state: ResMut<NextState<TrainState>>,
    mut current_stop: ResMut<CurrentStop>,
) {
    for _ in ev.read() {
        info!("Starting to advance!");
        next_state.set(TrainState::Advancing);

        current_stop.0 = None;
    }
}

fn move_train(
    mut train: Query<&mut Train>,
    train_stats: Res<TrainStats>,
    mut current_stop: ResMut<CurrentStop>,
    next_stop: Res<NextStop>,
    mut next_state: ResMut<NextState<TrainState>>,
    mut commands: Commands,
    time: Res<Time>,
    mut ev: EventWriter<StopEvent>,
    mut fuel: ResMut<TrainFuel>,
    mut fuel_ev: EventWriter<RunOutOfFuelEvent>,
) {
    let mut train = train.single_mut().unwrap();

    train.velocity = if next_stop.distance - train.distance < (train.velocity * 3.1) {
        (((next_stop.distance - train.distance) * 0.8) + 0.1).min(train.velocity)
    } else {
        train.velocity + train_stats.acceleration * time.delta_secs()
    };

    train.velocity = train.velocity.min(train_stats.max_velocity);

    train.distance += train.velocity * time.delta_secs();
    fuel.0 -= train.velocity * time.delta_secs();
    if fuel.0 <= 0.0 {
        fuel_ev.write(RunOutOfFuelEvent);
    }
    // info!("Distance: {}", train.distance);

    if next_stop.distance - train.distance < 17.0 {
        next_state.set(TrainState::Arriving);
    }

    if next_stop.distance - train.distance < 0.1 {
        info!(
            "{} - {} = {}",
            next_stop.distance,
            train.distance,
            next_stop.distance - train.distance
        );
        info!("Stopping");
        next_state.set(TrainState::Stopped);
        train.velocity = 0.0;

        train.velocity = 0.0;

        ev.write(StopEvent {
            stop: next_stop.stop.0.clone(),
            name: next_stop.name.clone(),
        });

        current_stop.0 = Some(next_stop.stop.clone());

        commands.trigger(GenerateNextStop);
    }
}

fn update_train_height(
    mut height: ResMut<MaxPixelHeightOfTrain>,
    changed_comp: Query<&GlobalTransform, (Changed<GlobalTransform>, With<Building>)>,
    removed_comp: RemovedComponents<Building>,
    buildings: Query<&GlobalTransform, With<Building>>,
) {
    for changed in changed_comp.iter() {
        let t = changed.translation();

        if t.y > height.height {
            height.height = t.y;
        }
    }

    if !removed_comp.is_empty() {
        let mut nh = 0.0f32;

        for i in buildings {
            nh = nh.max(i.translation().y)
        }

        // not updating if not changed because performance stuff
        // (in case someone does change detection on the height)
        if nh != height.height {
            height.height = nh;
        }
    }
}

fn handle_run_out_of_fuel(
    mut ev: EventReader<RunOutOfFuelEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    main_game_objects: Query<Entity, (Without<Camera>, With<MainGameObject>)>,
) {
    for ev in ev.read() {
        'a: for main_game_object in &main_game_objects {
            commands
                .entity(main_game_object)
                .despawn_related::<Children>()
                .despawn();
        }
        next_state.set(GameState::MainMenu);
    }
}
