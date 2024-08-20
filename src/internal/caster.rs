
use super::{framebuffer::{Framebuffer}, maze::Maze, player::{Player}};

pub struct Intersect {
    pub distance : f32,
    pub impact : char
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    cell_size: usize,
    draw_line: bool
) -> Intersect {
    let mut d = 0.0;
    
    framebuffer.set_current_color_hex(0xFFFFFF);
    loop {
        let cos = d * player.angle.cos();
        let sin = d * player.angle.sin();
        
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
                impact: cell
            }
        }
        d += 5.0
    }
}