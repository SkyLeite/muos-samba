extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{self, Color};

use sdl2::gfx::primitives::DrawRenderer;

mod ui;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut ui = ui::Ui::new()?;
    let mut events = sdl_context.event_pump()?;

    'main: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                // Event::KeyDown {
                //     keycode: Some(keycode),
                //     ..
                // } => {
                //     if keycode == Keycode::Escape {
                //         break 'main;
                //     } else if keycode == Keycode::Space {
                //         println!("space down");
                //         for i in 0..400 {
                //             canvas.pixel(i as i16, i as i16, 0xFF000FFu32)?;
                //         }
                //     }
                // }

                // Event::MouseButtonDown { x, y, .. } => {
                //     let color = pixels::Color::RGB(x as u8, y as u8, 255);
                //     let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                //     lastx = x as i16;
                //     lasty = y as i16;
                //     println!("mouse btn down at ({},{})", x, y);
                // }
                _ => {}
            }
        }

        ui.render(&mut canvas)?;
        canvas.present();
    }

    Ok(())
}
