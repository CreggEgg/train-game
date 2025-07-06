use bevy::prelude::*;

use crate::{GameState, InGameState, train_plugin::AdvanceEvent};

pub fn control_panel_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_control_panel);
    app.add_systems(
        Update,
        advance_button
            .run_if(in_state(GameState::InGame))
            .run_if(in_state(InGameState::Running)),
    );
}

#[derive(Component)]
struct AdvanceButton;

fn spawn_control_panel(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(5.0),
            display: Display::Grid,
            padding: UiRect::all(Val::Px(4.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::srgb(0., 0., 0.)),
        children![(
            Node {
                width: Val::Px(160.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // margin: UiRect::AUTO.with_left(Val::Px(0.)).with_right(Val::Px(0.)),
                ..Default::default()
            },
            BackgroundColor(Color::srgb(0., 1., 0.)),
            AdvanceButton,
            Button,
            children![Text::new("Advance")]
        )],
    ));
}

fn advance_button(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<AdvanceButton>),
    >,
    mut ev: EventWriter<AdvanceEvent>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            info!("Sending advance event");
            ev.write(AdvanceEvent);
        }
    }
}
