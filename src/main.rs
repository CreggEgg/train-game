use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
};

mod build_plugin;
mod camera_plugin;
mod control_panel_plugin;
mod debug_plugin;
mod main_menu;
mod train_plugin;
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
    #[asset(path = "farm.png")]
    farm: Handle<Image>,
    #[asset(path = "rail.png")]
    rail: Handle<Image>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins((
            train_plugin::train_plugin,
            camera_plugin::camera_plugin,
            world_plugin::world_plugin,
            control_panel_plugin::control_panel_plugin,
            build_plugin::build_plugin,
            main_menu::main_menu_plugin,
        ))
        .init_state::<InGameState>()
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::InGame)
                .load_collection::<ImageAssets>(),
        );
    #[cfg(debug_assertions)]
    app.add_plugins(debug_plugin::debug_plugin);
    app.run();
}
