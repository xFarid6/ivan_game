mod constants;

use bevy::{
   diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, 
   prelude::*, 
   sprite::MaterialMesh2dBundle, 
};


use constants::*;


fn main() {
   App::new()
      .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
      .add_systems(Startup, setup)
      .add_systems(Update, (fps_text_update_system, gravity_text_update_system))
      .add_systems(FixedUpdate, (
                  (window_walls, mouse_click, apply_gravity, apply_velocity).chain(),
                  change_gravity
               ))
      .run();
}

// A unit struct to help identify the Ball component
#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec2);

#[derive(Debug, Resource)]
struct Gravity{
   x: f32,
   y: f32
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

#[derive(Component)]
struct GravityText;

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

