use bevy::{color::palettes::tailwind::GREEN_200, prelude::*};

use crate::{
    FontAssets, GameState, ImageAssets, MainGameObject, resources_plugin::Inventory,
    ui_state::InMenu,
};

use super::{
    Building, LiquidTank, ResourceProduction, Workshop,
    bird_plane::{Roost, roost_menu},
};

fn reset_resources(mut inspected_building: ResMut<BuildingInspected>) {
    *inspected_building = BuildingInspected(None);
}

pub fn building_menus_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_building_menu)
        .add_systems(OnEnter(GameState::MainMenu), reset_resources)
        .add_systems(OnEnter(InMenu::BuildingMenu), show_building_menu)
        .add_systems(OnExit(InMenu::BuildingMenu), hide_building_menu)
        .add_systems(
            FixedUpdate,
            update_inspected_building.run_if(
                resource_exists::<BuildingInspected>.and(resource_changed::<BuildingInspected>),
            ),
        )
        .insert_resource(BuildingInspected(None));
    // .add_event::<InspectBuilding>();
}

#[derive(Resource)]
pub struct BuildingInspected(pub Option<Entity>);

#[derive(Component)]
struct BuildingMenu;

#[derive(Component)]
struct BuildingMenuSlot;

// #[derive(Event)]
// pub struct InspectBuilding {
//     pub building: Entity,
// }

fn spawn_building_menu(mut commands: Commands) {
    commands
        .spawn((
            Pickable::default(),
            MainGameObject,
            Visibility::Hidden,
            BuildingMenu,
            Node {
                width: Val::Vw(100.0),
                height: Val::Vh(100.0),
                display: Display::Flex,
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                right: Val::Px(0.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,

                ..Default::default()
            },
            // BackgroundColor(Color::WHITE),
            // children![(Text::new("X"), TextColor(Color::BLACK))],
        ))
        .observe(
            |mut trigger: Trigger<Pointer<Click>>, mut next_state: ResMut<NextState<InMenu>>| {
                next_state.set(InMenu::None);
                trigger.propagate(false);
            },
        )
        .with_children(|parent| {
            parent
                .spawn((
                    // Pickable::default(),
                    Node {
                        width: Val::Vw(60.0),
                        height: Val::Vh(60.0),
                        margin: UiRect::AUTO,
                        padding: UiRect::all(Val::Px(19.0)),
                        ..Default::default()
                    },
                    BorderRadius::all(Val::Px(20.0)),
                    BackgroundColor(Color::srgba(0.7, 0.7, 0.7, 0.55)),
                    children![(
                        Node {
                            width: Val::Percent(100.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        // BackgroundColor(Color::WHITE),
                        BuildingMenuSlot
                    )],
                ))
                .observe(|mut trigger: Trigger<Pointer<Click>>| {
                    trigger.propagate(false);
                });
        });
}

fn show_building_menu(mut menu: Query<&mut Visibility, With<BuildingMenu>>) {
    *menu.single_mut().unwrap() = Visibility::Visible;
}

fn hide_building_menu(mut menu: Query<&mut Visibility, With<BuildingMenu>>) {
    *menu.single_mut().unwrap() = Visibility::Hidden;
}

fn update_inspected_building(
    mut inspected_building: ResMut<BuildingInspected>,
    mut buildings: Query<(
        Entity,
        &Building,
        Option<&Inventory>,
        Option<&mut Roost>,
        Option<&ResourceProduction>,
        Option<&LiquidTank>,
        Option<&Workshop>,
    )>,
    building_menu_slot: Single<Entity, With<BuildingMenuSlot>>,
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    image_assets: Res<ImageAssets>,
) {
    let Some(entity) = inspected_building.0 else {
        return;
    };
    let Ok((
        building_entity,
        building,
        inventory,
        roost,
        resource_production,
        liquid_tank,
        workshop,
    )) = buildings.get_mut(entity)
    else {
        inspected_building.0 = None;
        return;
    };
    commands
        .entity(*building_menu_slot)
        .despawn_related::<Children>()
        .with_children(|parent| {
            parent.spawn((
                Text::new(building.0.name()),
                TextColor::BLACK,
                TextFont {
                    font_size: 48.0,
                    ..Default::default()
                },
            ));
            match building.0 {
                super::BuildingType::Storage => {
                    for (item, amount) in &inventory.unwrap().items {
                        // parent.spawn((Text::new(item.name())));
                        parent.spawn((
                            TextColor::BLACK,
                            Text::new(format!("{}x{}", item.name(), amount)),
                            TextFont::from_font(font_assets.default_font.clone()),
                        ));
                    }
                    if inventory.unwrap().is_empty() {
                        parent.spawn((
                            TextColor::BLACK,
                            Text::new("Empty"),
                            TextFont::from_font(font_assets.default_font.clone()),
                        ));
                    }
                }
                super::BuildingType::LiquidTank => {
                    let liquid_tank = liquid_tank.unwrap();
                    parent.spawn((Text::new(format!("{}/{} L {}", 
                        liquid_tank.contained_liters,
                        liquid_tank.max_liters,
                        if let Some(contained) = &liquid_tank.contained_fluid {
                            contained.name()
                        } else {
                            ""
                        })), TextColor::BLACK));
                }
                super::BuildingType::Roost => {
                    roost_menu(parent, &roost.unwrap(), building_entity);
                }
                super::BuildingType::Workshop => {
                    let workshop = workshop.unwrap();
                    parent.spawn((Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        ..Default::default()},
                        BackgroundColor(Color::BLACK), children!
                        [
                            (Node {
                                width: Val::Percent(100.0 * (workshop.progress.elapsed_secs() / workshop.progress.duration().as_secs_f32())),
                                height: Val::Percent(100.0),
                                ..Default::default()
                            }, 
                            BackgroundColor(GREEN_200.into()))
                    ]));
                }
                super::BuildingType::AlchemyLab => {
                    parent
                        .spawn((Node {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            display: Display::Flex,
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },))
                        .with_children(|parent| {
                            for recipe in building.0.get_resource_production() {
                                let active = {
                                    if let Some(resource_production) = resource_production {
                                        recipe.input == resource_production.input
                                            && recipe.output == resource_production.output
                                            && recipe.timer.duration()
                                                == resource_production.timer.duration()
                                    } else {
                                        false
                                    }
                                };

                                let mut button = parent.spawn((
                                    Node {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(100.0),
                                        display: Display::Flex,
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..Default::default()
                                    },
                                    RecipeSwitchButton(recipe.clone()),
                                    BackgroundColor(if active {
                                            Color::srgba(0.0, 0.0, 0.0, 0.2)
                                        } else {
                                            Color::srgba(0.0, 0.0, 0.0, 0.7)
                                        }),
                                        BorderRadius::all(Val::Px(15.0)),

                                        children![
                                            ImageNode::new(recipe.input.clone().unwrap().0.get_image(&image_assets)),
                                            Text::new(format!("x{}", recipe.input.clone().unwrap().1)),

                                            Text::new("=>"),
                                            ImageNode::new(recipe.output.0.get_image(&image_assets)),
                                            Text::new(format!("x{}", recipe.output.clone().1)),
                                        ]
                                    ),

                                );
                                button.observe(
                                    move |mut _trigger: Trigger<Pointer<Pressed>>,
                                    mut backgrounds: Query<(&mut BackgroundColor, &RecipeSwitchButton)>,
                                     mut buildings: Query<
                                        &mut ResourceProduction,
                                    >| {
                                        let mut resource_production = buildings.get_mut(building_entity).unwrap();
                                        *resource_production = recipe.clone();

                                        for (mut button, RecipeSwitchButton(recipe)) in &mut backgrounds  {
                                            let active = {
                                                recipe.input == resource_production.input
                                                    && recipe.output == resource_production.output
                                                    && recipe.timer.duration()
                                                        == resource_production.timer.duration()
                                            };

                                            button.0 = if active {
                                                Color::srgba(0.0, 0.0, 0.0, 0.2)
                                            } else {
                                                Color::srgba(0.0, 0.0, 0.0, 0.7)
                                            };
                                        }
                                    },
                                );
                            }
                        });
                }
                _ => {}
            }
        });
}

#[derive(Component)]
struct RecipeSwitchButton(ResourceProduction);
