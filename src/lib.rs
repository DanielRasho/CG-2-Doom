use std::time::Instant;
mod internal;

use internal::audio::{self, AudioPlayer};
use nalgebra_glm::{Vec2, Vec3};
use rusttype::Font;
use image::{GenericImageView, DynamicImage};

use internal::player::{process_events, Player};
use minifb::{Window, WindowOptions, Key};
use std::f32::consts::PI;
use std::time::Duration;
use internal::framebuffer::{Framebuffer};
use internal::color::Color;
use internal::render::{render};
use internal::maze::{Maze, load_maze};
use internal::text::{render_text, render_text_thicker, render_text_with_outline};


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
    
    let mut audio = AudioPlayer::new("./assets/soundtrack.wav", 0.08);
    audio.play();
    
    show_welcome_screen(&mut window, &mut framebuffer);
    
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

        if process_events(&mut window, &mut player, &maze, 30, Vec2::new(20.0, 13.0)) {
            render_end_screen(&mut framebuffer, &mut window);
            break
        }

        let fps = calculate_fps(&mut last_frame_time);
        let fps_text = format!("FPS: {}", fps as u32);
        render_text_with_outline(&mut framebuffer, &font, &fps_text, 550, 380, 20.0, Color::new(255, 255, 255), Color::new(0, 0, 0), 2);

        window
         .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
         .unwrap();

        framebuffer.clear();
        std::thread::sleep(frame_delay)
    }
    audio.stop();
}


fn calculate_fps(last_frame_time: &mut Instant) -> f32 {
    let now = Instant::now();
    let duration = now.duration_since(*last_frame_time);
    *last_frame_time = now;

    1.0 / duration.as_secs_f32()
}

// Function to display the welcome screen
pub fn show_welcome_screen(window: &mut Window, framebuffer: &mut Framebuffer) {
    // Load the welcome image
    let img = image::open("assets/welcome.png").expect("Failed to load welcome image");
    let (img_width, img_height) = img.dimensions();

    // Scale the image to fit the screen
    let scale_x = framebuffer.width as f32 / img_width as f32;
    let scale_y = framebuffer.height as f32 / img_height as f32;
    let scale = scale_x.min(scale_y);

    // Render the image on the framebuffer
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let img_x = ((x as f32 / scale) as u32).min(img_width - 1);
            let img_y = ((y as f32 / scale) as u32).min(img_height - 1);
            let pixel = img.get_pixel(img_x, img_y);
            let color = Color::new(pixel[0], pixel[1], pixel[2]);
            framebuffer.set_current_color(color);
            framebuffer.draw_point(x, y);
        }
    }

    // Display the framebuffer
    window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
        .expect("Failed to update window with welcome screen");

    // Wait for any key press to start the game
    while !window.is_open() || !window.is_key_down(Key::A) {
        if window.is_key_down(Key::Escape) {
            break;
        }
        window.update();
    }
}

pub fn render_end_screen(framebuffer: &mut Framebuffer, window: &mut Window) {
    let end_image = include_bytes!("../assets/end.png");
    let img = image::load_from_memory(end_image).expect("Failed to load end screen image");

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let pixel = img.get_pixel(x as u32, y as u32);
            let color = Color::new(pixel[0], pixel[1], pixel[2]);
            framebuffer.set_current_color(color);
            framebuffer.draw_point(x, y);
        }
    }

    window.update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
        .expect("Failed to update window with welcome screen");

    // Wait for any key press to start the game
    while !window.is_open() || !window.is_key_down(Key::A) {
        if window.is_key_down(Key::Escape) {
            break;
        }
        window.update();
    }
}
