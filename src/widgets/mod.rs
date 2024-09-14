use sdl2::{render::Canvas, video::Window};
use taffy::{Layout, Point};

mod border;
mod button;
mod text;

pub use border::Border;
pub use button::Button;
pub use text::Text;

pub trait Widget {
    fn render(
        &self,
        absolute_position: Point<f32>,
        layout: &Layout,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
