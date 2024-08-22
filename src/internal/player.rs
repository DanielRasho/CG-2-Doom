use std::{cell, f32::consts::PI};

use minifb::{Window, Key, MouseMode};
use nalgebra_glm::Vec3;

use super::maze::Maze;

pub struct Player {
    pub pos: Vec3,
    pub angle: f32,
    pub field_of_view: f32,
    pub mouse_x: f32, // Track mouse position
    pub mouse_y: f32,
}

impl Player {
    pub fn new(pos: Vec3, angle: f32, field_of_view: f32) -> Player {
        Player {
            pos,
            angle,
            field_of_view,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }
}

pub fn process_events(window: &Window, player: &mut Player, maze: &Maze, cell_size: usize) {
    const MOVE_SPEED: f32 = 5.0;
    const MOUSE_SENSITIVITY: f32 = 0.005; // Adjust sensitivity as needed
    const ROTATION_SPEED: f32 = PI / 16.0;

    let mouse_pos = window.get_mouse_pos(MouseMode::Clamp).unwrap_or((0.0, 0.0));
    let (mouse_x, mouse_y) = mouse_pos;

    let dx = mouse_x - player.mouse_x;

    player.angle += dx as f32 * MOUSE_SENSITIVITY;

    player.mouse_x = mouse_x;
    player.mouse_y = mouse_y;

    if window.is_key_down(Key::A) {
        player.angle -= ROTATION_SPEED;
    }
    if window.is_key_down(Key::D) {
        player.angle += ROTATION_SPEED;
    }
    if window.is_key_down(Key::W) {
        let new_x = player.pos.x + MOVE_SPEED * player.angle.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.angle.sin();
        
        if !is_collision(new_x, new_y, maze, cell_size) {
            player.pos.x = new_x;
            player.pos.y = new_y;
        }
    }
    
    if window.is_key_down(Key::S) {
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