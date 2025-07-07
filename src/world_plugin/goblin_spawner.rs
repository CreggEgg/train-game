use crate::{
    control_panel_plugin::AdvanceBlocker,
    goblins::Goblin,
    train_plugin::{MaxPixelHeightOfTrain, TrainStats},
};
use bevy::prelude::*;

#[derive(Clone)]
pub enum GoblinType {
    Basic,
}

#[derive(Component)]
pub struct GoblinSpawner {
    waves: Vec<Vec<GoblinType>>,
    current_wave: usize,
}

impl GoblinSpawner {
    pub fn new(waves: Vec<Vec<GoblinType>>) -> Self {
        Self {
            waves,
            current_wave: 0,
        }
    }
}

const SPREAD: f32 = 100.0;
const HEIGHT_ABOVE_TRAIN: f32 = 500.0;

pub fn spawn_goblins(
    mut goblin_spawner: Query<(&mut GoblinSpawner, &GlobalTransform, Entity)>,
    goblins: Query<&Goblin>,
    train_height: Res<MaxPixelHeightOfTrain>,
    mut commands: Commands,
    train_stats: Res<TrainStats>,
) {
    let len = goblins.iter().len();

    if len == 0 {
        for mut s in goblin_spawner.iter_mut() {
            if s.0.current_wave >= s.0.waves.len() {
                commands.entity(s.2).despawn();
                continue;
            }

            if s.1.translation().x.abs() > 100.0 {
                continue;
            }

            let t = s.0.waves[s.0.current_wave].len();

            let spread = train_stats.train_size() + SPREAD * 2.0;

            for (i, g) in s.0.waves[s.0.current_wave].iter().enumerate() {
                match g {
                    GoblinType::Basic => {
                        commands.spawn((
                            Goblin,
                            Sprite::from_color(Color::srgb(0.0, 1.0, 1.0), Vec2::ONE),
                            Transform {
                                translation: Vec3 {
                                    x: ((i as f32 + 0.5) / t as f32) * spread - SPREAD,
                                    y: train_height.height + HEIGHT_ABOVE_TRAIN,
                                    z: 0.0,
                                },
                                scale: Vec2::new(10.0, 10.0).extend(1.0),
                                ..default()
                            },
                            AdvanceBlocker,
                        ));
                    }
                }
            }

            s.0.current_wave += 1;

            break;
        }
    }
}
