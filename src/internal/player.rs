use std::f32::consts::PI;

use minifb::{Window, Key};
use nalgebra_glm::Vec3;

pub struct Player {
    pub pos : Vec3,
    pub angle : f32
}

impl Player {
    pub fn new( pos: Vec3, angle: f32) -> Player {
        Player{ pos, angle }
    }
}

pub fn process_events (window: &Window, player: &mut Player) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION: f32 = PI / 15.0;

    if window.is_key_down(Key::Left) {
        player.pos.x -= MOVE_SPEED
    }
    if window.is_key_down(Key::Right) {
        player.pos.x += MOVE_SPEED
    }
    if window.is_key_down(Key::Up) {
        player.pos.y -= MOVE_SPEED
    }
    if window.is_key_down(Key::Down) {
        player.pos.y += MOVE_SPEED
    }
}