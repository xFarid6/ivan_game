use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

// TODO: aggiungere una pallina alla finestra e farla cadere a terra

fn main() {
   App::new()
      .add_plugins(DefaultPlugins)
      .add_systems(Startup, setup_system)
      .add_systems(Update, hello_world_system)
      .run();
}

fn setup_system(
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<ColorMaterial>>,
   asset_server: Res<AssetServer>,
) {

   // Camera
   commands.spawn(Camera2dBundle::default());

   // Ball
   commands.spawn();
}

fn hello_world_system() {
   println!("hello world");
}

