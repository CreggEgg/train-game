use bevy::prelude::*;

use crate::{GameState, InGameState, MainGameObject, save_plugin::SaveEvent, ui_state::InMenu};

pub fn pause_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_pause_menu)
        .add_systems(Update, pause_game)
        .add_systems(
            OnEnter(InGameState::Paused),
            |mut pause_menu: Single<&mut Visibility, With<PauseMenu>>| {
                **pause_menu = Visibility::Visible;
            },
        )
        .add_systems(
            OnExit(InGameState::Paused),
            |mut pause_menu: Single<&mut Visibility, With<PauseMenu>>| {
                **pause_menu = Visibility::Hidden;
            },
        );
}

#[derive(Component)]
struct PauseMenu;

fn pause_game(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_pause_state: Res<State<InGameState>>,
    mut next_pause_state: ResMut<NextState<InGameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        let next = match **current_pause_state {
            InGameState::Running => InGameState::Paused,
            InGameState::Paused => InGameState::Running,
        };
        next_pause_state.set(next);
    }
}

fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            Visibility::Hidden,
            PauseMenu,
            MainGameObject,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..Default::default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::WHITE),
                    children![(Text::new("Return to game"), TextColor(Color::BLACK))],
                ))
                .observe(
                    |mut trigger: Trigger<Pointer<Pressed>>,
                     mut next_pause_state: ResMut<NextState<InGameState>>| {
                        next_pause_state.set(InGameState::Running);
                    },
                );
            parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::WHITE),
                    children![(Text::new("Save game"), TextColor(Color::BLACK))],
                ))
                .observe(
                    |mut trigger: Trigger<Pointer<Pressed>>, mut ev: EventWriter<SaveEvent>| {
                        ev.write(SaveEvent);
                    },
                );
            parent.spawn((
                Node {
                    padding: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::WHITE),
                children![(Text::new("Quit to main menu"), TextColor(Color::BLACK))],
            ));
        });
}
