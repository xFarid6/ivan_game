use std::borrow::BorrowMut;
use std::process::exit;
use std::time::Instant;

use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use std::f32::consts::{TAU, PI};

#[derive(Debug, Component)]
pub struct CircleTimer(Instant);

pub fn setup_circle_timer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(CircularSector::new(80.0, 0.))),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                0.0,
                30.0,
                0.0,
            ),
            ..default()
        },
        CircleTimer(Instant::now()),
        )
    );
}

pub fn update_timer(
    mut query: Query<(&mut Mesh2dHandle, &CircleTimer, &mut Transform), With<CircleTimer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>
) {
    let duration_of_timer = 10.;
    // 2p : 10 = x : t
    // x = 2p * t / 10

    // 1s = 2p / 10
    
    for (mut q, ct, mut transf) in query.iter_mut() {
        let elapsed_time = ct.0.elapsed().as_secs_f32();
        let advancement = PI * elapsed_time / duration_of_timer;
        transf.rotate_z(f32::to_radians(PI / duration_of_timer));
        q.0 = meshes.add(CircularSector::new(80.0, advancement));

        if elapsed_time >= duration_of_timer + 0.4 {
            exit(0);
        }        
    }
}

pub fn update_timer0(
    mut query: Query<(&mut Mesh2dHandle, &CircleTimer, &mut Transform), With<CircleTimer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>
) {
    let duration_of_timer = 10.;
    // TAU : 10 = x : t
    // x = TAU * t / 10

    for (mut q, ct, mut transf) in query.iter_mut() {
        let advancement = ct.0.elapsed().as_secs_f32() / 2.;
        transf.rotate_z(f32::to_radians(-0.32));
        q.0 = meshes.add(CircularSector::new(80.0, advancement));
    }
}

pub fn update_timer1(
    mut query: Query<(&mut Mesh2dHandle, &CircleTimer, &mut Transform), With<CircleTimer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>
) {

    for (mut q, ct, mut transf) in query.iter_mut() {
        let advancement = ct.0.elapsed().as_secs_f32() / 2.;
        transf.rotate_y(advancement);
        q.0 = meshes.add(CircularSector::new(80.0, advancement));
    }
}

pub fn update_timer2(
    mut query: Query<(&mut Mesh2dHandle, &CircleTimer), With<CircleTimer>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>
) {

    for (mut q, ct) in query.iter_mut() {
        let advancement = ct.0.elapsed().as_secs_f32();
        q.0 = meshes.add(CircularSector::new(80.0, advancement));
    }
}
