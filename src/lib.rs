mod internal;

use nalgebra_glm::Vec3;

use internal::player::{process_events, Player};
use minifb::{Window, WindowOptions, Key};
use std::f32::consts::PI;
use std::time::Duration;
use internal::framebuffer::{Framebuffer};
use internal::color::Color;
use internal::render::{render_2d, render,draw_cell};
use internal::maze::{Maze, load_maze};

pub fn start(){
    
    // Window Size configuration
    let window_width = 660;
    let window_height = 420;
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
    
    // Player
    let mut player = Player::new( 
        Vec3::new(60.0, 50.0, 0.0), 
        PI/2.0,
        PI/3.0
    );
    
    // RENDER LOOP
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        
        render(&mut framebuffer, &maze, &player);

        process_events(&window, &mut player, &maze, 30);

        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        framebuffer.clear();
        std::thread::sleep(frame_delay)
    }
  
}