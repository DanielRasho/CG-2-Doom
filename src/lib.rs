mod internal;

use minifb::{Window, WindowOptions, Key};
use std::time::Duration;
use internal::framebuffer::{Framebuffer};
use internal::color::Color;
use internal::render::{render_2d,draw_cell};
use internal::maze::{Maze, load_maze};

pub fn start(){
    
    // Window Size configuration
    let window_width = 660;
    let window_height = 450;
    let framebuffer_width =  window_width;
    let framebuffer_height = window_height;
    
    // Frame Rate
    let frame_delay = Duration::from_millis(16);
  
    // Window Objects initialization
    let mut framebuffer = Framebuffer::new(window_width, window_height, Color::new(0, 0, 0));
    let mut window = Window::new(
      "The Mynotaurs Maze",
      window_width,
      window_height,
      WindowOptions::default()
    ).unwrap();
    window.update();
    
    // LOAD MAZE
    let maze = load_maze("./maze.txt");
    
    // RENDER LOOP
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        
        render_2d(&mut framebuffer, &maze);
        framebuffer.set_current_color(Color::new(0, 0, 230));

        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        std::thread::sleep(frame_delay)
    }
  
}