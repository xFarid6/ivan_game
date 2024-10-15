use bevy::{
   ecs::query::{self, QueryData}, prelude::*, sprite::MaterialMesh2dBundle, window::WindowResized
};

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const BALL_RADIUS: f32 = 15.;
const BALL_DIAMETER: f32 = BALL_RADIUS * 2.;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0., -0.1);
const BALL_SPEED: f32 = 400.0;
const GRAVITY: Vec2 = Vec2::new(0., -0.5);

fn main() {
   App::new()
      .add_plugins(DefaultPlugins)
      .add_systems(Startup, setup)
      .add_systems(Update, resize_notificator)
      .add_systems(FixedUpdate, (window_walls, apply_gravity, apply_velocity).chain())
      .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut, Debug)]
struct Velocity(Vec2);

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
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

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

   let (wx, wy) = getwindowsize(window); 

   let right_x = wx / 2.;
   let left_x = -right_x;
   let upper_y = wy / 2.;
   let lower_y = -upper_y;
   // println!("{} {} {} {}", right_x,lower_y, wx, wy);
    
   for (mut transform, mut velocity, _) in &mut query {

      // Lower screen border
      if transform.translation.y < lower_y + BALL_RADIUS {
         transform.translation.y = lower_y + BALL_RADIUS;
         velocity.y = 0.;
      }

      // Upper screen border
      if transform.translation.y > upper_y + BALL_RADIUS {
         transform.translation.y = upper_y - BALL_RADIUS;
         velocity.y = 0.;
      }

      // Right border
      if transform.translation.x > right_x + BALL_RADIUS {
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

fn resize_notificator(mut resize_events: EventReader<WindowResized>) {
   for event in resize_events.read().into_iter() {
      //  println!("width = {} height = {}", event.width, event.height);
   }
}

fn getwindowsize(window: Query<&Window>) -> (f32, f32){
   let window = window.single();

   let width = window.resolution.width();
   let height = window.resolution.height();

   // dbg!(width, height, x, y);
   (width, height)
}

fn apply_gravity(mut query: Query<&mut Velocity>) {
   for mut velocity in &mut query {
      velocity.x += GRAVITY.x;
      velocity.y += GRAVITY.y;
   }
}
