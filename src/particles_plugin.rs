use bevy::prelude::*;
use bevy_hanabi::prelude::*;

use crate::{GameState, ImageAssets, MainGameObject, train_plugin::TrainState};

#[derive(Resource)]
pub struct Particles {
    sparking: Handle<EffectAsset>,
    steam: Handle<EffectAsset>,
}

pub fn particles_plugin(app: &mut App) {
    app.add_plugins(HanabiPlugin)
        .add_systems(OnEnter(GameState::Loading), setup_particles)
        .add_systems(OnEnter(GameState::InGame), play_steam_asset)
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
    let writer = ExprWriter::new();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(2.).expr(),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel = SetAttributeModifier::new(
        Attribute::VELOCITY,
        writer
            .lit(vec3(150.0, 0.0, 0.0))
            .add(
                (writer
                    .rand(ScalarType::Float)
                    .mul(writer.lit(vec3(0.0, 75.0, 0.0)))),
            )
            .expr(),
    );

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = writer.lit(0.5); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime.expr());

    // Every frame, add a gravity-like acceleration downward
    let accel = writer.lit(Vec3::new(5., 0., 0.));
    let update_accel = AccelModifier::new(accel.expr());

    let module = writer.finish();

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        SpawnerSettings::rate(50.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("Sparking")
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
    let sparking_handle = effects.add(effect);

    //===========================================================================
    //===========================================================================
    //===========================================================================
    //===========================================================================
    //===========================================================================
    //===========================================================================
    //===========================================================================
    //===========================================================================
    //
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.25, 0.25, 0.25, 0.));
    gradient.add_key(0.1, Vec4::new(0.25, 0.25, 0.25, 1.));
    gradient.add_key(1.0, Vec4::new(0.0, 0.0, 0.0, 0.));
    let writer = ExprWriter::new();

    // On spawn, randomly initialize the position of the particle
    // to be over the surface of a sphere of radius 2 units.
    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(10.).expr(),
        dimension: ShapeDimension::Surface,
    };

    // Also initialize a radial initial velocity to 6 units/sec
    // away from the (same) sphere center.
    let init_vel =
        SetAttributeModifier::new(Attribute::VELOCITY, writer.lit(vec3(5.0, 50.0, 0.0)).expr());

    // Initialize the total lifetime of the particle, that is
    // the time for which it's simulated and rendered. This modifier
    // is almost always required, otherwise the particles won't show.
    let lifetime = writer.lit(5.0); // literal value "10.0"
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime.expr());

    // Every frame, add a gravity-like acceleration downward
    let accel = writer.lit(Vec3::new(5., 0., 0.));
    let update_accel = AccelModifier::new(accel.expr());

    let rotation = (writer.rand(ScalarType::Float) * writer.lit(std::f32::consts::TAU)).expr();
    let init_rotation = SetAttributeModifier::new(Attribute::F32_0, rotation);

    let rotation_attr = writer.attr(Attribute::F32_0).expr();
    let texture_slot = writer.lit(0u32).expr();

    let mut module = writer.finish();
    module.add_texture_slot("color");

    // Create the effect asset
    let effect = EffectAsset::new(
        // Maximum number of particles alive at a time
        32768,
        // Spawn at a rate of 5 particles per second
        SpawnerSettings::rate(5.0.into()),
        // Move the expression module into the asset
        module,
    )
    .with_name("Steam")
    .init(init_pos)
    .init(init_vel)
    .init(init_lifetime)
    .init(init_rotation)
    .update(update_accel)
    .render(ParticleTextureModifier {
        texture_slot,
        sample_mapping: ImageSampleMapping::ModulateOpacityFromR,
    }).render(ColorOverLifetimeModifier {gradient, blend: ColorBlendMode::Modulate, mask: ColorBlendMask::all() })
    .render(SizeOverLifetimeModifier {
                gradient: Gradient::constant([50.0; 3].into()),
                screen_space_size: false,
            })
    /* .render(OrientModifier {
        mode: OrientMode::FaceCameraPosition,
        rotation: Some(rotation_attr),
    }) */;

    // Insert into the asset system
    let steam_handle = effects.add(effect);

    commands.insert_resource(Particles {
        sparking: sparking_handle,
        steam: steam_handle,
    });
}

fn play_spark_assset(mut commands: Commands, particles: Res<Particles>) {
    commands.spawn((
        MainGameObject,
        Spark,
        ParticleEffect::new(particles.sparking.clone()),
        Transform::from_translation(Vec3::new(40.0, -60.0, 0.0)),
    ));
}

fn play_steam_asset(
    mut commands: Commands,
    particles: Res<Particles>,
    image_assets: Res<ImageAssets>,
) {
    commands.spawn((
        MainGameObject,
        ParticleEffect::new(particles.steam.clone()),
        EffectMaterial {
            images: vec![image_assets.steam_particle.clone()],
        },
        Transform::from_translation(Vec3::new(-60.0, 70.0, 0.0)),
    ));
}
