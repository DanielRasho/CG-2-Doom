
use super::{color::Color, framebuffer::Framebuffer, maze::Maze, player::Player};

pub struct Intersect {
    pub distance : f32,
    pub impact_color : Color
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    angle: f32,
    cell_size: usize,
    draw_line: bool
) -> Intersect {
    let mut d = 0.0;
    
    framebuffer.set_current_color_hex(0xFFFFFF);
    loop {
        let cos = d * angle.cos();
        let sin = d * angle.sin();
        
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;
        
        let i = x / cell_size;
        let j = y / cell_size;
        
        if draw_line == true {
            framebuffer.draw_point(x, y)
        }

        let cell = maze.char_at(j, i);
        if cell != ' ' {
            return Intersect {
                distance: d,
                impact_color: maze.color_for_cell(j, i)
            }
        }
        d += 0.5
    }
}