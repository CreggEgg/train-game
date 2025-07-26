#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use bevy::{prelude::*, window::WindowMode};
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
};
use bevy_common_assets::ron::RonAssetPlugin;
use build_plugin::synergies::Synergies;
use ui_state::InMenu;

mod animations;
mod build_plugin;
mod camera_plugin;
mod control_panel_plugin;
mod debug_plugin;
mod goblins;
mod main_menu;
#[cfg(not(target_family = "wasm"))]
mod particles_plugin;
mod resources_plugin;
mod train_plugin;
mod ui_state;
mod world_plugin;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum GameState {
    #[default]
    MainMenu,
    Loading,
    InGame,
}

#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
enum InGameState {
    #[default]
    Running,
    Paused,
}

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(path = "traincar.png")]
    train_car: Handle<Image>,
    #[asset(path = "trainlocomotive.png")]
    train_locomotive: Handle<Image>,
    #[asset(path = "caboose.png")]
    train_caboose: Handle<Image>,
    #[asset(path = "stop_bg.png")]
    stop_bg: Handle<Image>,
    #[asset(path = "stop_fg.png")]
    stop_fg: Handle<Image>,
    #[asset(path = "goblinstop_bg.png")]
    goblin_stop_bg: Handle<Image>,
    #[asset(path = "goblinstop_fg.png")]
    goblin_stop_fg: Handle<Image>,
    #[asset(path = "mine_stop.png")]
    mine_stop: Handle<Image>,
    #[asset(path = "farm.png")]
    farm: Handle<Image>,
    #[asset(path = "rail.png")]
    rail: Handle<Image>,
    #[asset(path = "housing.png")]
    housing: Handle<Image>,
    #[asset(path = "shippingcontainer.png")]
    shipping_container: Handle<Image>,
    #[asset(path = "Contract.png")]
    contract: Handle<Image>,
    // #[asset(path = "BoothCard.png")]
    // booth_card: Handle<Image>,
    #[asset(path = "DebugBuilding.png")]
    debug_building: Handle<Image>,
    #[asset(path = "Ground.png")]
    ground: Handle<Image>,
    #[asset(path = "map_pin.png")]
    map_pin: Handle<Image>,
    #[asset(path = "workshop.png")]
    workshop: Handle<Image>,
    #[asset(path = "Roost.png")]
    roost: Handle<Image>,

    #[asset(path = "smoke.png")]
    steam_particle: Handle<Image>,

    #[asset(path = "items/metal.png")]
    item_metal: Handle<Image>,
    #[asset(path = "items/wood.png")]
    item_wood: Handle<Image>,
    #[asset(path = "items/stone.png")]
    item_stone: Handle<Image>,

    #[asset(path = "minecarts/empty.png")]
    minecart_empty: Handle<Image>,
    #[asset(path = "minecarts/metal.png")]
    minecart_metal: Handle<Image>,
    #[asset(path = "minecarts/wood.png")]
    minecart_wood: Handle<Image>,
    #[asset(path = "minecarts/stone.png")]
    minecart_stone: Handle<Image>,

    #[asset(path = "signature_anim/signature1.png")]
    signature_1: Handle<Image>,
    #[asset(path = "signature_anim/signature2.png")]
    signature_2: Handle<Image>,
    #[asset(path = "signature_anim/signature3.png")]
    signature_3: Handle<Image>,
    #[asset(path = "signature_anim/signature4.png")]
    signature_4: Handle<Image>,
    #[asset(path = "signature_anim/signature5.png")]
    signature_5: Handle<Image>,
    #[asset(path = "signature_anim/signature6.png")]
    signature_6: Handle<Image>,
    #[asset(path = "signature_anim/signature7.png")]
    signature_7: Handle<Image>,
    #[asset(path = "signature_anim/signature8.png")]
    signature_8: Handle<Image>,
    #[asset(path = "signature_anim/signature9.png")]
    signature_9: Handle<Image>,
    #[asset(path = "signature_anim/signature10.png")]
    signature_10: Handle<Image>,
    #[asset(path = "signature_anim/signature11.png")]
    signature_11: Handle<Image>,
    #[asset(path = "signature_anim/signature12.png")]
    signature_12: Handle<Image>,
    #[asset(path = "signature_anim/signature13.png")]
    signature_13: Handle<Image>,

    #[asset(path = "bird_plane_anim/bird_plane_away0001.png")]
    bird_plane_away_1: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0002.png")]
    bird_plane_away_2: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0003.png")]
    bird_plane_away_3: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0004.png")]
    bird_plane_away_4: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0005.png")]
    bird_plane_away_5: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0006.png")]
    bird_plane_away_6: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0007.png")]
    bird_plane_away_7: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0008.png")]
    bird_plane_away_8: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0009.png")]
    bird_plane_away_9: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0010.png")]
    bird_plane_away_10: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0011.png")]
    bird_plane_away_11: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0012.png")]
    bird_plane_away_12: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0013.png")]
    bird_plane_away_13: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0014.png")]
    bird_plane_away_14: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0015.png")]
    bird_plane_away_15: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0016.png")]
    bird_plane_away_16: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0017.png")]
    bird_plane_away_17: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0018.png")]
    bird_plane_away_18: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0019.png")]
    bird_plane_away_19: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0020.png")]
    bird_plane_away_20: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0021.png")]
    bird_plane_away_21: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0022.png")]
    bird_plane_away_22: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0023.png")]
    bird_plane_away_23: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0024.png")]
    bird_plane_away_24: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0025.png")]
    bird_plane_away_25: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0026.png")]
    bird_plane_away_26: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0027.png")]
    bird_plane_away_27: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0028.png")]
    bird_plane_away_28: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0029.png")]
    bird_plane_away_29: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0030.png")]
    bird_plane_away_30: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0031.png")]
    bird_plane_away_31: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0032.png")]
    bird_plane_away_32: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0033.png")]
    bird_plane_away_33: Handle<Image>,
    #[asset(path = "bird_plane_anim/bird_plane_away0034.png")]
    bird_plane_away_34: Handle<Image>,

    #[asset(path = "alchemy_lab_anim/alchemy_lab0001.png")]
    alchemy_lab_1: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0002.png")]
    alchemy_lab_2: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0003.png")]
    alchemy_lab_3: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0004.png")]
    alchemy_lab_4: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0005.png")]
    alchemy_lab_5: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0006.png")]
    alchemy_lab_6: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0007.png")]
    alchemy_lab_7: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0008.png")]
    alchemy_lab_8: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0009.png")]
    alchemy_lab_9: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0010.png")]
    alchemy_lab_10: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0011.png")]
    alchemy_lab_11: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0012.png")]
    alchemy_lab_12: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0013.png")]
    alchemy_lab_13: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0014.png")]
    alchemy_lab_14: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0015.png")]
    alchemy_lab_15: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0016.png")]
    alchemy_lab_16: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0017.png")]
    alchemy_lab_17: Handle<Image>,
    #[asset(path = "alchemy_lab_anim/alchemy_lab0018.png")]
    alchemy_lab_18: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
struct ConfigurationAssets {
    #[asset(path = "configuration/synergies.synergies.ron")]
    synergies: Handle<Synergies>,
}

#[derive(AssetCollection, Resource)]
struct FontAssets {
    #[asset(path = "fonts/OldLondon.ttf")]
    town_title_font: Handle<Font>,
    #[asset(path = "fonts/Arvo-Regular.ttf")]
    default_font: Handle<Font>,
}

#[derive(Component)]
struct MainGameObject;

fn reset_states(
    mut in_game_state: ResMut<NextState<InGameState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut menu_state: ResMut<NextState<InMenu>>,
) {
    in_game_state.set(InGameState::Running);
    game_state.set(GameState::MainMenu);
    menu_state.set(InMenu::None);
}

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    mode: WindowMode::Fullscreen(
                        MonitorSelection::Primary,
                        VideoModeSelection::Current,
                    ),

                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..Default::default()
            }),
        RonAssetPlugin::<Synergies>::new(&["synergies.ron"]),
    ))
    .add_plugins((
        train_plugin::train_plugin,
        camera_plugin::camera_plugin,
        world_plugin::world_plugin,
        control_panel_plugin::control_panel_plugin,
        build_plugin::build_plugin,
        main_menu::main_menu_plugin,
        resources_plugin::resources_plugin,
        #[cfg(not(target_family = "wasm"))]
        particles_plugin::particles_plugin,
        animations::animations_plugin,
    ))
    .init_state::<InGameState>()
    .init_state::<GameState>()
    .init_state::<InMenu>()
    .add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::InGame)
            .load_collection::<ImageAssets>()
            .load_collection::<FontAssets>()
            .load_collection::<ConfigurationAssets>(),
    )
    .add_systems(OnEnter(GameState::MainMenu), reset_states);
    #[cfg(debug_assertions)]
    app.add_plugins(debug_plugin::debug_plugin);
    app.run();
}
