use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

#[derive(Component)]
pub struct Animation(pub Vec<Handle<Image>>, pub usize);

pub fn animations_plugin(app: &mut App) {
    app.add_systems(
        Update,
        animate.run_if(on_timer(Duration::from_secs_f32(1.0 / 24.0))),
    );
}

fn animate(mut animated_objects: Query<(&mut Animation, &mut Sprite)>) {
    for (mut animation, mut sprite) in &mut animated_objects {
        animation.1 += 1;
        animation.1 %= animation.0.len();
        sprite.image = animation.0.get(animation.1).unwrap().clone();
    }
}
