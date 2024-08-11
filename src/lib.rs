mod internal;

use internal::line::Line;
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec3;
use std::time::Duration;
use internal::framebuffer::{Framebuffer, Renderable};
use internal::color::Color;

pub fn start(){
    
    // Window Size configuration
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width =  window_width;
    let framebuffer_height = window_height;
    
    // Frame Rate
    let frame_delay = Duration::from_millis(16);
  
    // Window Objects initialization
    let mut framebuffer = Framebuffer::new(window_width, window_height, Color::new(0, 0, 0));
    let mut window = Window::new(
      "The Minotaur's Maze",
      window_width,
      window_height,
      WindowOptions::default()
    ).unwrap();
    
    // RENDER LOOP
    let mut x = 1 as i32;
    let mut speed = 1 as i32;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        
        // prepare variables for rendering
        if x as usize == framebuffer_width {
            speed = -1;
        }
        if x == 0 {
            speed = 1;
        }
        x += speed;

        // Clear the framebuffer
        framebuffer.set_background_color_hex(0x333355);
        framebuffer.clear();

        // Draw some points
        framebuffer.set_current_color_hex(0xFFDDDD);
        framebuffer.draw_point(x as usize, 40);

        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        std::thread::sleep(frame_delay)
    }
  
}