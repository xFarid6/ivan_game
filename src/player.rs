use bevy::prelude::*;

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