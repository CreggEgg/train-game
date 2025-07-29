use std::{collections::HashMap, time::Duration};

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{ConfigurationAssets, GameState, InGameState, resources_plugin::Fluid};

use super::{Building, BuildingType, LiquidTank};

pub fn synergies_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        update_synergies.run_if(
            on_timer(Duration::from_secs(4))
                .and(in_state(GameState::InGame))
                .and(in_state(InGameState::Running)),
        ),
    )
    .add_systems(
        FixedUpdate,
        populate_synergy_map.run_if(resource_exists::<ConfigurationAssets>),
    )
    .init_resource::<SynergyMap>();
}

#[derive(serde::Deserialize)]
struct Synergy {
    above: Vec<SynergyPredicate>,
    building: BuildingType,
    below: Vec<SynergyPredicate>,
}

#[derive(serde::Deserialize, Clone, Debug)]
enum SynergyPredicate {
    IsType(BuildingType),
    ContainsFluid(Fluid),
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Synergies(Vec<Synergy>);

#[derive(Clone, Debug)]
struct SynergyPairing {
    above: Vec<SynergyPredicate>,
    below: Vec<SynergyPredicate>,
}

#[derive(Resource, Default)]
struct SynergyMap(HashMap<BuildingType, Vec<SynergyPairing>>);

#[derive(Component)]
pub struct Synergized;

fn update_synergies(
    mut commands: Commands,
    synergy_map: Res<SynergyMap>,
    buildings: Query<(
        Entity,
        &ChildOf,
        &Children,
        &Building,
        Option<&Synergized>,
        Option<&LiquidTank>,
    )>,
) {
    println!("52");
    for (entity, child_of, children, building, currently_synergized, _) in &buildings {
        println!("54");
        let valid_synergies = synergy_map.0.get(&building.0).cloned().unwrap_or_default();
        let synergized = {
            let actual_below = buildings.get(child_of.0);
            let actual_above = children.get(0).and_then(|child| buildings.get(*child).ok());
            dbg!(&valid_synergies);
            valid_synergies.iter().any(|valid_synergy| {
                let is_below_correct = actual_below
                    .map(|(_, _, _, Building(it), _, liquid_tank)| {
                        valid_synergy.below.iter().all(|predicate| match predicate {
                            SynergyPredicate::IsType(building_type) => building_type == it,
                            SynergyPredicate::ContainsFluid(fluid) => {
                                liquid_tank.and_then(|it| it.contained_fluid.clone())
                                    == Some(fluid.clone())
                            }
                        })
                    })
                    .unwrap_or_default();

                let is_above_correct = actual_above
                    .map(|(_, _, _, Building(it), _, liquid_tank)| {
                        valid_synergy.above.iter().all(|predicate| match predicate {
                            SynergyPredicate::IsType(building_type) => building_type == it,
                            SynergyPredicate::ContainsFluid(fluid) => {
                                liquid_tank.and_then(|it| it.contained_fluid.clone())
                                    == Some(fluid.clone())
                            }
                        })
                    })
                    .unwrap_or_default();

                /* if let Some(synergy_below) = valid_synergy.below {
                    actual_below
                        .map(|(_, _, _, Building(it), _)| *it == synergy_below)
                        .unwrap_or_default()
                } else {
                    true
                }; */
                println!("above: {}, below: {}", is_above_correct, is_below_correct);
                is_above_correct && is_below_correct
            })
        };
        // let synergized = true;

        if synergized && currently_synergized.is_none() {
            println!("adding synergized");
            commands.entity(entity).insert(Synergized);
        } else if !synergized && currently_synergized.is_some() {
            commands.entity(entity).remove::<Synergized>();
        }
    }
}

fn populate_synergy_map(
    mut ev: EventReader<AssetEvent<Synergies>>,
    mut synergy_map: ResMut<SynergyMap>,
    configuration_assets: Res<ConfigurationAssets>,
    synergy_assets: Res<Assets<Synergies>>,
) {
    for _ in ev.read() {
        info!("populating synergy map");
        synergy_map.0 = HashMap::new();
        let synergies = synergy_assets.get(&configuration_assets.synergies).unwrap();
        for synergy in &synergies.0 {
            synergy_map
                .0
                .entry(synergy.building)
                .or_insert(vec![])
                .push(SynergyPairing {
                    above: synergy.above.clone(),
                    below: synergy.below.clone(),
                });
        }
    }
}
