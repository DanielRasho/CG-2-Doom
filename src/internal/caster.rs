use super::{color::Color, framebuffer::Framebuffer, maze::Maze, player::Player};

pub struct Intersect {
    pub distance : f32,
    pub impact_char : char,
    pub hit_x: usize
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

        let hitx = x - i * cell_size;
        let hity = y - j * cell_size;
        let mut max_hit = hity;
        
        if 1 < hitx && hitx < cell_size - 1 {
            max_hit = hitx
        }

        let cell = maze.char_at(j, i);
        if cell != ' ' {
            return Intersect {
                distance: d,
                impact_char: maze.char_at(j, i),
                hit_x: max_hit
            }
        }
        d += 0.5
    }
}