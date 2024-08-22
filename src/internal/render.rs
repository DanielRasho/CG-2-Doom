use super::caster::cast_ray;
use super::color::Color;
use super::framebuffer::{Framebuffer};
use super::maze::Maze;
use super::player::Player;

const CELL_SIZE : usize = 30;

pub fn render(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    let sky_color = Color::new(53, 53, 53);
    let floor_color = Color::new(30, 30, 30);

    // Render sky and floor
    for i in 0..framebuffer.width {
        framebuffer.set_current_color(sky_color);
        for j in 0..(framebuffer.height / 2) {
            framebuffer.draw_point(i, j);
        }
        framebuffer.set_current_color(floor_color);
        for j in (framebuffer.height / 2)..framebuffer.height {
            framebuffer.draw_point(i, j);
        }
    }

    // Render walls with textures
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle = player.angle - (player.field_of_view / 2.0) + (player.field_of_view * current_ray);
        let intersect = cast_ray(framebuffer, maze, player, angle, CELL_SIZE, false);
        
        let distance = intersect.distance * (angle - player.angle).cos();
        let stake_height = (framebuffer.height as f32 / distance) * 30.0;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        // Determine texture coordinates
        let texture_height = 512 - 1; // Correct height of the texture
        let texture_width = 512 - 1;  // Correct width of the texture

        for y in stake_top..stake_bottom {
            let texture_y = ((y - stake_top) as f32 / stake_height) * texture_height as f32;
            
            // Calculate texture_x based on hit_x and wall orientation
            let texture_x = intersect.hit_x as f32 / CELL_SIZE as f32 * texture_width as f32;
            
            // Get the color from the texture
            let color = maze.texture_for_cell(intersect.impact_char, texture_x as u32, texture_y as u32);
            framebuffer.set_current_color(color);
            framebuffer.draw_point(i, y);
        }
    }
    render_minimap(framebuffer, maze, player, 0, 0)
}

pub fn render_minimap(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, posx: usize, posy: usize){

    // Clear framebuffer
    let cell_size = 8;
    
    for row in 0..maze.rows() {
        for col in 0..maze.column_len(row) {
            let cell_xo = col * cell_size;
            let cell_yo = row * cell_size;
            let color = maze.color_for_cell(row, col);
            framebuffer.set_current_color( color );
            draw_cell(framebuffer, cell_xo, cell_yo, cell_size);
        }
    }
    
    // Render player
    let player_x = player.pos.x / (cell_size as f32 / 2.0);
    let player_y = player.pos.y / (cell_size as f32 / 2.0);
    framebuffer.set_current_color_hex( 0xFF0000 );
    draw_cell(framebuffer, player_x as usize, player_y as usize, cell_size);
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