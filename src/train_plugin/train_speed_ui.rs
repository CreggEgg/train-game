use bevy::prelude::*;

use super::Train;

#[derive(Component)]
pub struct SpeedUI;

const SPEED_FONT_SIZE: f32 = 20.0;
const SPEED_FONT_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const SPEED_TEXT_PADDING: Val = Val::Px(10.0);

pub(crate) fn make_ui(mut commands: Commands) {
    commands.spawn((
        Text::new("Speed: "),
        TextFont {
            font_size: SPEED_FONT_SIZE,
            ..default()
        },
        TextColor(SPEED_FONT_COLOR),
        SpeedUI,
        Node {
            position_type: PositionType::Absolute,
            bottom: SPEED_TEXT_PADDING,
            left: SPEED_TEXT_PADDING,
            ..default()
        },
        children![(
            TextSpan::default(),
            TextFont {
                font_size: SPEED_FONT_SIZE,
                ..default()
            },
            TextColor(SPEED_FONT_COLOR),
        )],
    ));
}

pub(crate) fn update_train_speed(
    train: Query<&Train, Changed<Train>>,
    speed_ui: Single<Entity, (With<SpeedUI>, With<Text>)>,
    mut writer: TextUiWriter,
) {
    let train = match train.single() {
        Ok(a) => a,
        Err(_) => return,
    };

    let speed = train.velocity;

    *writer.text(*speed_ui, 1) = format!("{:.2}", speed);
}
