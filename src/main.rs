use bevy::{
   prelude::*,
   sprite::MaterialMesh2dBundle,
};

const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_DIAMETER: f32 = 30.;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0., -0.5);
const BALL_SPEED: f32 = 400.0;

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .add_systems(Startup, setup)
       .add_systems(
           FixedUpdate,
           (
               apply_velocity
           ).chain(),
       )
       .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup (
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<ColorMaterial>>,
) {
   commands.spawn(Camera2dBundle::default());

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
