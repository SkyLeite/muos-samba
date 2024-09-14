use sdl2::{gfx::primitives::DrawRenderer as _, pixels::Color};
use taffy::Point;

use super::Widget;

pub struct Border;

impl Widget for Border {
    fn render(
        &self,
        absolute_position: Point<f32>,
        layout: &taffy::Layout,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let location = absolute_position;
        let size = layout.size;

        canvas.line(
            location.x as i16,
            location.y as i16,
            (location.x + size.width) as i16,
            (location.y) as i16,
            Color::RGB(255, 255, 0),
        )?;

        // Left border
        canvas.line(
            location.x as i16,
            location.y as i16,
            (location.x) as i16,
            (location.y + size.height) as i16,
            Color::RGB(255, 255, 0),
        )?;

        // Bottom border
        canvas.line(
            location.x as i16,
            (location.y + size.height) as i16,
            (location.x + size.width) as i16,
            (location.y + size.height) as i16,
            Color::RGB(255, 255, 0),
        )?;

        // Right border
        canvas.line(
            (location.x + size.width) as i16,
            location.y as i16,
            (location.x + size.width) as i16,
            (location.y + size.height) as i16,
            Color::RGB(255, 255, 0),
        )?;

        Ok(())
    }
}
