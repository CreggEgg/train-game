use bevy::prelude::*;

use crate::{
    GameState, InGameState,
    build_plugin::BuildState,
    train_plugin::{AdvanceEvent, Train},
    world_plugin::NextStop,
};

pub fn control_panel_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_control_panel)
        .add_systems(
            Update,
            (advance_button, build_button)
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(InGameState::Running)),
        )
        .add_systems(
            Update,
            update_next_town_display
                .run_if(resource_changed::<NextStop>.and(in_state(GameState::InGame))),
        )
        .add_systems(
            OnEnter(BuildState::Building),
            |mut build_button: Query<(&mut BuildButton, &Children)>,
             mut text_query: Query<&mut Text>| {
                println!("hi");
                let (mut build_button, children) = build_button.single_mut().unwrap();
                *build_button = BuildButton::EndBuilding;
                let mut text = text_query.get_mut(children[0]).unwrap();
                **text = "Stop Building".to_string();
            },
        )
        .add_systems(
            OnExit(BuildState::Building),
            |mut build_button: Query<(&mut BuildButton, &Children)>,
             mut text_query: Query<&mut Text>| {
                let (mut build_button, children) = build_button.single_mut().unwrap();
                *build_button = BuildButton::StartBuilding;
                let mut text = text_query.get_mut(children[0]).unwrap();
                **text = "Build".to_string();
            },
        );
}

#[derive(Component)]
struct AdvanceButton;
#[derive(Component)]
enum BuildButton {
    StartBuilding,
    EndBuilding,
}

fn spawn_control_panel(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(5.0),
            display: Display::Flex,
            padding: UiRect::all(Val::Px(4.0)),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0., 0., 0.)),
        children![
            (
                Node {
                    width: Val::Px(160.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // margin: UiRect::AUTO.with_left(Val::Px(0.)).with_right(Val::Px(0.)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                BorderRadius::MAX,
                AdvanceButton,
                Button,
                children![Text::new("Advance")]
            ),
            (NextTownDisplay, Text::new("Next town: {}")),
            (
                Node {
                    width: Val::Px(180.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // margin: UiRect::AUTO.with_left(Val::Px(0.)).with_right(Val::Px(0.)),
                    ..Default::default()
                },
                BackgroundColor(Color::srgb(0.5, 0.5, 0.5)),
                BuildButton::StartBuilding,
                BorderRadius::MAX,
                Button,
                children![Text::new("Build")]
            )
        ],
    ));
}

#[derive(Component)]
struct NextTownDisplay;

#[derive(Component)]
pub struct AdvanceBlocker;

fn advance_button(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<AdvanceButton>),
    >,
    blockers: Query<&AdvanceBlocker>,
    mut ev: EventWriter<AdvanceEvent>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed && blockers.iter().len() == 0 {
            info!("Sending advance event");
            ev.write(AdvanceEvent);
        }
    }
}

fn build_button(
    interaction_query: Query<(&Interaction, &BuildButton), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<BuildState>>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(match button {
                BuildButton::StartBuilding => BuildState::Building,
                BuildButton::EndBuilding => BuildState::NotBuilding,
            });
        }
    }
}

fn update_next_town_display(
    mut next_town_display: Query<&mut Text, With<NextTownDisplay>>,
    next_stop: Res<NextStop>,
) {
    **next_town_display.single_mut().unwrap() = format!("Next town: {}", next_stop.name,);
}
