use bevy::prelude::*;

use crate::{FontAssets, GameState, MainGameObject, resources_plugin::Inventory, ui_state::InMenu};

use super::{
    Building,
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
            MainGameObject,
            Node {
                width: Val::Vw(60.0),
                height: Val::Vh(60.0),
                margin: UiRect::AUTO,
                ..Default::default()
            },
            BuildingMenu,
            Visibility::Hidden,
            BackgroundColor(Color::BLACK),
            children![(
                Node {
                    width: Val::Percent(100.0),
                    top: Val::Px(25.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                BackgroundColor(Color::WHITE),
                BuildingMenuSlot
            )],
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(25.0),
                        height: Val::Px(25.0),
                        display: Display::Flex,
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.),
                        right: Val::Px(0.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,

                        ..Default::default()
                    },
                    BackgroundColor(Color::WHITE),
                    Pickable::default(),
                    children![(Text::new("X"), TextColor(Color::BLACK))],
                ))
                .observe(
                    |mut trigger: Trigger<Pointer<Click>>,
                     mut next_state: ResMut<NextState<InMenu>>| {
                        next_state.set(InMenu::None);
                    },
                );
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
    mut buildings: Query<(Entity, &Building, Option<&Inventory>, Option<&mut Roost>)>,
    building_menu_slot: Single<Entity, With<BuildingMenuSlot>>,
    mut commands: Commands,
    font_assets: Res<FontAssets>,
) {
    let Some(entity) = inspected_building.0 else {
        return;
    };
    let Ok((building_entity, building, inventory, roost)) = buildings.get_mut(entity) else {
        inspected_building.0 = None;
        return;
    };

    commands
        .entity(*building_menu_slot)
        .despawn_related::<Children>()
        .with_children(|parent| {
            parent.spawn((Text::new(building.0.name()), TextColor::BLACK));
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
                super::BuildingType::Roost => {
                    roost_menu(parent, &mut roost.unwrap(), building_entity);
                }
                _ => {}
            }
        });
}
