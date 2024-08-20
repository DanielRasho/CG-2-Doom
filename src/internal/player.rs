use std::f32::consts::PI;

use minifb::{Window, Key};
use nalgebra_glm::Vec3;

pub struct Player {
    pub pos : Vec3,
    pub angle : f32,
    pub field_of_view: f32,
}

impl Player {
    pub fn new( pos: Vec3, angle: f32, field_of_view: f32) -> Player {
        Player{ pos, angle, field_of_view}
    }
}

pub fn process_events (window: &Window, player: &mut Player) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION: f32 = PI / 15.0;

    if window.is_key_down(Key::Left) {
        player.angle -= ROTATION;
    }
    if window.is_key_down(Key::Right) {
        player.angle += ROTATION;
    }
    if window.is_key_down(Key::Up) {
        player.pos.x = player.pos.x + MOVE_SPEED * player.angle.cos();
        player.pos.y = player.pos.y + MOVE_SPEED * player.angle.sin();
    }
    if window.is_key_down(Key::Down) {
        player.pos.x = player.pos.x + MOVE_SPEED * player.angle.cos();
        player.pos.y = player.pos.y + MOVE_SPEED * player.angle.sin();
    }
}