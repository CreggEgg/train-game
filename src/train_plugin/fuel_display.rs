use bevy::prelude::*;

use crate::{GameState, MainGameObject};

pub fn fuel_display_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_fuel_display);
}

fn spawn_fuel_display(mut commands: Commands) {
    commands.spawn((
        MainGameObject,
        Node {
            height: Val::Percent(100.0),
            width: Val::Px(75.0),
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        // BackgroundColor(RED.into()),
        children![Text::new("100 L")],
    ));
}
