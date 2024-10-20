use bevy::prelude::*;

use crate::get_mouse_position;

/*
Key Components of a Particle System
Emitter: Responsible for spawning particles over time.
Particle: Represents an individual particle, containing properties like position, velocity, lifetime, and other attributes (color, size, etc.).
Update Systems: Systems that update particle behavior, like movement, size, fading, and removing particles when their lifetime is over.

To add: collision, interaction with the environment, etc.).

*/


pub const PARTICLE_GRAVITY: Vec3 = Vec3::new(0.0, -9.8, 0.0); // Gravity pulling down on the Y axis



// ====== STRUCTS ======

#[derive(Component, Debug)]
pub struct Particle {
    velocity: Vec3,
    lifetime: f32, // How long the particle should live
    size: f32, // Initial size of the particle
}

#[derive(Debug, Component)]
pub struct ParticleEmitter {
    spawn_rate: f32, // Particles per second
    time_since_last_spawn: f32,
}

#[derive(Debug, Resource)]
pub struct ParticleMaterialHandle(pub Handle<Image>);

// ====== METHODS ======

pub fn spawn_emitter(mut commands: Commands) {
    commands.spawn((
        ParticleEmitter {
            spawn_rate: 20.0, // 10 particles per second
            time_since_last_spawn: 0.0,
        },
        Transform::default(), // Position of the emitter
        // Transform {
        //     translation: Vec3 { x: -50., y: 200., z: 1. },
        //     ..default()
        // },
        GlobalTransform::default(),
    ));
}

pub fn move_emitter(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window>, 
    mut emitter: Query<(&mut Transform, &ParticleEmitter)>
) {
    if let Some(Vec2 { x, y }) = get_mouse_position(camera_query, window) {
        for (mut tr, _) in &mut emitter {
            tr.translation = Vec3::new(x, y, 1.);
        }
    } else {
        return;
    };
}

pub fn emitter_system(
    time: Res<Time>,
    mut commands: Commands,
    particle_material: Res<ParticleMaterialHandle>,
    mut query: Query<(&mut ParticleEmitter, &Transform)>
) {
    for (mut emitter, transform) in query.iter_mut() {
        emitter.time_since_last_spawn += time.delta_seconds();

        let particles_to_spawn = (emitter.time_since_last_spawn * emitter.spawn_rate).floor() as i32;
        emitter.time_since_last_spawn %= 1.0 / emitter.spawn_rate;

        for _ in 0..particles_to_spawn {
            commands.spawn((
                Particle {
                    velocity: Vec3::new(
                        rand::random::<f32>() * 2.0 - 1.0,
                        rand::random::<f32>() * 2.0 - 1.0,
                        0.0,
                    ),
                    lifetime: 4.0,
                    size: rand::random::<f32>() * 5.0 + 1.0, // Random initial size between 5 and 15
                },
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE, // Use white to keep the original texture color
                        custom_size: Some(Vec2::splat(10.0)), // Set the size
                        ..Default::default()
                    },
                    texture: particle_material.0.clone(), // Use the Handle<Image> here
                    transform: Transform {
                        translation: transform.translation,
                        scale: Vec3::splat(1.), // Adjust scale
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ));
        }
    }
}


pub fn particle_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Particle)>
) {
    for (mut transform, particle) in query.iter_mut() {
        transform.translation += particle.velocity * time.delta_seconds();
    }
}


pub fn particle_lifetime_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Particle)>
) {
    for (entity, mut particle) in query.iter_mut() {
        particle.lifetime -= time.delta_seconds();
        if particle.lifetime <= 0.0 {
            commands.entity(entity).despawn(); // Remove the particle when its lifetime is over
        }
    }
}

pub fn particle_fade_system(
    time: Res<Time>,
    mut query: Query<(&mut Sprite, &Particle)>
) {
    for (mut sprite, particle) in query.iter_mut() {
        let alpha = particle.lifetime / 4.0; // Assuming the original lifetime was 2 seconds
        sprite.color.set_alpha(alpha); // Set alpha based on remaining lifetime
    }
}

pub fn particle_size_scaling_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Particle)>
) {
    for (mut transform, particle) in query.iter_mut() {
        // Calculate the scale factor based on the remaining lifetime
        let scale_factor = particle.lifetime / 4.0; // Assuming original lifetime is 2 seconds
        let new_size = particle.size * scale_factor;

        // Apply the new size to the particle's transform
        transform.scale = Vec3::splat(new_size);
    }
}

pub fn particle_gravity_system(
    time: Res<Time>,
    mut query: Query<&mut Particle>
) {
    for mut particle in query.iter_mut() {
        // Apply gravity to the particle's velocity
        particle.velocity += PARTICLE_GRAVITY * time.delta_seconds();
    }
}
