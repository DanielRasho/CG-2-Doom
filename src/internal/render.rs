use super::framebuffer::{Framebuffer};
use super::maze::Maze;

const CELL_SIZE : usize = 30;

pub fn render_2d(framebuffer: &mut Framebuffer, maze: &Maze){

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