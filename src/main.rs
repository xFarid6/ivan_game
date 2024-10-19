mod constants;

use bevy::{
   diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume}, prelude::*, render::view::window, sprite::MaterialMesh2dBundle, winit::WinitSettings
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng;
use std::time::Duration;

use constants::*;


fn main() {
   App::new()
      .insert_resource
      (
         WinitSettings 
         {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::reactive_low_power(Duration::from_millis(10))
         }
      )
      .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
      .add_plugins(WorldInspectorPlugin::new())
      .add_systems(Startup, setup)
      .add_systems(Update, (fps_text_update_system, gravity_text_update_system))
      .add_systems(FixedUpdate, (
                  (window_walls, mouse_click, apply_gravity, apply_velocity).chain(),
                  (add_ball_random_pos, remove_temp_balls),
                  change_gravity,
                  camera_move_on_window_move
               ))
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
struct Gravity{
   x: f32,
   y: f32,
}

#[derive(Component)]
struct Triangle;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct GravityText;

#[derive(Component)]
struct Collidable;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Resource, Debug)]
struct LastWindowPos {
   x: i32,
   y: i32
}

impl Default for LastWindowPos {
   fn default() -> Self {
       LastWindowPos { x: 0, y: 0 }
   }
}

impl LastWindowPos {
   fn position(&self) -> IVec2 {
      IVec2 { x: self.x, y: self.y }
   }
}

fn setup (
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
         mesh: meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        )).into(),
         material: materials.add(Color::srgb(1., 0., 0.)),
         // transform: Transform::from_translation(Vec3 { x: 0., y: 0., z: 1. }),
         transform: Transform { translation: Vec3 { x: 0., y: 0., z: 1. }, scale: Vec3 { x: 2., y: 3., z: 1. }, ..default() },
         ..default()
      },
      Triangle,
      Collidable
   ));

   // Gravity
   commands.insert_resource( Gravity { x: 0., y: -1. } );

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
   commands.insert_resource(LastWindowPos{ x: 0, y: 0 });

}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
      transform.translation.x += velocity.x * time.delta_seconds();
      transform.translation.y += velocity.y * time.delta_seconds();
    }
}

// Add boundaries for the ball (screen)
fn window_walls(
   mut query: Query<(&mut Transform, &mut Velocity, &Ball)>,
   window: Query<&Window>
   // mut resize_events: EventReader<WindowResized>
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

fn getwindowsize(window: Query<&Window>) -> (f32, f32){
   let window = window.single();

   let width = window.resolution.width();
   let height = window.resolution.height();

   // dbg!(width, height, x, y);
   (width, height)
}

// Add gravity for making all balls fall
fn apply_gravity(
   mut query: Query<&mut Velocity>,
   gravity: Res<Gravity>
) {
   for mut velocity in &mut query {
      velocity.x += gravity.x;
      velocity.y += gravity.y;
   }
}

// Reposition ball on mouse-click position
fn mouse_click(
   mouse_button_input: Res<ButtonInput<MouseButton>>,
   mut query: Query<&mut Transform, With<Ball>>,
   window: Query<&Window>,
) {
   if mouse_button_input.just_pressed(MouseButton::Left) {
      let Some(cursor_position) = window.single().cursor_position() else {
         return;
      };

      let (window_width, window_height) = getwindowsize(window);

      let mut transform = query.single_mut();

      transform.translation.x = cursor_position.x - (window_width / 2.);

      if cursor_position.y <= (window_height / 2.) {
         transform.translation.y = (window_height / 2.) - cursor_position.y;
      } else {
         transform.translation.y = -(cursor_position.y - (window_height / 2.));
      }
   }
}

fn change_gravity(
   mut gravity: ResMut<Gravity>,
   keyboard_input: Res<ButtonInput<KeyCode>>
) {
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

fn fps_text_update_system(
   diagnostics: Res<DiagnosticsStore>,
   mut query: Query<&mut Text, With<FpsText>>,
) {
   for mut text in &mut query {
       if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
           if let Some(value) = fps.smoothed() {
               // Update the value of the second section
               text.sections[0].value = format!("FPS: {value:.2}");
           }
       }
   }
}

fn gravity_text_update_system(
   mut query: Query<&mut Text, With<GravityText>>,
   gravity: Res<Gravity>
) {
   for mut text in &mut query {
      text.sections[0].value = format!("Gravity values: {:.2}, {:.2}", gravity.x, gravity.y);
   }
}

fn add_ball_random_pos(
   keyboard_input: Res<ButtonInput<KeyCode>>,
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<ColorMaterial>>,
   window: Query<&Window>
) {
   if keyboard_input.pressed(KeyCode::KeyA) {
      // Create rng generator
      let mut rng = rand::thread_rng();
      // println!("Integer: {}", rng.gen_range(0..10));
      // println!("Float: {}", rng.gen_range(0.0..10.0));

      // Get window dimensions
      let (width, height) = getwindowsize(window);

      // Select upper part of the window
      let x = rng.gen_range(-width/2. .. width/2.);
      let y = rng.gen_range((height/2.)-50. .. height/2.);

      // Describe the new position for the ball
      let pos: Vec3 = Vec3 { x, y, z: 1. };

      // Spawn the ball
      commands.spawn((
         MaterialMesh2dBundle {
            mesh: meshes.add(Circle::default()).into(),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(pos)
               .with_scale(Vec2::splat(BALL_DIAMETER/5.).extend(1.)),
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
   window: Query<&Window>
) {
   let (_w, h) = getwindowsize(window);
   let basevec = Vec3 { x: 0.0, y: -h/2., z: 1. };
   for (entity, transform) in query.iter() {
      if transform.translation.y < basevec.y + BALL_RADIUS {
         commands.entity(entity).despawn();
      }
  }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn ball_collision(
   ball: BoundingCircle,
   bounding_box: Aabb2d
) -> Option<Collision> {
   if !ball.intersects(&bounding_box) {
      return None;
  }

  let closest = bounding_box.closest_point(ball.center());
  let offset = ball.center() - closest;
  let side =   if offset.x.abs() > offset.y.abs() 
               {
                  if offset.x < 0. {
                     Collision::Left
                  } else {
                     Collision::Right
                  }
               } else if offset.y > 0. {
                     Collision::Top
               } else {
                     Collision::Bottom
               };

  Some(side)
}

// Move the camera when moving the window
// WindowMoved - An event that is sent when a window is repositioned in physical pixels.
// WindowResized - A window event that is sent whenever a window’s logical size has changed.

// fn window_resized_event(mut events: EventReader<WindowResized>) {
//    for event in events.iter() {
//        // resize window
//    }
// }


fn camera_move_on_window_move(
   mut win_move: EventReader<WindowMoved>, 
   window: Query<&Window>,
   mut last_win_pos: ResMut<LastWindowPos>,
   mut camera: Query<&mut Transform, With<Camera>>
) {
   /*
   Manual Dragging: Bevy, by default, does not detect when a window is dragged manually 
   by the user outside of Bevy's API. If you need to track such changes, 
   you'd need to use platform-specific solutions 
   (e.g., using raw window handles and interfacing with system APIs).
   */
    
   // Can we raelly use this super useful method on window.position and that's it?!
   // .is_changed()

   let window_s = window.single();
   // println!("{:?}", window.position);

   // Basically if the window hasn't ever been moved this function will **return early**
   match window_s.position {
      WindowPosition::Automatic => return,
      WindowPosition::Centered(_monitor_selection) => return,
      WindowPosition::At(ivec2) => {
         if ivec2 == last_win_pos.position() {
            return;
         }
      },
   }

   // Get current window coordinates
   let (mut current_w_x, mut current_w_y) = get_window_coordinates(window);

   // Calculate the shift
   let last_w_x = last_win_pos.x;
   let last_w_y = last_win_pos.y;
   
   let move_on_x = current_w_x - last_w_x;
   let move_on_y = current_w_y - last_w_y;

   // Reposition the camera
   let mut camera_transform = camera.single_mut();

   camera_transform.translation = Vec3 { 
      x: camera_transform.translation.x + move_on_x as f32, 
      y: camera_transform.translation.y - move_on_y as f32, 
      ..camera_transform.translation};


   // Update the LastWinPos Resource
   last_win_pos.x = current_w_x;
   last_win_pos.y = current_w_y;
}

// Close the focused window whenever the escape key (<kbd>Esc</kbd>) is pressed
//
// This is useful for examples or prototyping.
pub fn close_on_esc(
   mut commands: Commands,
   focused_windows: Query<(Entity, &Window)>,
   input: Res<ButtonInput<KeyCode>>,
) {
   for (window, focus) in focused_windows.iter() {
       if !focus.focused {
           continue;
       }

       if input.just_pressed(KeyCode::Escape) {
           commands.entity(window).despawn();
       }
   }
}

fn get_window_coordinates(
   window: Query<&Window>
) -> (i32, i32){
   let window = window.get_single().expect("The number of queries for &Window was not 1");
   // Get current window coordinates
   let mut current_w_x: i32 = 0;
   let mut current_w_y: i32 = 0;
   match window.position {
      WindowPosition::Automatic => { }, // println!("Window position: Automatic (system defined)"),
      WindowPosition::Centered(_monitor) => { }, // println!("Window position: {:?}", monitor),
      WindowPosition::At(pos) => {
         current_w_x = pos.x;
         current_w_y = pos.y;
       },// println!("Window position: At x: {}, y: {}", pos.x, pos.y),
   }

   (current_w_x, current_w_y)
}

