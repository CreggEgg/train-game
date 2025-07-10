use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{RED, YELLOW},
    ecs::{
        name,
        relationship::{RelatedSpawnerCommands, Relationship},
    },
    platform::collections::HashMap,
    prelude::*,
    reflect::Array,
    state::commands,
};
use rand::{Rng, seq::IndexedRandom};

use crate::{
    FontAssets, GameState, ImageAssets, InGameState,
    control_panel_plugin::AdvanceBlocker,
    resources_plugin::{Inventory, Item},
    train_plugin::TrainState,
    ui_state::InMenu,
    world_plugin::{self, NextStop},
};

use super::{CurrentStop, GameWorld, NumberedStop, Stop};
pub fn stop_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_stop_menu)
        .insert_resource(ActiveContracts(Vec::new()))
        .insert_resource(FadeTime { time: 0. })
        .add_systems(
            OnEnter(InMenu::StopMenu),
            |mut menu: Single<&mut Visibility, With<StopMenu>>| {
                **menu = Visibility::Visible;
            },
        )
        .add_systems(
            OnExit(InMenu::StopMenu),
            |mut menu: Single<&mut Visibility, With<StopMenu>>| {
                **menu = Visibility::Hidden;
            },
        )
        .add_systems(
            Update,
            (
                show_stop_menu
                    .run_if(resource_exists::<CurrentStop>.and(resource_changed::<CurrentStop>)),
                hide_stop_menu.run_if(
                    in_state(GameState::InGame)
                        .and(in_state(InGameState::Running))
                        .and(in_state(InMenu::StopMenu)),
                ),
                fade_title_text
                    .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running))),
                handle_signature_animation
                    .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running))),
            ),
        )
        .add_systems(
            OnEnter(TrainState::Stopped),
            evaluate_contracts
                .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running))),
        )
        .add_systems(OnEnter(TrainState::Arriving), spawn_town_arrival_text);
}

#[derive(Resource)]
pub struct ActiveContracts(pub Vec<Contract>);

#[derive(Debug, Clone)]
pub struct Contract {
    pub required: (Item, usize),
    pub reward: (Item, usize),
    pub stop_number: usize,
}
impl Contract {
    fn generate_random(rng: &mut impl Rng, current_stop_number: usize) -> Self {
        let variants = [
            (Item::Food, 1),
            (Item::Water, 1),
            (Item::Wood, 1),
            (Item::Clay, 1),
            (Item::Brick, 1),
            (Item::Metal, 1),
            (Item::Glass, 1),
            (Item::Bullet, 1),
            (Item::Money, 1),
        ];
        let required = variants
            .choose_weighted(rng, |(_, w)| *w)
            .unwrap()
            .0
            .clone();
        let reward = variants
            .choose_weighted(rng, |(_, w)| *w)
            .unwrap()
            .0
            .clone();
        let required_amount = rng.random_range(15..100);
        let multiplier = ((required_amount as f32) / 10.0).max(1.2).sqrt();
        Contract {
            required: (required, required_amount),
            reward: (reward, (required_amount as f32 * multiplier) as usize),
            stop_number: current_stop_number + rng.random_range(2..=6),
        }
    }
}

#[derive(Component)]
struct StopMenu;
#[derive(Component)]
struct CloseMenuButton;

#[derive(Component)]
struct FadeTitleText;

#[derive(Resource)]
struct FadeTime {
    time: f32,
}

#[derive(Component)]
struct Signature {
    time: f32,
    visible: bool,
}

fn spawn_town_arrival_text(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    next_stop: Res<world_plugin::NextStop>,
    mut fade_time: ResMut<FadeTime>,
) {
    let town_name: String = next_stop.name.to_string();
    if town_name == "Goblin Ambush" {
        return;
    }
    println!("arriving at town: {}", town_name);

    commands.spawn((
        Text::new("Welcome To ".to_string() + &town_name),
        TextFont {
            font: font_assets.town_title_font.clone().into(),
            font_size: 90.0,
            ..Default::default()
        },
        Node {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            bottom: Val::Vh(8.),
            ..default()
        },
        FadeTitleText,
    ));
    fade_time.time = 5.;
}

fn fade_title_text(
    mut commands: Commands,
    mut fade_time: ResMut<FadeTime>,
    time: Res<Time>,
    mut text_colors: Query<&mut TextColor, With<FadeTitleText>>,
    mut entities: Query<Entity, With<FadeTitleText>>,
) {
    if fade_time.time > 0. {
        fade_time.time -= time.delta_secs();
        for mut text_color in &mut text_colors {
            text_color.0 =
                Color::linear_rgba(1.0, 1.0, 1.0, ops::sin((PI / 5.) * fade_time.time).max(0.));
        }
    } else {
        for entity in &mut entities {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct ContractImage;

const CONTRACT_RATIO: f32 = 149.0 / 99.0;
const CONTRACT_WIDTH: f32 = 200.0;

fn spawn_stop_menu(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands
        .spawn((
            Node {
                margin: UiRect::AUTO,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            StopMenu,
            AdvanceBlocker,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    width: Val::Px(CONTRACT_WIDTH * 6.),
                    height: Val::Px(CONTRACT_WIDTH * CONTRACT_RATIO),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,

                    ..Default::default()
                },))
                .with_children(|parent| {
                    for i in 0..6 {
                        parent.spawn((
                            ContractImage,
                            Node {
                                width: Val::Px(CONTRACT_WIDTH),
                                height: Val::Px(CONTRACT_WIDTH * CONTRACT_RATIO),

                                ..Default::default()
                            },
                            ImageNode::new(image_assets.contract.clone()),
                        ));
                    }
                });
            parent.spawn((
                Node {
                    width: Val::Px(160.0),
                    height: Val::Px(20.0),
                    ..Default::default()
                },
                BackgroundColor(Color::WHITE),
                Button,
                CloseMenuButton,
                children![(Text::new("Close"), TextColor(Color::BLACK))],
            ));
        });
}

#[derive(Component)]
struct ContractDisplay;

fn show_stop_menu(
    current_stop: Res<CurrentStop>,
    mut menu: Query<&mut Visibility, With<StopMenu>>,
    mut menu_state: ResMut<NextState<InMenu>>,
    mut commands: Commands,
    contracts: Query<Entity, With<ContractImage>>,
    mut world: ResMut<GameWorld>,
    contract_displays: Query<Entity, With<ContractDisplay>>,
) {
    if let Some(NumberedStop(Stop::Town, current_stop_number)) = current_stop.0 {
        if let Ok(mut menu) = menu.single_mut() {
            *menu = Visibility::Visible;
            menu_state.set(InMenu::StopMenu);

            for contract_display in &contract_displays {
                commands
                    .entity(contract_display)
                    .despawn_related::<Children>()
                    .despawn();
            }

            for booth in contracts {
                let contract = Contract::generate_random(
                    &mut world.rng,
                    current_stop.0.clone().map(|it| it.1).unwrap_or(0),
                );
                commands.entity(booth).with_children(|booth| {
                    booth
                        .spawn((
                            ContractDisplay,
                            Node {
                                // position_type: PositionType::Absolute,
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                display: Display::Flex,
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,

                                ..Default::default()
                            },
                            children![
                                (
                                    Text::new(format!(
                                        "{}x{}",
                                        contract.required.0.name(),
                                        contract.required.1
                                    )),
                                    TextColor(Color::BLACK)
                                ),
                                (Text::new("for"), TextColor(Color::BLACK)),
                                (
                                    Text::new(format!(
                                        "{}x{}",
                                        contract.reward.0.name(),
                                        contract.reward.1
                                    )),
                                    TextColor(Color::BLACK)
                                ),
                                (
                                    Text::new(format!(
                                        "in {} stops",
                                        contract.stop_number - current_stop_number
                                    )),
                                    TextColor(Color::BLACK)
                                ),
                            ], // BackgroundColor(Color::WHITE),
                        ))
                        .with_children(|parent| {
                            let contract_display = parent.target_entity();
                            parent
                                .spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(20.0),
                                        display: Display::Flex,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    // BackgroundColor(YELLOW.into()),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextColor(RED.into()), Text::new("Sign __"),
                                    ))

                                .observe(
                                    move |mut trigger: Trigger<Pointer<Pressed>>,
                                     mut commands: Commands,
                                     mut active_contracts: ResMut<ActiveContracts>,
                                     image_assets: Res<'_, ImageAssets>,
                                     | {
                                        trigger.propagate(false);

                                        commands.entity(contract_display).with_child(
                                            (
                                                Node {
                                                    position_type: PositionType::Absolute,
                                                    width: Val::Px(300. * 0.55),
                                                    height: Val::Px(167. * 0.55),
                                                    bottom: Val::Px(86.),
                                                    left: Val::Px(-13.),
                                                    ..Default::default()
                                                },
                                                ImageNode::new(image_assets.signature_1.clone())
                                                    .with_color(Color::linear_rgba(1., 1., 1., 1.)),
                                                Signature {
                                                    time: 0.,
                                                    visible: true,
                                                }
                                            ),
                                        );
                                        commands
                                            .entity(trigger.event().target)
                                            .despawn_related::<Children>()
                                            // .despawn_related::<ChildOf>()
                                            .despawn();
                                        active_contracts.0.push(contract.clone());
                                    }
                                    );
                                });
                        });
                });
            }
        }
    }
}
fn hide_stop_menu(
    interaction_query: Query<
        (&Interaction, &CloseMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu: Query<&mut Visibility, With<StopMenu>>,
    mut menu_state: ResMut<NextState<InMenu>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            menu_state.set(InMenu::None);
        }
    }
}

fn handle_signature_animation(
    mut commands: Commands,
    mut signatures: Query<(&mut ImageNode, &mut Signature)>,
    time: Res<Time>,
    image_assets: Res<ImageAssets>,
) {
    let anim_images: [Handle<Image>; 13] = [
        image_assets.signature_1.clone(),
        image_assets.signature_2.clone(),
        image_assets.signature_3.clone(),
        image_assets.signature_4.clone(),
        image_assets.signature_5.clone(),
        image_assets.signature_6.clone(),
        image_assets.signature_7.clone(),
        image_assets.signature_8.clone(),
        image_assets.signature_9.clone(),
        image_assets.signature_10.clone(),
        image_assets.signature_11.clone(),
        image_assets.signature_12.clone(),
        image_assets.signature_13.clone(),
    ];

    for mut signature in &mut signatures {
        signature.1.time += time.delta_secs();
        if signature.1.time > 0. && signature.1.time <= (13. / 8.) {
            let idx = ops::floor(signature.1.time * 8.) as usize;
            signature.0.image = anim_images[idx].clone();
        } else {
            signature.0.image = anim_images[12].clone();
        }
    }
}

fn evaluate_contracts(
    mut contracts: ResMut<ActiveContracts>,
    mut inventories: Query<&mut Inventory>,
    current_stop: Res<CurrentStop>,
) {
    info!("Number of contracts: {}", contracts.0.len());
    dbg!(&contracts.0);
    dbg!(&current_stop.0.is_some());

    let contracts_to_check = contracts.0.iter().enumerate().filter_map(|(i, contract)| {
        if contract.stop_number == current_stop.0.clone().map(|it| it.1).unwrap_or(0) {
            Some(i)
        } else {
            None
        }
    });
    let mut to_remove = Vec::with_capacity(contracts.0.len());
    for i in contracts_to_check {
        let contract = &contracts.0[i];
        to_remove.push(i);
        let total_owned = {
            let mut total = 0;
            for mut inventory in &mut inventories {
                total += *inventory
                    .items
                    .entry(contract.required.0.clone())
                    .or_insert(0);
            }
            total
        };
        let required = contract.required.1;
        if total_owned < required {
            info!("Failed contract");
            continue;
        }
        for mut inventory in &mut inventories {
            let owned = inventory
                .items
                .entry(contract.required.0.clone())
                .or_insert(0);
            let actual_given = (*owned).min(required);
            *owned -= actual_given;
        }

        for mut inventory in &mut inventories {
            *inventory
                .items
                .entry(contract.reward.0.clone())
                .or_insert(0) += contract.reward.1;
        }
        info!("Succeeded contract");
    }
    for i in to_remove {
        contracts.0.remove(i);
    }
}
