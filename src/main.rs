#![allow(clippy::type_complexity, clippy::too_many_arguments)]
use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
};
use ui_state::InMenu;

mod build_plugin;
mod camera_plugin;
mod control_panel_plugin;
mod debug_plugin;
mod goblins;
mod main_menu;
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
    #[asset(path = "stop_bg.png")]
    stop_bg: Handle<Image>,
    #[asset(path = "stop_fg.png")]
    stop_fg: Handle<Image>,
    #[asset(path = "goblinstop_bg.png")]
    goblin_stop_bg: Handle<Image>,
    #[asset(path = "goblinstop_fg.png")]
    goblin_stop_fg: Handle<Image>,
    #[asset(path = "farm.png")]
    farm: Handle<Image>,
    #[asset(path = "rail.png")]
    rail: Handle<Image>,
    #[asset(path = "housing.png")]
    housing: Handle<Image>,
    #[asset(path = "Contract.png")]
    contract: Handle<Image>,
    #[asset(path = "BoothCard.png")]
    booth_card: Handle<Image>,
    #[asset(path = "DebugBuilding.png")]
    debug_building: Handle<Image>,
    #[asset(path = "Ground.png")]
    ground: Handle<Image>,
    #[asset(path = "map_pin.png")]
    map_pin: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
struct FontAssets {
    #[asset(path = "fonts/OldLondon.ttf")]
    town_title_font: Handle<Font>,
    #[asset(path = "fonts/Arvo-Regular.ttf")]
    default_font: Handle<Font>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..Default::default()
            }),
    )
    .add_plugins((
        train_plugin::train_plugin,
        camera_plugin::camera_plugin,
        world_plugin::world_plugin,
        control_panel_plugin::control_panel_plugin,
        build_plugin::build_plugin,
        main_menu::main_menu_plugin,
        resources_plugin::resources_plugin,
    ))
    .init_state::<InGameState>()
    .init_state::<GameState>()
    .init_state::<InMenu>()
    .add_loading_state(
        LoadingState::new(GameState::Loading)
            .continue_to_state(GameState::InGame)
            .load_collection::<ImageAssets>()
            .load_collection::<FontAssets>(),
    );
    #[cfg(debug_assertions)]
    app.add_plugins(debug_plugin::debug_plugin);
    app.run();
}
