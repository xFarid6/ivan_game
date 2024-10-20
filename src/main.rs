#[allow(unused_variables)]

// Modules
mod app_utils;
mod constants;
mod ui;

// Bevy
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
    render::mesh::Mesh,
    sprite::MaterialMesh2dBundle,
    winit::WinitSettings,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

// Std rust libraries
use rand::Rng;
use std::time::Duration;

// Other libraries

// Personal imports
use app_utils::*;
use constants::*;
use ui::*;

fn main() {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(
                1000,
            )),
        })
        // .insert_resource(WinitSettings::desktop_app())
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (fps_text_update_system, gravity_text_update_system))
        .add_systems(
            FixedUpdate,
            (
                (
                    window_walls,
                    reposition_ball_on_mouse_click,
                    apply_gravity,
                    apply_velocity,
                )
                    .chain(),
                (add_ball_random_pos, remove_temp_balls),
                change_gravity,
                translate_everything_on_window_move,
                draw_a_line_example,
                draw_cursor,
                close_on_esc,
            ),
        )
        .run();
}

// A unit struct to help identify the Ball component
#[derive(Component)]
struct Ball;

#[derive(Component, Debug)]
struct TempBall;

#[derive(Component, Deref, DerefMut, Debug, PartialEq)]
struct Velocity(Vec2);

#[derive(Debug, Resource)]
struct Gravity {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Triangle;

#[derive(Component)]
struct Collidable;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Ball
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(BALL_STARTING_POSITION)
                .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
            ..default()
        },
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
    ));

    // Triangle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Triangle2d::new(
                    Vec2::Y * 50.0,
                    Vec2::new(-50.0, -50.0),
                    Vec2::new(50.0, -50.0),
                ))
                .into(),
            material: materials.add(Color::srgb(1., 0., 0.)),
            // transform: Transform::from_translation(Vec3 { x: 0., y: 0., z: 1. }),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 1.,
                },
                scale: Vec3 {
                    x: 2.,
                    y: 3.,
                    z: 1.,
                },
                ..default()
            },
            ..default()
        },
        Triangle,
        Collidable,
    ));

    // Gravity
    commands.insert_resource(Gravity { x: 0., y: -1. });

    // FPS Text
    commands.spawn((
        TextBundle::from_section(
            "FPS: ",
            TextStyle {
                font_size: 30.,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.),
            right: Val::Px(50.),
            ..default()
        }),
        FpsText,
    ));

    // Gravity Text
    commands.spawn((
        TextBundle::from_section(
            "Gravity values: ",
            TextStyle {
                font_size: 30.,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.),
            right: Val::Px(50.),
            ..default()
        }),
        GravityText,
    ));

    // Init LastWindowPos Resource
    // commands.insert_resource(LastWindowPos{ x: 0, y: 0 });
    commands.init_resource::<LastWindowPos>();
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

// TODO: Fix after window movement
// Add boundaries for the ball (screen)
fn window_walls(
    mut query: Query<(&mut Transform, &mut Velocity, &Ball)>,
    window: Query<&Window>, // mut resize_events: EventReader<WindowResized>
) {
    // let (mut wx, mut wy) = (0., 0.);
    // for event in resize_events.read().into_iter() {
    //    // println!("width = {} height = {}", event.width, event.height);
    //    (wx, wy) = (event.width, event.height);
    // }

    let (window_width, window_height) = getwindowsize(window);

    let right_x = window_width / 2.;
    let left_x = -right_x;
    let upper_y = window_height / 2.;
    let lower_y = -upper_y;
    // println!("{} {} {} {}", right_x,lower_y, wx, wy);

    for (mut transform, mut velocity, _) in &mut query {
        // Lower screen border
        if transform.translation.y < lower_y + BALL_RADIUS {
            transform.translation.y = lower_y + BALL_RADIUS;
            velocity.y = 0.;
        }

        // Upper screen border
        if transform.translation.y > upper_y - BALL_RADIUS {
            transform.translation.y = upper_y - BALL_RADIUS;
            velocity.y = 0.;
        }

        // Right border
        if transform.translation.x > right_x - BALL_RADIUS {
            transform.translation.x = right_x - BALL_RADIUS;
            velocity.x = 0.;
        }

        // Left border
        if transform.translation.x < left_x + BALL_RADIUS {
            transform.translation.x = left_x + BALL_RADIUS;
            velocity.x = 0.;
        }
    }
}

// Add gravity for making all balls fall
fn apply_gravity(mut query: Query<&mut Velocity>, gravity: Res<Gravity>) {
    for mut velocity in &mut query {
        velocity.x += gravity.x;
        velocity.y += gravity.y;
    }
}

// Reposition ball on mouse-click position
// The fix is to use a ParamSet.
// This allows you to define multiple queries that access the same component without conflicts,
// ensuring that each query is disjoint.
fn reposition_ball_on_mouse_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window: Query<&Window>,
    mut ball_transform: Query<&mut Transform, With<Ball>>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Some(cursor_position) = window.single().cursor_position() else {
            return;
        };

        let (window_width, window_height) = getwindowsize(window);

        let mut b_transform = ball_transform.single_mut();

        b_transform.translation.x = cursor_position.x - (window_width / 2.);

        if cursor_position.y <= (window_height / 2.) {
            b_transform.translation.y = (window_height / 2.) - cursor_position.y;
        } else {
            b_transform.translation.y = -(cursor_position.y - (window_height / 2.));
        }
    }
}

fn ball_to_mouse_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut param_set: ParamSet<(
        Query<&mut Transform, With<Ball>>,
        Query<&Transform, With<Camera>>,
    )>,
    windows: Query<&Window>,
) {
    // Early return condition
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let m_pos = windows
        .single()
        .cursor_position()
        .expect("Mouse in the window");
    let window = windows.single();
    let binding = param_set.p1();
    let camera = binding.single();
    let new_pos = window_to_world(m_pos, window, camera);
    param_set.p0().single_mut().translation = Vec3 {
        x: new_pos.x,
        y: -new_pos.y,
        z: 1.,
    };

    // NOTE: using Rust’s destructuring syntax to unpack a Vec2 into separate variables
    // let Vec2 { x: mx, y: my } = windows.get_single()
    //       .expect("Didn't find exactly one window")
    //       .cursor_position()
    //       .expect("Didn't find the cursor in the window");

    // let window = windows.get_single().expect("Only one window");
    // let binding = param_set.p1();
    // let camera = binding.get_single().expect("Only one camera");
    // let Vec3 { x: wmx, y: wmy, z: _ } = window_to_world(Vec2 { x: mx, y: my }, window, camera);

    // param_set.p0().get_single_mut()
    // .expect("Only one ball")
    // .translation.x = wmx;

    // param_set.p0().get_single_mut()
    // .expect("Only one ball")
    // .translation.y = wmy;
}

fn change_gravity(mut gravity: ResMut<Gravity>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        gravity.y += -GRAVITY_CHANGE;
    } else if keyboard_input.pressed(KeyCode::ArrowUp) {
        gravity.y += GRAVITY_CHANGE;
    }

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        gravity.x += -GRAVITY_CHANGE;
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        gravity.x += GRAVITY_CHANGE;
    }

    // Normalizing gravity values
    let ng = Vec2::new(gravity.x, gravity.y).normalize() * Vec2::new(4., 4.);
    gravity.x = ng[0];
    gravity.y = ng[1];
}

fn add_ball_random_pos(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    if keyboard_input.pressed(KeyCode::KeyA) {
        // Create rng generator
        let mut rng = rand::thread_rng();
        // println!("Integer: {}", rng.gen_range(0..10));
        // println!("Float: {}", rng.gen_range(0.0..10.0));

        // Get window dimensions
        let (width, height) = getwindowsize(window);

        // Select upper part of the window
        let x = rng.gen_range(-width / 2. ..width / 2.);
        let y = rng.gen_range((height / 2.) - 50. ..height / 2.);

        // Describe the new position for the ball
        let pos: Vec3 = Vec3 { x, y, z: 1. };

        // Spawn the ball
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                material: materials.add(Color::WHITE),
                transform: Transform::from_translation(pos)
                    .with_scale(Vec2::splat(BALL_DIAMETER / 5.).extend(1.)),
                ..default()
            },
            TempBall,
            Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
        ));
    }
}

fn remove_temp_balls(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<TempBall>>,
    window: Query<&Window>,
) {
    let (_w, h) = getwindowsize(window);
    let basevec = Vec3 {
        x: 0.0,
        y: -h / 2.,
        z: 1.,
    };
    for (entity, transform) in query.iter() {
        if transform.translation.y < basevec.y + BALL_RADIUS {
            commands.entity(entity).despawn();
        }
    }
}

fn draw_a_line_example(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::new(20., 20., 1.), Color::WHITE);
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a world position based on the cursor's position.
    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    gizmos.circle_2d(point, 10., Color::WHITE);
}
