use bevy::prelude::*;

use crate::{
    GameState, ImageAssets, InGameState, control_panel_plugin::AdvanceBlocker,
    resources_plugin::Item, ui_state::InMenu,
};

use super::CurrentStop;
pub fn stop_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_stop_menu)
        .insert_resource(ActiveContracts(Vec::new()))
        .add_systems(
            Update,
            (
                show_stop_menu.run_if(resource_changed::<CurrentStop>),
                hide_stop_menu.run_if(
                    in_state(GameState::InGame)
                        .and(in_state(InGameState::Running))
                        .and(in_state(InMenu::StopMenu)),
                ),
            ),
        );
}

#[derive(Resource)]
pub struct ActiveContracts(pub Vec<Contract>);

pub struct Contract {
    pub required: (Item, usize),
    pub reward: (Item, usize),
}

#[derive(Component)]
struct StopMenu;
#[derive(Component)]
struct CloseMenuButton;

fn spawn_stop_menu(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let booth_ratio = 149.0 / 99.0;
    let booth_width = 200.0;
    commands
        .spawn((
            Node {
                margin: UiRect::AUTO,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            StopMenu,
            AdvanceBlocker,
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    width: Val::Px(booth_width * 6.),
                    height: Val::Px(booth_width * booth_ratio),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,

                    ..Default::default()
                },))
                .with_children(|parent| {
                    for i in 0..6 {
                        parent.spawn((
                            Node {
                                width: Val::Px(booth_width),
                                height: Val::Px(booth_width * booth_ratio),

                                ..Default::default()
                            },
                            ImageNode::new(image_assets.booth.clone()),
                        ));
                    }
                });
            parent.spawn((
                Node {
                    width: Val::Px(160.0),
                    height: Val::Px(20.0),
                    ..Default::default()
                },
                BackgroundColor(Color::WHITE),
                Button,
                CloseMenuButton,
                children![(Text::new("Close"), TextColor(Color::BLACK))],
            ));
        });
}

fn show_stop_menu(
    current_stop: Res<CurrentStop>,
    mut menu: Query<&mut Visibility, With<StopMenu>>,
    mut menu_state: ResMut<NextState<InMenu>>,
) {
    if current_stop.0.is_some() {
        if let Ok(mut menu) = menu.single_mut() {
            *menu = Visibility::Visible;
            menu_state.set(InMenu::StopMenu);
        }
    }
}
fn hide_stop_menu(
    interaction_query: Query<
        (&Interaction, &CloseMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut menu: Query<&mut Visibility, With<StopMenu>>,
    mut menu_state: ResMut<NextState<InMenu>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            *menu.single_mut().unwrap() = Visibility::Hidden;
            menu_state.set(InMenu::None);
        }
    }
}
