use bevy::prelude::*;

use crate::GameState;

#[derive(Component)]
struct MainMenu;
#[derive(Component)]
struct StartGame;

pub fn main_menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
        .add_systems(Update, start_button.run_if(in_state(GameState::MainMenu)));
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        MainMenu,
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
        children![
            (
                Text::new("THE ENGINE OF TOMORROW"),
                TextFont {
                    font_size: 128.0,
                    ..Default::default()
                },
                Node {
                    margin: UiRect::bottom(Val::Vh(20.0)),
                    ..Default::default()
                }
            ),
            (
                Button,
                StartGame,
                Node {
                    height: Val::Px(40.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                BackgroundColor(Color::WHITE),
                children![(Text::new("Start Game"), TextColor(Color::BLACK))]
            )
        ],
    ));
}

fn cleanup_main_menu(mut commands: Commands, elements: Query<Entity, With<MainMenu>>) {
    for entity in &elements {
        commands.entity(entity).despawn_related::<Children>();
        commands.entity(entity).despawn();
    }
}

fn start_button(
    mut interaction_query: Query<
        (&Interaction, &Children, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<StartGame>),
    >,
    mut state: ResMut<NextState<GameState>>,

    mut text_query: Query<&mut Text>,
) {
    for (interaction, children, mut background_color) in &mut interaction_query {
        if *interaction == Interaction::Hovered {
            background_color.0 = Color::srgb(0.85, 0.85, 0.85);
            let mut text = text_query.get_mut(children[0]).unwrap();
            **text = "CHOO CHOO".to_string();
        }
        if *interaction == Interaction::None {
            background_color.0 = Color::srgb(1., 1., 1.);
            let mut text = text_query.get_mut(children[0]).unwrap();
            **text = "Start Game".to_string();
        }
        if *interaction == Interaction::Pressed {
            state.set(GameState::Loading);
        }
    }
}
