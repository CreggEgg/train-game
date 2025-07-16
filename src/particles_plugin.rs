use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{GameState, train_plugin::TrainState};

#[derive(Resource)]
pub struct Particles {
    sparking: Handle<EffectAsset>,
}

pub fn particles_plugin(app: &mut App) {
    app.add_plugins(HanabiPlugin)
        .add_systems(OnEnter(GameState::Loading), setup_particles)
        .add_systems(OnEnter(TrainState::Arriving), play_spark_assset)
        .add_systems(
            OnExit(TrainState::Arriving),
            |mut commands: Commands, sparks: Query<Entity, With<Spark>>| {
                for spark in &sparks {
                    commands.entity(spark).despawn();
                }
            },
        );
}

#[derive(Component)]
struct Spark;

fn setup_particles(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.98, 0.72, 0.01, 1.));

    // Create a new expression module
    let mut module = Module::default();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),
        radius: module.lit(2.),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),
        speed: module.lit(6.),
    };

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = module.lit(10.); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Every frame, add a gravity-like acceleration downward
    let accel = module.lit(Vec3::new(5., 0., 0.));
    let update_accel = AccelModifier::new(accel);

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        SpawnerSettings::rate(5.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("MyEffect")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .update(update_accel)
    // Render the particles with a color gradient over their
    // lifetime. This maps the gradient key 0 to the particle spawn
    // time, and the gradient key 1 to the particle death (10s).
    .render(ColorOverLifetimeModifier {
        gradient,
        ..default()
    });

    // Insert into the asset system
    let effect_handle = effects.add(effect);
    commands.insert_resource(Particles {
        sparking: effect_handle,
    });
}

fn play_spark_assset(mut commands: Commands, particles: Res<Particles>) {
    commands.spawn((
        Spark,
        ParticleEffect::new(particles.sparking.clone()),
        Transform::from_translation(Vec3::Y),
    ));
}
