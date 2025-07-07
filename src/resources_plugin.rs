use bevy::prelude::*;

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Resource)]
pub struct Food(pub f32);

pub fn resources_plugin(app: &mut App) {
    app.insert_resource(Money(0.)).insert_resource(Food(0.));
}
