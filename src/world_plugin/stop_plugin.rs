use std::f32::consts::PI;

use bevy::{ecs::name, prelude::*};

use crate::{
    FontAssets, GameState, ImageAssets, InGameState,
    control_panel_plugin::AdvanceBlocker,
    resources_plugin::Item,
    train_plugin::TrainState,
    ui_state::InMenu,
    world_plugin::{self, NextStop},
};

use super::CurrentStop;
pub fn stop_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_stop_menu)
        .insert_resource(ActiveContracts(Vec::new()))
        .insert_resource(FadeTime { time: 0. })
        .add_systems(
            Update,
            (
                show_stop_menu.run_if(resource_changed::<CurrentStop>),
                hide_stop_menu.run_if(
                    in_state(GameState::InGame)
                        .and(in_state(InGameState::Running))
                        .and(in_state(InMenu::StopMenu)),
                ),
                fade_title_text
                    .run_if(in_state(GameState::InGame).and(in_state(InGameState::Running))),
            ),
        )
        .add_systems(OnEnter(TrainState::Arriving), spawn_town_arrival_text);
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

#[derive(Component)]
struct FadeTitleText;

#[derive(Resource)]
struct FadeTime {
    time: f32,
}

fn spawn_town_arrival_text(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    next_stop: Res<world_plugin::NextStop>,
    mut fade_time: ResMut<FadeTime>,
) {
    let town_name: String = next_stop.name.to_string();
    if town_name == "Goblin Ambush" {
        return;
    }
    println!("arriving at town: {}", town_name);

    commands.spawn((
        Text::new("Welcome To ".to_string() + &town_name),
        TextFont {
            font: font_assets.town_title_font.clone().into(),
            font_size: 90.0,
            ..Default::default()
        },
        Node {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            bottom: Val::Vh(2.0),
            ..default()
        },
        FadeTitleText,
    ));
    fade_time.time = 5.;
}

fn fade_title_text(
    mut commands: Commands,
    mut fade_time: ResMut<FadeTime>,
    time: Res<Time>,
    mut text_colors: Query<&mut TextColor, With<FadeTitleText>>,
    mut entities: Query<Entity, With<FadeTitleText>>,
) {
    if fade_time.time > 0. {
        fade_time.time -= time.delta_secs();
        for mut text_color in &mut text_colors {
            text_color.0 =
                Color::linear_rgba(1.0, 1.0, 1.0, ops::sin((PI / 5.) * fade_time.time).max(0.));
        }
    } else {
        for entity in &mut entities {
            commands.entity(entity).despawn();
        }
    }
}

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
