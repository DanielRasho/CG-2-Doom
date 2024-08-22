use std::time::Instant;
mod internal;

use nalgebra_glm::Vec3;
use rusttype::Font;

use internal::player::{process_events, Player};
use minifb::{Window, WindowOptions, Key};
use std::f32::consts::PI;
use std::time::Duration;
use internal::framebuffer::{Framebuffer};
use internal::color::Color;
use internal::render::{render};
use internal::maze::{Maze, load_maze};
use internal::text::{render_text, render_text_thicker, render_text_with_outline};

fn calculate_fps(last_frame_time: &mut Instant) -> f32 {
    let now = Instant::now();
    let duration = now.duration_since(*last_frame_time);
    *last_frame_time = now;

    1.0 / duration.as_secs_f32()
}

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
    
    // LOAD FONTS
    let font_data = include_bytes!("../assets/fonts/NotoSansMono-Bold.ttf");
    let font = Font::try_from_bytes(font_data as &[u8]).expect("Error loading font");
    
    // LOAD MAZE
    let maze = load_maze("./maze.txt");
    
    // Player
    let mut player = Player::new( 
        Vec3::new(60.0, 50.0, 0.0), 
        PI/2.0,
        PI/3.0
    );

    let mut last_frame_time = Instant::now();
    let mut fps = 0.0;
    
    // RENDER LOOP
    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }
        

        render(&mut framebuffer, &maze, &player);

        process_events(&mut window, &mut player, &maze, 30);

        let fps = calculate_fps(&mut last_frame_time);
        let fps_text = format!("FPS: {}", fps as u32);
        render_text_with_outline(&mut framebuffer, &font, &fps_text, 550, 380, 20.0, Color::new(255, 255, 255), Color::new(0, 0, 0), 2);

        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        framebuffer.clear();
        std::thread::sleep(frame_delay)
    }
  
}