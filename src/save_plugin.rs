use std::{fs, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    GameState,
    build_plugin::{
        BuildLocation, Building, BuildingType,
        bird_plane::{Bird, BirdReturnData, BirdTimer, Roost},
    },
    resources_plugin::Inventory,
    train_plugin::{Train, TrainCar, TrainFuel, TrainState, TrainStats},
};

pub fn save_plugin(app: &mut App) {
    app.add_systems(
        Update,
        save_game_data
            .run_if(on_timer(Duration::from_secs_f32(1.0)).and(in_state(GameState::InGame))),
    );
}

#[derive(serde::Deserialize, serde::Serialize, Resource, Debug)]
pub struct GameSave {
    pub train_stats: TrainStats,
    pub fuel: f32,
    pub saved_train_cars: Vec<SavedTrainCar>,
    pub train: Train,
    pub train_state: TrainState,
}

impl Default for GameSave {
    fn default() -> Self {
        Self {
            train_stats: TrainStats {
                acceleration: 1.0,
                max_velocity: 27.0,
            },
            fuel: 1000.0,
            saved_train_cars: vec![SavedTrainCar::default()],
            train: Train {
                distance: 0.0,
                velocity: 0.0,
            },
            train_state: TrainState::Stopped,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct SavedTrainCar {
    pub children: Vec<SavedBuilding>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SavedRoost {
    pub birds: Vec<SavedBird>,
    pub id: u32,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SavedBuilding {
    pub building_type: BuildingType,
    pub above: Option<Box<SavedBuilding>>,
    pub inventory: Option<Inventory>,
    pub roost: Option<SavedRoost>,
}
// pub enum SavedBuilding {
//     Housing {
//         above: Option<Box<SavedBuilding>>,
//     },
//     Farm,
//     Storage {
//         above: Option<Box<SavedBuilding>>,
//         inventory: Inventory,
//     },
//     Sawmill {
//         above: Option<Box<SavedBuilding>>,
//     },
//     AlchemyLab {
//         above: Option<Box<SavedBuilding>>,
//     },
//     Cannon {
//         above: Option<Box<SavedBuilding>>,
//     },
//     Workshop,
//     Roost {
//         birds: Vec<SavedBird>,
//         id: u32,
//     },
//     LiquidTank {
//         above: Option<Box<SavedBuilding>>,
//     },
//     Factory,
// }

// impl SavedBuilding {
//     pub fn get_building_type(&self) -> BuildingType {
//         match self {
//             SavedBuilding::Housing { .. } => BuildingType::Housing,
//             SavedBuilding::Farm => BuildingType::Farm,
//             SavedBuilding::Storage { .. } => BuildingType::Storage,
//             SavedBuilding::Sawmill { .. } => BuildingType::Sawmill,
//             SavedBuilding::AlchemyLab { .. } => BuildingType::AlchemyLab,
//             SavedBuilding::Cannon { .. } => BuildingType::Cannon,
//             SavedBuilding::Workshop => BuildingType::Workshop,
//             SavedBuilding::Roost { .. } => BuildingType::Roost,
//             SavedBuilding::LiquidTank { .. } => BuildingType::LiquidTank,
//             SavedBuilding::Factory => BuildingType::Factory,
//         }
//     }
// }

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SavedBird {
    pub out: bool,
    pub remaining_secs: f32,
    pub return_location: Vec2,
    pub roost: u32,
}

impl From<Roost> for SavedRoost {
    fn from(value: Roost) -> Self {
        todo!()
    }
}

fn save_game_data(
    train_stats: Res<TrainStats>,
    train_fuel: Res<TrainFuel>,
    train: Single<&Train>,
    train_cars: Query<&Children, With<TrainCar>>,
    buildings: Query<(
        Has<BuildLocation>,
        Option<&Children>,
        Option<&Building>,
        Option<&Inventory>,
        Option<&Roost>,
        Option<&Transform>,
    )>,
    birds: Query<(&BirdTimer, &BirdReturnData), With<Bird>>,
    mut commands: Commands,
    train_state: Res<State<TrainState>>,
) {
    let mut saved_train_cars: Vec<SavedTrainCar> = Vec::new();
    for train_car in &train_cars {
        let mut buildings = train_car
            .iter()
            .map(|building_id| {
                fn recurse(
                    building_id: Entity,
                    buildings: Query<(
                        Has<BuildLocation>,
                        Option<&Children>,
                        Option<&Building>,
                        Option<&Inventory>,
                        Option<&Roost>,
                        Option<&Transform>,
                    )>,
                    birds: Query<(&BirdTimer, &BirdReturnData), With<Bird>>,
                    commands: &mut Commands,
                ) -> Option<SavedBuilding> {
                    let (is_build_location, children, building, inventory, roost, transform) =
                        buildings.get(building_id).unwrap();
                    if is_build_location {
                        return None;
                    }
                    let mut above = |index: usize| {
                        recurse(
                            *children.unwrap().get(index).unwrap(),
                            buildings,
                            birds,
                            commands,
                        )
                        .map(|it| Box::new(it))
                    };
                    let building_type = building.unwrap().0.clone();
                    Some(SavedBuilding {
                        building_type,
                        above: if children.is_some_and(|it| !it.is_empty()) {
                            above(0)
                        } else {
                            None
                        },
                        inventory: inventory.cloned(),
                        roost: roost.cloned().map(|roost| SavedRoost {
                            birds: roost
                                .birds
                                .iter()
                                .map(|it| SavedBird {
                                    out: false,
                                    remaining_secs: 0.0,
                                    return_location: transform.unwrap().translation.xy(),
                                    roost: building_id.index(),
                                })
                                .collect::<Vec<_>>(),
                            id: building_id.index(),
                        }),
                    })
                }
                recurse(building_id, buildings, birds, &mut commands)
            })
            .collect::<Vec<_>>();
        saved_train_cars.push(SavedTrainCar {
            children: buildings.drain(..).filter_map(|it| it).collect::<Vec<_>>(),
        });
    }

    let save = GameSave {
        train_stats: train_stats.clone(),
        fuel: train_fuel.0,
        saved_train_cars,
        train_state: train_state.clone(),
        train: train.clone(),
    };

    info!("saved");
    fs::write("./save.tmrw", serde_json::to_string_pretty(&save).unwrap()).unwrap();
}
