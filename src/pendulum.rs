// use std::f32::consts::PI;

// use bevy::{gizmos::gizmos, prelude::*};

// // Component per il primo e secondo pendolo
// #[derive(Component)]
// pub struct Pendulum {
//     angle: f32,                 // L'angolo attuale
//     angular_velocity: f32,      // Velocità angolare
//     angular_acceleration: f32,  // Accelerazione angolare
//     length: f32,                // Lunghezza dell'asta
//     mass: f32,                  // Massa del pendolo
// }

// #[derive(Component)]
// struct PendulumTrace {
//     positions: Vec<Vec3>,       // Memorizza le posizioni delle masse
//     max_points: usize,          // Numero massimo di punti da mantenere
// }

// impl Pendulum {
//     pub fn new(angle: f32, length: f32, mass: f32) -> Self {
//         Pendulum {
//             angle,
//             angular_velocity: 0.0,
//             angular_acceleration: 0.0,
//             length,
//             mass,
//         }
//     }
// }

// pub fn setup_pendulum(mut commands: Commands) {
//     commands.spawn((
//         Pendulum::new(PI / 4.0, 100.0, 1.0), 
//         PendulumTrace {
//             positions: Vec::new(),
//             max_points: 500,  // Traccia lunga al massimo 500 punti
//         },
//     )); // Primo pendolo
//     commands.spawn((
//         Pendulum::new(PI / 4.0, 100.0, 1.0),
//         PendulumTrace {
//             positions: Vec::new(),
//             max_points: 500,  // Traccia lunga al massimo 500 punti
//         },
//     )); // Secondo pendolo
// }

// // Using Euler's method
// pub fn update_pendulum_system(
//     time: Res<Time>,
//     mut query: Query<&mut Pendulum>,
// ) {
//     let dt = time.delta_seconds();

//     for mut pendulum in query.iter_mut() {
//         // Aggiornare accelerazione, velocità e posizione
//         let g = 9.81; // Gravità
//         let num1 = -g * (2.0 * pendulum.mass) * pendulum.angle.sin();
//         let num2 = -pendulum.mass * g * (pendulum.angle - PI).sin();
//         let denom = pendulum.length;
//         pendulum.angular_acceleration = (num1 + num2) / denom;

//         // Aggiornare velocità e angolo
//         pendulum.angular_velocity += pendulum.angular_acceleration * dt;
//         pendulum.angle += pendulum.angular_velocity * dt;
//     }
// }

// fn draw_pendulum_system(
//     mut gizmos: Gizmos,
//     query: Query<&Pendulum>,
// ) {
//     for pendulum in query.iter() {
//         let x1 = pendulum.length1 * pendulum.theta1.sin();
//         let y1 = -pendulum.length1 * pendulum.theta1.cos();

//         let x2 = x1 + pendulum.length2 * pendulum.theta2.sin();
//         let y2 = y1 - pendulum.length2 * pendulum.theta2.cos();

//         // Disegna la linea tra il punto di origine e il primo pendolo
//         gizmos.line(Vec3::new(0.0, 0.0, 0.0), Vec3::new(x1, y1, 0.0), Color::WHITE);

//         // Disegna la linea tra il primo e il secondo pendolo
//         gizmos.line(Vec3::new(x1, y1, 0.0), Vec3::new(x2, y2, 0.0), Color::WHITE);

//         // Disegna le masse (possono essere cerchi o piccole sfere)
//         gizmos.sphere(
//             Vec3::new(x1, y1, 0.0), 
//             Quat::from(Vec4::new(0., 0., 0., 0.)), 
//             5.0, 
//             Color::RED
//         );
//         gizmos.sphere(
//             Vec3::new(x2, y2, 0.0), 
//             Quat::from(Vec4::new(0., 0., 0., 0.)), 
//             5.0, 
//             Color::RED
//         );
//     }
// }

// fn draw_pendulum_trace(
//     mut gizmos: Gizmos,
//     query: Query<&PendulumTrace>,
// ) {
//     for trace in query.iter() {
//         for i in 1..trace.positions.len() {
//             gizmos.line(trace.positions[i - 1], trace.positions[i], Color::GREEN);
//         }
//     }
// }

// // Runge-Kutta (RK4) - Runge-Kutta di quarto ordine
// pub fn pendulum_accelerations(
//     theta1: f32, // θ è l'angolo
//     theta2: f32,
//     omega1: f32, // 𝜔 la velocità angolare
//     omega2: f32,
//     length1: f32,
//     length2: f32,
//     mass1: f32,
//     mass2: f32,
//     g: f32,
// ) -> (f32, f32) {
//     let delta_theta = theta1 - theta2;
//     let denom1 = (2.0 * mass1 + mass2 - mass2 * (2.0 * delta_theta).cos()) * length1;
//     let denom2 = (length2 / length1) * denom1;

//     let accel1 = (-g * (2.0 * mass1 + mass2) * theta1.sin()
//         - mass2 * g * (theta1 - 2.0 * theta2).sin()
//         - 2.0 * delta_theta.sin() * mass2
//         * (omega2 * omega2 * length2 + omega1 * omega1 * length1 * delta_theta.cos()))
//         / denom1;

//     let accel2 = (2.0 * delta_theta.sin()
//         * (omega1 * omega1 * length1 * (mass1 + mass2)
//         + g * (mass1 + mass2) * theta1.cos()
//         + omega2 * omega2 * length2 * mass2 * delta_theta.cos()))
//         / denom2;

//     (accel1, accel2)
// }

// pub fn rk4_step(
//     theta1: f32,
//     omega1: f32,
//     theta2: f32,
//     omega2: f32,
//     length1: f32,
//     length2: f32,
//     mass1: f32,
//     mass2: f32,
//     g: f32,
//     dt: f32,
// ) -> (f32, f32, f32, f32) {
//     // k1
//     let (accel1_k1, accel2_k1) = pendulum_accelerations(theta1, theta2, omega1, omega2, length1, length2, mass1, mass2, g);
//     let theta1_k1 = omega1;
//     let theta2_k1 = omega2;
    
//     // k2
//     let theta1_k2 = omega1 + 0.5 * dt * accel1_k1;
//     let theta2_k2 = omega2 + 0.5 * dt * accel2_k1;
//     let (accel1_k2, accel2_k2) = pendulum_accelerations(
//         theta1 + 0.5 * dt * theta1_k1,
//         theta2 + 0.5 * dt * theta2_k1,
//         omega1 + 0.5 * dt * accel1_k1,
//         omega2 + 0.5 * dt * accel2_k1,
//         length1, length2, mass1, mass2, g,
//     );

//     // k3
//     let theta1_k3 = omega1 + 0.5 * dt * accel1_k2;
//     let theta2_k3 = omega2 + 0.5 * dt * accel2_k2;
//     let (accel1_k3, accel2_k3) = pendulum_accelerations(
//         theta1 + 0.5 * dt * theta1_k2,
//         theta2 + 0.5 * dt * theta2_k2,
//         omega1 + 0.5 * dt * accel1_k2,
//         omega2 + 0.5 * dt * accel2_k2,
//         length1, length2, mass1, mass2, g,
//     );

//     // k4
//     let theta1_k4 = omega1 + dt * accel1_k3;
//     let theta2_k4 = omega2 + dt * accel2_k3;
//     let (accel1_k4, accel2_k4) = pendulum_accelerations(
//         theta1 + dt * theta1_k3,
//         theta2 + dt * theta2_k3,
//         omega1 + dt * accel1_k3,
//         omega2 + dt * accel2_k3,
//         length1, length2, mass1, mass2, g,
//     );

//     // Aggiornare i valori finali usando la media pesata dei k
//     let theta1_next = theta1 + (dt / 6.0) * (theta1_k1 + 2.0 * theta1_k2 + 2.0 * theta1_k3 + theta1_k4);
//     let omega1_next = omega1 + (dt / 6.0) * (accel1_k1 + 2.0 * accel1_k2 + 2.0 * accel1_k3 + accel1_k4);
//     let theta2_next = theta2 + (dt / 6.0) * (theta2_k1 + 2.0 * theta2_k2 + 2.0 * theta2_k3 + theta2_k4);
//     let omega2_next = omega2 + (dt / 6.0) * (accel2_k1 + 2.0 * accel2_k2 + 2.0 * accel2_k3 + accel2_k4);

//     (theta1_next, omega1_next, theta2_next, omega2_next)
// }

// // pub fn update_pendulum_system_rk4(
// //     time: Res<Time>,
// //     mut query: Query<&mut Pendulum>,
// // ) {
// //     let dt = time.delta_seconds();
// //     let g = 9.81; // gravità

// //     for mut pendulum in query.iter_mut() {
// //         let (theta1, omega1, theta2, omega2) = rk4_step(
// //             pendulum.theta1,
// //             pendulum.omega1,
// //             pendulum.theta2,
// //             pendulum.omega2,
// //             pendulum.length1,
// //             pendulum.length2,
// //             pendulum.mass1,
// //             pendulum.mass2,
// //             g,
// //             dt,
// //         );

// //         // Aggiornare i valori dei pendoli
// //         pendulum.theta1 = theta1;
// //         pendulum.omega1 = omega1;
// //         pendulum.theta2 = theta2;
// //         pendulum.omega2 = omega2;
// //     }
// // }

