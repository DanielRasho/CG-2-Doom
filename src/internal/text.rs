use rusttype::{Font, Scale, point};
use image::GenericImageView;
use super::framebuffer::Framebuffer;
use super::color::Color;

pub fn render_text(framebuffer: &mut Framebuffer, font: &Font, text: &str, x: usize, y: usize, scale: f32, color: Color) {
    let scale = Scale::uniform(scale);
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    for glyph in font.layout(text, scale, offset) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, gv| {
                let px = x as i32 + bounding_box.min.x + gx as i32;
                let py = y as i32 + bounding_box.min.y + gy as i32;

                if px >= 0 && py >= 0 && px < framebuffer.width as i32 && py < framebuffer.height as i32 {
                    let pixel_value = framebuffer.get_point_color(px as usize, py as usize);

                    // Blend the glyph's value with the framebuffer color
                    let blended_color = color * gv + pixel_value * (1.0 - gv);
                    framebuffer.set_current_color(blended_color);
                    framebuffer.draw_point(px as usize, py as usize);
                }
            });
        }
    }
}

pub fn render_text_thicker(framebuffer: &mut Framebuffer, font: &Font, text: &str, x: usize, y: usize, scale_factor: f32, color: Color, thickness: i32) {
    let scale = Scale::uniform(scale_factor);
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    // Render the text multiple times with slight offsets
    for dx in -thickness..=thickness {
        for dy in -thickness..=thickness {
            if dx == 0 && dy == 0 {
                continue; // Skip the original render position, we will render that separately
            }

            for glyph in font.layout(text, scale, offset) {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|gx, gy, gv| {
                        let px = x as i32 + bounding_box.min.x + gx as i32 + dx;
                        let py = y as i32 + bounding_box.min.y + gy as i32 + dy;

                        if px >= 0 && py >= 0 && px < framebuffer.width as i32 && py < framebuffer.height as i32 {
                            let pixel_value = framebuffer.get_point_color(px as usize, py as usize);

                            // Blend the glyph's value with the framebuffer color
                            let blended_color = color * gv + pixel_value * (1.0 - gv);
                            framebuffer.set_current_color(blended_color);
                            framebuffer.draw_point(px as usize, py as usize);
                        }
                    });
                }
            }
        }
    }

    // Render the original text (center)
    render_text(framebuffer, font, text, x, y, scale_factor, color);
}

pub fn render_text_with_outline(
    framebuffer: &mut Framebuffer,
    font: &Font,
    text: &str,
    x: usize,
    y: usize,
    scale: f32,
    text_color: Color,
    outline_color: Color,
    outline_thickness: i32,
) {
    let scale = Scale::uniform(scale);
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    // Render the outline
    for dx in -outline_thickness..=outline_thickness {
        for dy in -outline_thickness..=outline_thickness {
            if dx == 0 && dy == 0 {
                continue; // Skip the original render position, we'll render the main text separately
            }

            for glyph in font.layout(text, scale, offset) {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|gx, gy, gv| {
                        let px = x as i32 + bounding_box.min.x + gx as i32 + dx;
                        let py = y as i32 + bounding_box.min.y + gy as i32 + dy;

                        if px >= 0 && py >= 0 && px < framebuffer.width as i32 && py < framebuffer.height as i32 {
                            let pixel_value = framebuffer.get_point_color(px as usize, py as usize);

                            // Blend the glyph's value with the outline color
                            let blended_color = outline_color * gv + pixel_value * (1.0 - gv);
                            framebuffer.set_current_color(blended_color);
                            framebuffer.draw_point(px as usize, py as usize);
                        }
                    });
                }
            }
        }
    }

    // Render the main text on top
    for glyph in font.layout(text, scale, offset) {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|gx, gy, gv| {
                let px = x as i32 + bounding_box.min.x + gx as i32;
                let py = y as i32 + bounding_box.min.y + gy as i32;

                if px >= 0 && py >= 0 && px < framebuffer.width as i32 && py < framebuffer.height as i32 {
                    let pixel_value = framebuffer.get_point_color(px as usize, py as usize);

                    // Blend the glyph's value with the text color
                    let blended_color = text_color * gv + pixel_value * (1.0 - gv);
                    framebuffer.set_current_color(blended_color);
                    framebuffer.draw_point(px as usize, py as usize);
                }
            });
        }
    }
}