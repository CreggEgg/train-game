use bevy::prelude::*;

use crate::{GameState, ImageAssets};
pub fn stop_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::InGame), spawn_stop_menu);
}

fn spawn_stop_menu(mut commands: Commands, image_assets: Res<ImageAssets>) {
    let booth_ratio = 149.0 / 99.0;
    let booth_width = 200.0;
    commands
        .spawn((Node {
            margin: UiRect::AUTO,
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
}
