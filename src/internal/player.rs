use std::{cell, f32::consts::PI};

use minifb::{Window, Key};
use nalgebra_glm::Vec3;

use super::maze::Maze;

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

pub fn process_events (window: &Window, player: &mut Player, maze: &Maze , cell_size: usize) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION: f32 = PI / 15.0;

    if window.is_key_down(Key::Left) {
        player.angle -= ROTATION;
    }
    if window.is_key_down(Key::Right) {
        player.angle += ROTATION;
    }
    if window.is_key_down(Key::Up) {
        let new_x = player.pos.x + MOVE_SPEED * player.angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.angle.sin();
        
        if !is_collision(new_x, new_y, maze, cell_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
    
    if window.is_key_down(Key::Down) {
        let new_x = player.pos.x - MOVE_SPEED * player.angle.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.angle.sin();
        
        if !is_collision(new_x, new_y, maze, cell_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
}

fn is_collision(x: f32, y: f32, maze: &Maze, cell_size: usize) -> bool {
    let i = (x / cell_size as f32) as usize;
    let j = (y / cell_size as f32) as usize;
    
    let cell = maze.char_at(j, i);
    cell != ' '
}