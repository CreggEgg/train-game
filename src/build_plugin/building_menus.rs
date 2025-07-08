use bevy::prelude::*;

use crate::GameState;

pub fn building_menus_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_building_menu);
}

fn spawn_building_menu(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Vw(60.0),
            height: Val::Vh(60.0),
            margin: UiRect::AUTO,
            ..Default::default()
        },
        Visibility::Hidden,
        BackgroundColor(Color::BLACK),
        children![(
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
            children![(Text::new("X"), TextColor(Color::BLACK))]
        )],
    ));
}
