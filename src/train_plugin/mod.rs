use bevy::prelude::*;

use crate::{GameState, ImageAssets};

#[derive(Resource)]
pub struct TrainStats {
    length: usize,
}

pub fn train_plugin(mut app: &mut App) {
    app.insert_resource(TrainStats { length: 1 })
        .add_systems(OnEnter(GameState::InGame), spawn_train);
}

const CAR_SIZE: f32 = 140.0;

#[derive(Component)]
pub struct Locomotive;
#[derive(Component)]
pub struct TrainCar;

fn spawn_train(
    mut commands: Commands,
    image_assets: Res<ImageAssets>,
    train_stats: Res<TrainStats>,
) {
    commands
        .spawn((Visibility::default(), Transform::default()))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_image(image_assets.train_car.clone()),
                Name::new("Locomotive"),
                Locomotive,
            ));
            for i in 0..train_stats.length {
                parent.spawn((
                    Sprite::from_image(image_assets.train_car.clone()),
                    Name::new(format!("Car{i}")),
                    TrainCar,
                    Transform::from_xyz(CAR_SIZE * (i as f32 + 1.), 0., 0.),
                ));
            }
        });
}
