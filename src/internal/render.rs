use core::num;

use super::caster::cast_ray;
use super::color::Color;
use super::framebuffer::{Framebuffer};
use super::maze::Maze;
use super::player::Player;

const CELL_SIZE : usize = 30;

pub fn render_2d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player){

    // Clear framebuffer
    framebuffer.set_background_color_hex(0x333355);
    framebuffer.clear();
    
    for row in 0..maze.rows() {
        for col in 0..maze.column_len(row) {
            let cell_xo = col * CELL_SIZE;
            let cell_yo = row * CELL_SIZE;
            let color = maze.color_for_cell(row, col);
            framebuffer.set_current_color( color );
            draw_cell(framebuffer, cell_xo, cell_yo, CELL_SIZE);
        }
    }
    
    // Render player
    framebuffer.set_current_color_hex( 0xFF0000 );
    draw_cell(framebuffer, player.pos.x as usize, player.pos.y as usize, 4);
    
    let num_rays = 5;
    for i in 0..num_rays{
        let current_ray = i as f32 / num_rays as f32;
        let a = player.angle - (player.field_of_view/ 2.0) + (player.field_of_view * current_ray);
        cast_ray(framebuffer, maze, player, a, CELL_SIZE, true);
    }
}

pub fn draw_cell(framebuffer: &mut Framebuffer,
    xo : usize,
    yo : usize,
    cell_size: usize
){
    for x in xo..xo + cell_size {
        for y in yo..yo + cell_size {
            framebuffer.draw_point(x, y)
        }
    }
}