#[allow(unused_variables)]
use bevy::prelude::{Vec2, Vec3};

pub const HELLO_BEVY: &'static str = "Hello Bevy!";
pub const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
pub const BALL_RADIUS: f32 = 15.;
pub const BALL_DIAMETER: f32 = BALL_RADIUS * 2.;
pub const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0., -0.1);
pub const BALL_SPEED: f32 = 400.0;
pub const MAX_GRAVITY: f32 = 8.;
pub const MIN_GRAVITY: f32 = -8.;
pub const GRAVITY_CHANGE: f32 = 0.5;
