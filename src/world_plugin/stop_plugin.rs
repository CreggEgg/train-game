use std::f32::consts::PI;

use bevy::{ecs::error::info, prelude::*};
use rand::{Rng, seq::IndexedRandom};
use rand_chacha::ChaCha8Rng;

use crate::{
    FontAssets, GameState, ImageAssets, InGameState, MainGameObject,
    build_plugin::{BuildingTextureAtlas, BuildingType, UnlockedBuildings},
    consume_resource,
    control_panel_plugin::AdvanceBlocker,
    resources_plugin::{Inventory, Item},
    train_plugin::{TrainFuel, TrainLength, TrainState},
    ui_state::InMenu,
    world_plugin::{self},
};

use super::{CurrentStop, GameWorld, NumberedStop, Stop};

fn reset_resources(mut active_contracts: ResMut<ActiveContracts>, mut fade_time: ResMut<FadeTime>) {
    *active_contracts = ActiveContracts(Vec::new());
    *fade_time = FadeTime { time: 0. };
}

pub fn stop_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_stop_menu)
        .add_event::<PurchaseEvent>()
        .insert_resource(ActiveContracts(Vec::new()))
        .insert_resource(FadeTime { time: 0. })
        .add_systems(OnEnter(GameState::MainMenu), reset_resources)
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
                show_stop_menu.run_if(
                    resource_exists::<CurrentStop>
                        .and(resource_changed::<CurrentStop>)
                        .and(in_state(GameState::InGame)),
                ),
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

const REWARDS: &[(Item, usize)] = &[
    (Item::Food, 1),
    (Item::Wood, 1),
    (Item::Clay, 1),
    (Item::Brick, 1),
    (Item::Stone, 1),
    (Item::Metal, 1),
    (Item::Glass, 1),
    (Item::Bullet, 1),
    (Item::Money, 10),
];
const REQUIREMENTS: &[(Item, usize)] = &[
    (Item::Food, 1),
    (Item::Wood, 1),
    (Item::Clay, 1),
    (Item::Brick, 1),
    (Item::Stone, 1),
    (Item::Metal, 1),
    (Item::Glass, 1),
    (Item::Bullet, 1),
    (Item::Money, 0),
];

impl Contract {
    fn generate_random(rng: &mut impl Rng, current_stop_number: usize) -> Self {
        let required = REQUIREMENTS
            .choose_weighted(rng, |(_, w)| *w)
            .unwrap()
            .0
            .clone();
        let reward = REWARDS.choose_weighted(rng, |(_, w)| *w).unwrap().0.clone();
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
    println!("arriving at town: {town_name}");
    let display_text: String = "Welcome To ".to_string() + &town_name;
    println!("{}", display_text.len());
    let text_size: f32 = (100.0 - (1.25 * display_text.len() as f32)).clamp(20., 80.);

    commands.spawn((
        MainGameObject,
        Text::new(display_text),
        TextFont {
            font: font_assets.town_title_font.clone(),
            font_size: text_size,
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

#[derive(Event)]
enum PurchaseEvent {
    SuccessfulPurchase,
    FailedPurchase,
}

fn spawn_stop_menu(
    mut commands: Commands,
    mut world: ResMut<GameWorld>,
    image_assets: Res<ImageAssets>,
    unlocked_buildings: Res<UnlockedBuildings>,
    building_texture_atlas: Res<BuildingTextureAtlas>,
) {
    commands
        .spawn((
            MainGameObject,
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
                .spawn((
                    Node {
                        // width: Val::Px(160.0),
                        height: Val::Px(40.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::WHITE),
                    children![(Text::new("Buy train car $1000"), TextColor(Color::BLACK))],
                    Pickable::default(),
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Pressed>>,
                     mut train_length: ResMut<TrainLength>,
                     mut inventories: Query<&mut Inventory>,
                     mut ev: EventWriter<PurchaseEvent>| {
                        consume_resource!(
                            1000,
                            inventories,
                            {
                                ev.write(PurchaseEvent::FailedPurchase);
                                return;
                            },
                            {
                                train_length.0 += 1;
                                ev.write(PurchaseEvent::SuccessfulPurchase);
                            }
                        )
                    },
                );
            parent
                .spawn((
                    Node {
                        // width: Val::Px(160.0),
                        height: Val::Px(40.0),
                        ..Default::default()
                    },
                    BackgroundColor(Color::WHITE),
                    children![(Text::new("Buy 100 Fuel $100"), TextColor(Color::BLACK))],
                    Pickable::default(),
                ))
                .observe(
                    |_trigger: Trigger<Pointer<Pressed>>,
                     mut train_fuel: ResMut<TrainFuel>,
                     mut inventories: Query<&mut Inventory>,
                     mut ev: EventWriter<PurchaseEvent>| {
                        consume_resource!(
                            100,
                            inventories,
                            {
                                ev.write(PurchaseEvent::FailedPurchase);
                                return;
                            },
                            {
                                train_fuel.0 += 100.0;
                                ev.write(PurchaseEvent::SuccessfulPurchase);
                            }
                        )
                    },
                );
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
                    for _ in 0..6 {
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
            parent
                .spawn((
                    Node {
                        display: Display::Grid,
                        width: Val::Px(CONTRACT_WIDTH * 6.),
                        aspect_ratio: Some(1.7),

                        grid_template_columns: RepeatedGridTrack::flex(4, 1.0),
                        row_gap: Val::Px(0.0),
                        column_gap: Val::Px(0.0),
                        // Set the grid to have 4 rows all with sizes minmax(0, 1fr)
                        // This creates 4 exactly evenly sized rows
                        ..Default::default()
                    },
                    Name::new("Blueprints menu"),
                    BackgroundColor(Color::WHITE),
                ))
                .with_children(|parent| {
                    for building_type in BuildingType::iterator() {
                        if unlocked_buildings.0.contains(&building_type) {
                            continue;
                        }
                        let building_cost = building_type.get_blueprint_cost();

                        parent
                            .spawn((
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Column,
                                    ..Default::default()
                                },
                                Name::new("Blueprint Toplevel"),
                                BlueprintPurchased(false),
                                children![
                                    (
                                        Name::new("Blueprint Text"),
                                        Text::new(building_type.name()),
                                        TextColor::BLACK,
                                        BackgroundColor(Color::srgb_from_array(world.rng.random())),
                                    ),
                                    (
                                        Node {
                                            aspect_ratio: Some(1.0),
                                            width: Val::Percent(100.0),
                                            ..Default::default()
                                        },
                                        Name::new("Blueprint Image"),
                                        BackgroundColor(Color::srgb_from_array(world.rng.random())),
                                        ImageNode::from_atlas_image(
                                            building_type.get_texture(&image_assets),
                                            building_texture_atlas.0.clone()
                                        )
                                    )
                                ],
                            ))
                            .observe(
                                move |mut trigger: Trigger<Pointer<Pressed>>,
                                      mut commands: Commands,
                                      mut blueprint_items: Query<&mut BlueprintPurchased>,
                                      mut inventories: Query<&mut Inventory>,
                                      mut ev: EventWriter<PurchaseEvent>| {
                                    trigger.propagate(false);

                                    // commands.entity(trigger.target()).log_components();

                                    // println!("{}", blueprint_items.get(trigger.target()).unwrap());
                                    consume_resource!(building_cost, inventories, {
                                        ev.write(PurchaseEvent::FailedPurchase);
                                    }, {
                                        blueprint_items.get_mut(trigger.target()).unwrap().0 = true;
                                        commands
                                            .entity(trigger.target())
                                            .despawn_related::<Children>();
                                        commands.entity(trigger.target()).despawn();
                                        ev.write(PurchaseEvent::SuccessfulPurchase);
                                    })

                                    //
                                    // println!("{}", trigger.observer());
                                    // println!("{}", trigger.event().target);
                                },
                            );
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
struct BlueprintPurchased(bool);

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
    image_assets: Res<ImageAssets>,
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
                let item_image_required: Handle<Image> =
                    contract.required.0.get_image(&image_assets);
                let item_image_reward: Handle<Image> = contract.reward.0.get_image(&image_assets);

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
                                (Node {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(10.),
                                    ..Default::default()
                                },),
                                (
                                    Node {
                                        width: Val::Percent(100.),
                                        height: Val::Px(30.),
                                        flex_direction: FlexDirection::Row,
                                        ..Default::default()
                                    },
                                    children![
                                        (
                                            ImageNode::new(item_image_required.clone()),
                                            Node {
                                                position_type: PositionType::Absolute,
                                                left: Val::Percent(29.),
                                                top: Val::Percent(-24.),
                                                ..Default::default()
                                            }
                                        ),
                                        (
                                            Node {
                                                position_type: PositionType::Absolute,
                                                left: Val::Percent(52.),
                                                top: Val::Percent(6.),
                                                ..Default::default()
                                            },
                                            Text::new(format!("x{}", contract.required.1)),
                                            TextColor(Color::BLACK),
                                        )
                                    ]
                                ),
                                (Text::new("for"), TextColor(Color::BLACK)),
                                (
                                    Node {
                                        width: Val::Percent(100.),
                                        height: Val::Px(30.),
                                        flex_direction: FlexDirection::Row,
                                        ..Default::default()
                                    },
                                    children![
                                        (
                                            ImageNode::new(item_image_reward.clone()),
                                            Node {
                                                position_type: PositionType::Absolute,
                                                left: Val::Percent(29.),
                                                top: Val::Percent(-24.),
                                                ..Default::default()
                                            }
                                        ),
                                        (
                                            Node {
                                                position_type: PositionType::Absolute,
                                                left: Val::Percent(52.),
                                                top: Val::Percent(6.),
                                                ..Default::default()
                                            },
                                            Text::new(format!("x{}", contract.reward.1)),
                                            TextColor(Color::BLACK),
                                        )
                                    ]
                                ),
                                (
                                    Text::new(format!(
                                        "in {} stops",
                                        contract.stop_number - current_stop_number
                                    )),
                                    TextColor(Color::BLACK)
                                ),
                                (
                                    Node {
                                        position_type: PositionType::Absolute,
                                        bottom: Val::Percent(22.0),
                                        left: Val::Percent(21.),
                                        ..Default::default()
                                    },
                                    Text::new("X".to_string()),
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
                                        height: Val::Percent(22.0),
                                        display: Display::Flex,
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    //BackgroundColor(YELLOW.into()),
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextColor(Color::BLACK), Text::new("        "),
                                        Node {
                                            height: Val::Percent(80.),
                                            ..Default::default()
                                        }
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
                                                    width: Val::Px(300. * 0.5),
                                                    height: Val::Px(167. * 0.5),
                                                    bottom: Val::Px(63.),
                                                    left: Val::Px(1.),
                                                    ..Default::default()
                                                },
                                                ImageNode::new(image_assets.signature_1.clone())
                                                    .with_color(Color::linear_rgba(1., 1., 1., 1.)),
                                                Signature {
                                                    time: 0.,
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
        &Interaction,
        (Changed<Interaction>, With<Button>, With<CloseMenuButton>),
    >,
    // _menu: Query<&mut Visibility, With<StopMenu>>,
    mut menu_state: ResMut<NextState<InMenu>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            menu_state.set(InMenu::None);
        }
    }
}

fn handle_signature_animation(
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
        consume_resource!(
            contract.required.0.clone(),
            contract.required.1,
            inventories,
            {
                info!("Failed contract");
                continue;
            },
            {
                for mut inventory in &mut inventories {
                    *inventory
                        .items
                        .entry(contract.reward.0.clone())
                        .or_insert(0) += contract.reward.1;
                    break;
                }
                info!("Succeeded contract");
            }
        );
    }
    for i in to_remove {
        contracts.0.remove(i);
    }
}
