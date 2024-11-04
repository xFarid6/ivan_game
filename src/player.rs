use std::{fs, path::Path, collections::HashMap, time::Duration};

use bevy::{prelude::*, transform::commands};

/*
What should the player (Entity) be composed (Components) of?
- Healt
- Position
- Speed of movement
- is moving
- is jumping
- some inventory?
- how does he carry tools?
- Collision box (probably)
*/

// ====== STRUCTS ======

#[derive(Bundle)]
struct PlayerBundle {
    xp: PlayerXp,
    name: PlayerName,
    health: Health,
    marker: Player,

    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    sprite: SpriteBundle,
}

#[derive(Component, Debug)]
pub struct Health {
    hp: f32,
    extra: f32
} 

#[derive(Component)]
pub struct PlayerXp(u32);

#[derive(Component)]
pub struct PlayerName(String);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

// ====== METHODS ======

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            xp: PlayerXp(0),
            name: PlayerName("Player".into()),
            health: Health {
                hp: 100.0,
                extra: 0.0,
            },
            marker: Player,
            sprite: Default::default(),
        }
    }
}

impl PlayerXp {
    fn add(&mut self, amount: u32) {
        self.0 += amount
    }

    fn reset(&mut self) {
        self.0 = 0
    }
}

impl AnimationConfig {
    fn new(first: usize, action: &str, fps: u8) -> Self {
        let mut durations: HashMap<&str, u32> = HashMap::from([
            ("Attack", 3),
            ("Born", 6),
            ("Die", 10),
            ("Hurt", 3),
            ("Idle", 3),
            ("Idle2", 3),
            ("Jump", 7),
            ("Walking", 4),
        ]);

        Self {
            first_sprite_index: 1,
            last_sprite_index: *durations.get(&action).expect("Action not found") as usize,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

pub fn load_slimes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut durations: HashMap<&str, u32> = HashMap::from([
        ("Attack", 3),
        ("Born", 6),
        ("Die", 10),
        ("Hurt", 3),
        ("Idle", 3),
        ("Idle2", 3),
        ("Jump", 7),
        ("Walking", 4),
    ]);
    let slime_pixel_dimensions = 16;

    // Loop Slime Folders
    for entry in fs::read_dir("assets/characters/16PixelSlime").expect("Unable to list") {
        let entry = entry.expect("unable to get entry");
        if entry.path().ends_with(".DS_Store") || entry.path().ends_with("Aseprite") || entry.path().ends_with("GIFs") {
            continue;
        }
        // Loop Slime Spritesheets
        for file in fs::read_dir(entry.path()).expect("entry not found or smth") {
            // Work the name 
            let file = file.expect("unable to get entry");
            let file_name = file.path().file_name().unwrap().to_string_lossy().into_owned();
            // dbg!(&file_name);
            let file = file.path().to_string_lossy().into_owned();
            let file = file.replace("assets/", "");

            // Load the spritesheet
            let slime_action_sheet: Handle<Image> = asset_server.load(file);

            let action = get_slime_action(file_name);
            let action = action.as_str();
            // dbg!(action);

            // Get the appropriate values and store them in the Texture Atlas as a Layout
            // e.g. the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
            let layout = TextureAtlasLayout::from_grid
            (
                UVec2::splat(slime_pixel_dimensions), 
                *durations.get(&action).expect("Action not found in table"), 
                1, 
                None, None
            );
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
        }
    }
}

// fn spawn_slime(
//     commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,

// ) {
//     // Now spawn a first slime
//     // the first sprite runs at 10 FPS
//     let animation_config_1 = AnimationConfig::new(1, "Walking", 10);

//     // create the first (left-hand) sprite
//     let texture = asset_server.get_handle(
//         "/assets/characters/16PixelSlime/GreenSlime/GreenSlimeWalking-Sheet.png"
//     ).expect("Handle not found");
//     let texture_atlas_layout = texture_atlas_layouts.get(5).expect("Not found");
//     commands.spawn((
//         SpriteBundle {
//             transform: Transform::from_scale(Vec3::splat(6.0))
//                 .with_translation(Vec3::new(-50.0, 0.0, 0.0)),
//             texture: texture.clone(),
//             ..default()
//         },
//         TextureAtlas {
//             layout: texture_atlas_layout.clone(),
//             index: animation_config_1.first_sprite_index,
//         },
//         animation_config_1,
//     ));
// }

fn get_slime_action(file_name: String) -> String {
    // &file_name = "YellowSlimeWalking-Sheet.png" or &file_name = "YellowSlimeIdle2-Sheet.png"
    let parts: Vec<&str> = file_name.split("-").collect();
    let binding = parts[0].to_string();
    let parts: Vec<&str> = binding.split("Slime").collect();
    parts[1].to_owned()
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut query {
        // we track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished() {
            if atlas.index == config.last_sprite_index {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = config.first_sprite_index;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}
