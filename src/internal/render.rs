
use minifb::{Window, WindowOptions, Key};
use nalgebra_glm::Vec3;
use std::time::Duration;

use super::framebuffer::{self, Framebuffer, RenderableToFile};
use super::line::Line;
use super::color::Color;

pub fn render(framebuffer: &mut Framebuffer){
    // Clear framebuffer
    framebuffer.set_background_color_hex(0x333355);
    framebuffer.clear();
    
    framebuffer.set_current_color_hex(0xFFDDDD);
    framebuffer.draw_point(20, 40)
}