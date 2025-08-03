use core::f32;
use std::time::Duration;

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    math::FloatPow,
    picking::hover::HoverMap,
    platform::collections::HashMap,
    prelude::*,
    window::PrimaryWindow,
};
use bird_plane::Roost;
use building_menus::BuildingInspected;
use synergies::Synergized;

use crate::{
    GameState, ImageAssets, InGameState, MainGameObject,
    animations::Animation,
    consume_resource,
    resources_plugin::{Fluid, Inventory, Item},
    train_plugin::TrainState,
    ui_state::InMenu,
};

// #[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
// pub enum BuildState {
//     Building,
//     #[default]
//     NotBuilding,
// }
mod bird_plane;
mod building_menus;
pub mod synergies;

#[derive(Resource, Clone, Copy, serde::Deserialize, Hash, PartialEq, Eq, Debug)]
pub enum BuildingType {
    Housing,
    Farm,
    Storage,
    Sawmill,
    AlchemyLab,
    Cannon,
    Workshop,
    Roost,
    LiquidTank,
    Factory,
}

#[derive(Resource)]
pub struct UnlockedBuildings(pub Vec<BuildingType>);

impl BuildingType {
    pub fn get_texture(&self, image_assets: &ImageAssets) -> Handle<Image> {
        match self {
            BuildingType::Housing => image_assets.housing.clone(),
            BuildingType::Farm => image_assets.farm.clone(),
            BuildingType::Storage => image_assets.shipping_container.clone(),
            BuildingType::AlchemyLab => image_assets.alchemy_lab_1.clone(),
            BuildingType::Workshop => image_assets.workshop.clone(),
            BuildingType::Roost => image_assets.roost.clone(),
            _ => image_assets.debug_building.clone(),
        }
    }
    pub fn get_build_locations(&self) -> Vec<Vec2> {
        match self {
            BuildingType::Farm => vec![],
            BuildingType::Workshop => vec![],
            BuildingType::Roost => vec![],
            _ => vec![Vec2::new(0., 45.)],
        }
    }

    pub fn iterator() -> impl Iterator<Item = Self> {
        use BuildingType::*;
        [
            Housing, Farm, Storage, Sawmill, AlchemyLab, Cannon, Workshop, Roost, LiquidTank,
        ]
        .into_iter()
    }

    pub fn name(&self) -> &'static str {
        use BuildingType::*;
        match self {
            Housing => "Housing",
            Farm => "Farm",
            Storage => "Storage",
            Sawmill => "Sawmill",
            AlchemyLab => "Alchemy Lab",
            Cannon => "Cannon",
            Workshop => "Workshop",
            Roost => "Roost",
            LiquidTank => "Liquid Tank",
            Factory => "Factory",
        }
    }

    fn get_resource_production(&self) -> Vec<ResourceProduction> {
        use BuildingType::*;
        match self {
            Housing => vec![],
            Farm => vec![ResourceProduction {
                timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
                output: (Item::Food, 1),
                input: None,
            }],
            Sawmill => vec![ResourceProduction {
                timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
                output: (Item::Wood, 1),
                input: None,
            }],
            Storage => vec![],
            AlchemyLab => vec![
                ResourceProduction {
                    timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
                    input: Some((Item::Wood, 5)),
                    output: (Item::Stone, 1),
                },
                ResourceProduction {
                    timer: Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating),
                    input: Some((Item::Stone, 5)),
                    output: (Item::Metal, 1),
                },
            ],
            Cannon => vec![],
            Workshop => vec![],
            Roost => vec![],
            LiquidTank => vec![],
            Factory => vec![],
        }
    }

    pub(crate) fn get_blueprint_cost(&self) -> usize {
        use BuildingType::*;
        match self {
            Housing => 100,
            Farm => 50,
            Storage => 100,
            Sawmill => 100,
            AlchemyLab => 200,
            Cannon => 100,
            Workshop => 500,
            Roost => 500,
            LiquidTank => 100,
            Factory => 150,
        }
    }
}

#[derive(Component)]
pub struct BuildLocation(pub Vec2);

#[derive(Component)]
struct GhostBuilding;

#[derive(Component)]
pub struct Building(BuildingType);

#[derive(Component, Clone)]
pub struct ResourceProduction {
    pub timer: Timer,
    pub output: (Item, usize),
    pub input: Option<(Item, usize)>,
}

#[derive(Component)]
pub struct LiquidTank {
    pub contained_liters: f32,
    pub max_liters: f32,
    pub contained_fluid: Option<Fluid>,
}

impl Default for LiquidTank {
    fn default() -> Self {
        Self {
            contained_liters: 0.,
            max_liters: 1000.0,
            contained_fluid: None,
        }
    }
}

fn reset_resources(mut building_type: ResMut<BuildingType>) {
    *building_type = BuildingType::Farm;
}

pub fn build_plugin(app: &mut App) {
    app //.init_state::<BuildState>()
        .insert_resource(UnlockedBuildings(vec![
            BuildingType::Farm,
            BuildingType::Storage,
        ]))
        .insert_resource(BuildingType::Farm)
        .add_event::<BuildEvent>()
        .add_plugins((
            building_menus::building_menus_plugin,
            bird_plane::bird_plane_plugin,
            synergies::synergies_plugin,
        ))
        .add_systems(OnEnter(GameState::MainMenu), reset_resources)
        .add_systems(
            Update,
            (construct_buildings, change_selected_building).run_if(
                in_state(GameState::InGame)
                    .and(in_state(InGameState::Running))
                    .and(in_state(InMenu::BuildMenu)),
            ),
        )
        .add_systems(
            FixedUpdate,
            update_ghost.run_if(
                in_state(InMenu::BuildMenu)
                    .and(in_state(GameState::InGame))
                    .and(resource_exists::<BuildingType>)
                    .and(resource_changed::<BuildingType>),
            ),
        )
        // .insert_resource(WinitSettings::desktop_app())
        .add_systems(
            Update,
            update_scroll_position.run_if(in_state(InMenu::BuildMenu)),
        )
        .add_systems(
            OnEnter(InMenu::BuildMenu),
            |mut ghost: Query<&mut Visibility, With<BuildMenuItem>>| {
                for mut build_menu_item in &mut ghost {
                    *build_menu_item = Visibility::Visible;
                }
            },
        )
        .add_systems(
            OnExit(InMenu::BuildMenu),
            |mut ghost: Query<&mut Visibility, With<BuildMenuItem>>| {
                for mut build_menu_item in &mut ghost {
                    *build_menu_item = Visibility::Hidden;
                }
            },
        )
        .add_systems(
            FixedUpdate,
            (
                on_build.run_if(in_state(InMenu::BuildMenu)),
                update_build_menu.run_if(resource_changed::<UnlockedBuildings>),
            ),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                spawn_ghost,
                (spawn_blueprint_window, update_build_menu).chain(),
            ),
        )
        .add_systems(
            FixedUpdate,
            produce_resources.run_if(
                in_state(GameState::InGame)
                    .and(in_state(InGameState::Running))
                    .and(in_state(TrainState::Advancing)),
            ),
        )
        .add_systems(Startup, building_texture_atlas);
}

#[derive(Resource)]
pub struct BuildingTextureAtlas(pub TextureAtlas);

fn building_texture_atlas(
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let texture_atlas =
        TextureAtlasLayout::from_grid(UVec2::splat(80), 1, 1, None, Some(UVec2::new(200, 60)));
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let texture_atlas = TextureAtlas::from(texture_atlas_handle);
    commands.insert_resource(BuildingTextureAtlas(texture_atlas));
}

#[derive(Component)]
struct BuildMenuItem;

fn spawn_ghost(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn((
        MainGameObject,
        Visibility::Hidden,
        BuildMenuItem,
        GhostBuilding,
        Sprite::from_image(image_assets.farm.clone()),
        Transform::from_xyz(0., 0., 5.0),
    ));
}

fn update_ghost(
    mut ghost: Query<&mut Sprite, With<GhostBuilding>>,
    building_type: Res<BuildingType>,
    image_assets: Res<ImageAssets>,
) {
    let mut ghost = ghost.single_mut().unwrap();
    ghost.image = building_type.get_texture(&image_assets);
}

#[derive(Component)]
struct BluePrintButton(BuildingType);

#[derive(Component)]
struct BuildMenu;

fn spawn_blueprint_window(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    unlocked_buildings: Res<UnlockedBuildings>,
) {
    commands.spawn((
        MainGameObject,
        Visibility::Hidden,
        BuildMenuItem,
        BuildMenu,
        Node {
            top: Val::Vh(5.0),
            // height: Val::Percent(100.0),
            bottom: Val::Px(0.0),
            right: Val::Px(0.),
            display: Display::Flex,
            position_type: PositionType::Absolute,
            // justify_content: JustifyContent::End,
            // align_items: AlignItems::FlexEnd,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.0)),
            margin: UiRect::top(Val::Px(10.0)),

            align_self: AlignSelf::Stretch,

            overflow: Overflow::scroll_y(),
            ..Default::default()
        },
    ));
}

fn update_build_menu(
    menu: Single<Entity, With<BuildMenu>>,
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    building_texture_atlas: Res<BuildingTextureAtlas>,
    unlocked_buildings: Res<UnlockedBuildings>,
) {
    commands.entity(*menu).with_children(|parent| {
        for building_type in &unlocked_buildings.0 {
            parent.spawn((
                Node {
                    width: Val::Px(142.0),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                Pickable {
                    should_block_lower: false,
                    ..default()
                },
                children![
                    (
                        ImageNode::from_atlas_image(
                            building_type.get_texture(&image_assets),
                            building_texture_atlas.0.clone(),
                        ),
                        Node {
                            width: Val::Px(142.0),
                            height: Val::Px(142.0),
                            bottom: Val::Px(0.0),

                            ..Default::default()
                        },
                        BluePrintButton(building_type.clone()),
                        Button,
                        Pickable {
                            should_block_lower: false,
                            ..default()
                        },
                    ),
                    (
                        Node {
                            width: Val::Percent(100.0),
                            display: Display::Flex,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,

                            ..default()
                        },
                        children![(
                            Text::new(building_type.name()),
                            Pickable {
                                should_block_lower: false,
                                ..default()
                            },
                        )],
                        Pickable {
                            should_block_lower: false,
                            ..default()
                        },
                    ),
                ],
            ));
        }
    });
}

fn change_selected_building(
    interaction_query: Query<
        (&Interaction, &BluePrintButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut current_building: ResMut<BuildingType>,
) {
    for (interaction, BluePrintButton(building_type)) in &interaction_query {
        if *interaction == Interaction::Pressed {
            *current_building = *building_type;
        }
    }
}

pub const MAX_CONSTRUCTION_SNAPPING: f32 = 40.0;

#[derive(Event)]
pub struct BuildEvent {
    child_of: Entity,
    offset: Vec2,
    building_type: BuildingType,
}

fn construct_buildings(
    window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<IsDefaultUiCamera>>,
    mut ghost: Query<(&mut Sprite, &mut Transform), With<GhostBuilding>>,
    build_locations: Query<(Entity, &BuildLocation, &GlobalTransform, &ChildOf)>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut ev: EventWriter<BuildEvent>,
    mut commands: Commands,
    building_type: Res<BuildingType>,
) {
    let Ok((camera, camera_transform)) = q_camera.single() else {
        return;
    };
    if let Some(position) = window
        .cursor_position()
        .and_then(|position| camera.viewport_to_world_2d(camera_transform, position).ok())
    {
        let (mut ghost_sprite, mut ghost_transform) = ghost.single_mut().unwrap();

        let mut closest: Option<(f32, Entity, &BuildLocation, &GlobalTransform, &ChildOf)> = None;
        for (build_entity, build_location, build_transform, build_parent) in build_locations {
            let closest_distance = closest
                .map(|(distance, _, _, _, _)| distance)
                .unwrap_or(MAX_CONSTRUCTION_SNAPPING.squared());
            let distance = (position - (build_transform.translation().xy() + build_location.0))
                .length_squared();
            if distance < closest_distance {
                closest = Some((
                    distance,
                    build_entity,
                    build_location,
                    build_transform,
                    build_parent,
                ));
            }
        }
        if let Some((_, build_entity, build_location, build_transform, build_parent)) = closest {
            ghost_sprite.color = Color::srgb(0.0, 1., 0.);
            ghost_transform.translation =
                build_transform.translation() + build_location.0.extend(5.0);
            if buttons.just_pressed(MouseButton::Left) {
                commands.entity(build_entity).despawn();
                ev.write(BuildEvent {
                    child_of: build_parent.0,
                    offset: build_location.0,
                    building_type: *building_type,
                });
            }
        } else {
            ghost_sprite.color = Color::srgb(1.0, 0., 0.);
            ghost_transform.translation = position.extend(5.);
        }
    }
}

fn on_build(
    mut ev: EventReader<BuildEvent>,
    parents: Query<Entity, With<Transform>>,
    image_assets: Res<ImageAssets>,
    mut commands: Commands,
) {
    for BuildEvent {
        child_of,
        offset,
        building_type,
    } in ev.read()
    {
        let parent = parents.get(*child_of).unwrap();
        let mut building = commands.spawn((
            MainGameObject,
            Sprite::from_image(building_type.get_texture(&image_assets)),
            Transform::from_translation(offset.extend(4.0)),
            Building(*building_type),
            // children![(BuildLocation(Vec2::new(0., 40.)), Transform::default())],
            //
            Pickable::default(),
        ));
        if let Some(resource_production) = building_type.get_resource_production().first() {
            building.insert(resource_production.clone());
        }
        building.with_children(|parent| {
            for build_location in building_type.get_build_locations() {
                parent.spawn((BuildLocation(build_location), Transform::default()));
            }
        });
        match building_type {
            BuildingType::Storage => {
                building.insert(Inventory::default());
            }
            BuildingType::LiquidTank => {
                building.insert(LiquidTank::default());
            }
            BuildingType::Roost => {
                building.insert(Roost::default());
            }

            BuildingType::AlchemyLab => {
                building.insert(Animation(
                    vec![
                        image_assets.alchemy_lab_1.clone(),
                        image_assets.alchemy_lab_2.clone(),
                        image_assets.alchemy_lab_3.clone(),
                        image_assets.alchemy_lab_4.clone(),
                        image_assets.alchemy_lab_5.clone(),
                        image_assets.alchemy_lab_6.clone(),
                        image_assets.alchemy_lab_7.clone(),
                        image_assets.alchemy_lab_8.clone(),
                        image_assets.alchemy_lab_9.clone(),
                        image_assets.alchemy_lab_10.clone(),
                        image_assets.alchemy_lab_11.clone(),
                        image_assets.alchemy_lab_12.clone(),
                        image_assets.alchemy_lab_13.clone(),
                        image_assets.alchemy_lab_14.clone(),
                        image_assets.alchemy_lab_15.clone(),
                        image_assets.alchemy_lab_16.clone(),
                        image_assets.alchemy_lab_17.clone(),
                        image_assets.alchemy_lab_18.clone(),
                    ],
                    0,
                ));
            }
            _ => {}
        }

        let building_id = building.id();
        building.observe(
            move |mut trigger: Trigger<Pointer<Click>>,
                  mut selected_building: ResMut<BuildingInspected>,
                  mut menu_state: ResMut<NextState<InMenu>>,
                  current_menu_state: Res<State<InMenu>>| {
                if let InMenu::None = **current_menu_state {
                    println!("got click");
                    selected_building.0 = Some(building_id);
                    menu_state.set(InMenu::BuildingMenu);
                    trigger.propagate(false);
                }
            },
        );
        commands.entity(parent).add_child(building_id);
    }
}

fn produce_resources(
    mut buildings: Query<(&mut ResourceProduction, Option<&Synergized>)>,
    mut inventories: Query<&mut Inventory>,
    time: Res<Time>,
) {
    let mut produced_items = HashMap::new();
    for (mut building, synergized) in &mut buildings {
        if building.timer.tick(time.delta()).just_finished() {
            if let Some(required) = &building.input {
                consume_resource!(
                    required.0.clone(),
                    required.1,
                    inventories,
                    {
                        info!("Failed to produce output");
                        continue;
                    },
                    {}
                );
            }
            *produced_items.entry(building.output.0.clone()).or_insert(0) +=
                building.output.1 * if synergized.is_some() { 2 } else { 1 };
        }
    }
    for (item, amount) in produced_items {
        for mut inventory in &mut inventories {
            *inventory.items.entry(item.clone()).or_insert(0) += amount;
            break;
        }
    }
}

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (mouse_wheel_event.x * 21.0, mouse_wheel_event.y * 21.0),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}
