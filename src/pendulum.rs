use std::collections::VecDeque;
use std::f32::consts::PI;

use bevy::gizmos::circles;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::{gizmos::gizmos, prelude::*};
use bevy::color::palettes::{basic::*, css::*, tailwind::*};

// Component per il primo e secondo pendolo
#[derive(Component)]
pub struct Pendulum {
    angle: f32,                 // L'angolo attuale
    angular_velocity: f32,      // Velocità angolare
    angular_acceleration: f32,  // Accelerazione angolare
    length: f32,                // Lunghezza dell'asta
    mass: f32,                  // Massa del pendolo
}

#[derive(Component)]
pub struct PendulumTrace {
    positions: VecDeque<Vec3>,       // Memorizza le posizioni delle masse
    max_points: usize,          // Numero massimo di punti da mantenere
}

impl Pendulum {
    pub fn new(angle: f32, length: f32, mass: f32) -> Self {
        Pendulum {
            angle,
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            length,
            mass,
        }
    }
}

pub fn setup_pendulum(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Pendulum::new(PI / 4.0, 100.0, 1.0), 
        PendulumTrace {
            positions: VecDeque::new(),
            max_points: 100,  // Traccia lunga al massimo 500 punti
        },
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 25. } )),
            material: materials.add(
                Color::hsl(360. * 1 as f32 / 10 as f32, 0.95, 0.7)
            ),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                0.0,
                0.0,
                0.0,
            ),
            ..default()
        }
    ));
    println!("Pendulum setup completed!");
}

// Using Euler's method
pub fn update_pendulum_system(
    time: Res<Time>,
    mut query: Query<&mut Pendulum>,
) {
    let dt = time.delta_seconds() * 4.0; // Speed up the simulation by a factor of 2


    for mut pendulum in query.iter_mut() {
        // Aggiornare accelerazione, velocità e posizione
        let g = 9.81; // Gravità
        let num1 = -g * (2.0 * pendulum.mass) * pendulum.angle.sin();
        let num2 = -pendulum.mass * g * (pendulum.angle - PI).sin();
        let denom = pendulum.length;
        pendulum.angular_acceleration = (num1 + num2) / denom;

        // Aggiornare velocità e angolo
        pendulum.angular_velocity += pendulum.angular_acceleration * dt;
        pendulum.angle += pendulum.angular_velocity * dt;
    }
}

pub fn draw_pendulum_system(
    mut gizmos: Gizmos,
    mut query: Query<(&Pendulum, &mut Transform, &mut PendulumTrace)>,
) {
    for (pendulum, mut cirlce_mesh, mut trace) in query.iter_mut() {
        let x = pendulum.length * pendulum.angle.sin() - 100.;
        let y = -pendulum.length * pendulum.angle.cos() + 100.;

        // Disegna la linea tra il punto di origine e il primo pendolo
        gizmos.line(Vec3::new(-100.0, 100.0, 0.0), Vec3::new(x, y, 0.0), Color::WHITE);

        // Update the mesh transform
        cirlce_mesh.translation.x = x;
        cirlce_mesh.translation.y = y;
        
        trace.positions.push_back(Vec3::new(x, y, 0.0));
        if trace.positions.len() >= trace.max_points {
            trace.positions.pop_front();
        }
    }
}

pub fn draw_pendulum_trace(
    mut gizmos: Gizmos,
    query: Query<&PendulumTrace>,
) {
    for trace in query.iter() {
        for i in 1..trace.positions.len() {
            gizmos.line(trace.positions[i - 1], trace.positions[i], Color::hsl(264., 1., 0.7));
        }
    }
}

// ========= DOUBLE PENDULUM =========

#[derive(Component)]
pub struct DoublePendulum {
    theta1: f32,                 // Angle of the first pendulum
    omega1: f32,                 // Angular velocity of the first pendulum
    alpha1: f32,                 // Angular acceleration of the first pendulum
    length1: f32,                // Length of the first rod
    mass1: f32,                  // Mass of the first pendulum

    theta2: f32,                 // Angle of the second pendulum
    omega2: f32,                 // Angular velocity of the second pendulum
    alpha2: f32,                 // Angular acceleration of the second pendulum
    length2: f32,                // Length of the second rod
    mass2: f32,                  // Mass of the second pendulum
}

impl DoublePendulum {
    pub fn new(
        theta1: f32, length1: f32, mass1: f32, 
        theta2: f32, length2: f32, mass2: f32
    ) -> Self {
        DoublePendulum {
            theta1,
            omega1: 0.0,
            alpha1: 0.0,
            length1,
            mass1,
            theta2,
            omega2: 0.0,
            alpha2: 0.0,
            length2,
            mass2,
        }
    }
}

#[derive(Debug, Component)]
pub struct PendulumMass1;

#[derive(Debug, Component)]
pub struct PendulumMass2;

pub fn setup_double_pendulum(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn the double pendulum with both pendulum arms
    commands.spawn((
        DoublePendulum::new(
            PI / 2.0, // Initial angle for the first pendulum
            100.0,    // Length of the first pendulum's rod
            1000.0,      // Mass of the first pendulum
            PI / 4.0, // Initial angle for the second pendulum
            200.0,     // Length of the second pendulum's rod
            200.5,      // Mass of the second pendulum
        ),
        PendulumTrace {
            positions: VecDeque::new(),
            max_points: 1100,  // Traccia lunga al massimo 100 punti
        },
    ));

    // First pendulum visual representation (attached to a fixed point)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 25. } )),
            material: materials.add(
                Color::hsl(0.9, 0.45, 0.8)
            ),
            transform: Transform::from_xyz(0.0, 0.0, 0.0), // Initial position at the origin
            ..default()
        },
        PendulumMass1,
    ));

    // Second pendulum visual representation (attached to the end of the first pendulum)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 25. } )),
            material: materials.add(
                Color::hsl(0.3, 0.75, 0.7)
            ),
            transform: Transform::from_xyz(
                100.0 * (PI / 4.0).sin(), // Initial X position based on the first pendulum
                -100.0 * (PI / 4.0).cos(), // Initial Y position based on the first pendulum
                0.0,
            ), // Start at the end of the first pendulum
            ..default()
        },
        PendulumMass2
    ));

    println!("Double pendulum setup completed!");
}


pub fn draw_double_pendulum_system(
    mut commands: Commands,
    mut query: Query<&DoublePendulum>,
    mut mesh1: Query<&mut Transform, (With<PendulumMass1>, Without<PendulumMass2>)>,
    mut mesh2: Query<&mut Transform, (With<PendulumMass2>, Without<PendulumMass1>)>,
    mut gizmos: Gizmos,
    mut trace: Query<&mut PendulumTrace, With<DoublePendulum>>
) {
    for pendulum in query.iter_mut() {
        // Calculate the first pendulum position (relative to the origin)
        let x1 = pendulum.length1 * pendulum.theta1.sin();
        let y1 = -pendulum.length1 * pendulum.theta1.cos(); // Y-axis is inverted in screen coordinates

        // Calculate the second pendulum position (relative to the end of the first pendulum)
        let x2 = x1 + pendulum.length2 * pendulum.theta2.sin();
        let y2 = y1 - pendulum.length2 * pendulum.theta2.cos();

        // Draw the arms between the pendulum points
        // Line from the fixed point (origin) to the first pendulum mass
        gizmos.line(Vec3::ZERO, Vec3::new(x1, y1, 0.0), Color::WHITE);

        // Line from the first pendulum mass to the second pendulum mass
        gizmos.line(Vec3::new(x1, y1, 0.0), Vec3::new(x2, y2, 0.0), Color::WHITE);

        // Update the positions of the two pendulum masses (the two circles)
        // transforms.translation = Vec3::new(x1, y1, 0.0); // First pendulum mass position
        // transforms.translation = Vec3::new(x2, y2, 0.0); // Second pendulum mass position
        let mut transform1 = mesh1.single_mut();
        transform1.translation = Vec3::new(x1, y1, 0.0);
        let mut transform2 = mesh2.single_mut();
        transform2.translation = Vec3::new(x2, y2, 0.0);

        let mut trace = trace.single_mut();
        trace.positions.push_back(Vec3::new(x2, y2, 0.0));
        if trace.positions.len() >= trace.max_points {
            trace.positions.pop_front();
        }
    }
}

// // Runge-Kutta (RK4) - Runge-Kutta di quarto ordine
fn pendulum_accelerations(
    theta1: f32, // θ è l'angolo
    theta2: f32,
    omega1: f32, // 𝜔 la velocità angolare
    omega2: f32,
    length1: f32,
    length2: f32,
    mass1: f32,
    mass2: f32,
    g: f32,
) -> (f32, f32) {
    let delta_theta = theta1 - theta2;
    let denom1 = (2.0 * mass1 + mass2 - mass2 * (2.0 * delta_theta).cos()) * length1;
    let denom2 = (length2 / length1) * denom1;

    let accel1 = (-g * (2.0 * mass1 + mass2) * theta1.sin()
        - mass2 * g * (theta1 - 2.0 * theta2).sin()
        - 2.0 * delta_theta.sin() * mass2
        * (omega2 * omega2 * length2 + omega1 * omega1 * length1 * delta_theta.cos()))
        / denom1;

    let accel2 = (2.0 * delta_theta.sin()
        * (omega1 * omega1 * length1 * (mass1 + mass2)
        + g * (mass1 + mass2) * theta1.cos()
        + omega2 * omega2 * length2 * mass2 * delta_theta.cos()))
        / denom2;

    (accel1, accel2)
}

fn rk4_step(
    theta1: f32,
    omega1: f32,
    theta2: f32,
    omega2: f32,
    length1: f32,
    length2: f32,
    mass1: f32,
    mass2: f32,
    g: f32,
    dt: f32,
) -> (f32, f32, f32, f32) {
    // k1
    let (accel1_k1, accel2_k1) = pendulum_accelerations(theta1, theta2, omega1, omega2, length1, length2, mass1, mass2, g);
    let theta1_k1 = omega1;
    let theta2_k1 = omega2;
    
    // k2
    let theta1_k2 = omega1 + 0.5 * dt * accel1_k1;
    let theta2_k2 = omega2 + 0.5 * dt * accel2_k1;
    let (accel1_k2, accel2_k2) = pendulum_accelerations(
        theta1 + 0.5 * dt * theta1_k1,
        theta2 + 0.5 * dt * theta2_k1,
        omega1 + 0.5 * dt * accel1_k1,
        omega2 + 0.5 * dt * accel2_k1,
        length1, length2, mass1, mass2, g,
    );

    // k3
    let theta1_k3 = omega1 + 0.5 * dt * accel1_k2;
    let theta2_k3 = omega2 + 0.5 * dt * accel2_k2;
    let (accel1_k3, accel2_k3) = pendulum_accelerations(
        theta1 + 0.5 * dt * theta1_k2,
        theta2 + 0.5 * dt * theta2_k2,
        omega1 + 0.5 * dt * accel1_k2,
        omega2 + 0.5 * dt * accel2_k2,
        length1, length2, mass1, mass2, g,
    );

    // k4
    let theta1_k4 = omega1 + dt * accel1_k3;
    let theta2_k4 = omega2 + dt * accel2_k3;
    let (accel1_k4, accel2_k4) = pendulum_accelerations(
        theta1 + dt * theta1_k3,
        theta2 + dt * theta2_k3,
        omega1 + dt * accel1_k3,
        omega2 + dt * accel2_k3,
        length1, length2, mass1, mass2, g,
    );

    // Aggiornare i valori finali usando la media pesata dei k
    let theta1_next = theta1 + (dt / 6.0) * (theta1_k1 + 2.0 * theta1_k2 + 2.0 * theta1_k3 + theta1_k4);
    let omega1_next = omega1 + (dt / 6.0) * (accel1_k1 + 2.0 * accel1_k2 + 2.0 * accel1_k3 + accel1_k4);
    let theta2_next = theta2 + (dt / 6.0) * (theta2_k1 + 2.0 * theta2_k2 + 2.0 * theta2_k3 + theta2_k4);
    let omega2_next = omega2 + (dt / 6.0) * (accel2_k1 + 2.0 * accel2_k2 + 2.0 * accel2_k3 + accel2_k4);

    (theta1_next, omega1_next, theta2_next, omega2_next)
}

pub fn update_pendulum_system_rk4(
    time: Res<Time>,
    mut query: Query<&mut DoublePendulum>,
) {
    let dt = time.delta_seconds() * 5.;
    let g = 9.81; // gravità

    for mut pendulum in query.iter_mut() {
        let (theta1, omega1, theta2, omega2) = rk4_step(
            pendulum.theta1,
            pendulum.omega1,
            pendulum.theta2,
            pendulum.omega2,
            pendulum.length1,
            pendulum.length2,
            pendulum.mass1,
            pendulum.mass2,
            g,
            dt,
        );

        // Aggiornare i valori dei pendoli
        pendulum.theta1 = theta1;
        pendulum.omega1 = omega1;
        pendulum.theta2 = theta2;
        pendulum.omega2 = omega2;
    }
}

