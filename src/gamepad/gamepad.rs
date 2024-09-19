use crate::prelude::*;
use egui_sdl2_gl::EguiStateHandler;
use sdl2::{event::Event, Sdl};

use super::Key;

pub fn init(sdl_context: &Sdl) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize controllers
    let game_controller_subsystem = sdl_context.game_controller()?;
    let available = game_controller_subsystem.num_joysticks()?;

    println!("{} joysticks available", available);

    // Iterate over all available joysticks and look for game controllers.
    (0..available).find_map(|id| {
        if !game_controller_subsystem.is_game_controller(id) {
            println!("{} is not a game controller", id);
            return None;
        }

        println!("Attempting to open controller {}", id);

        match game_controller_subsystem.open(id) {
            Ok(c) => {
                // We managed to find and open a game controller,
                // exit the loop
                println!("Success: opened \"{}\"", c.name());
                Some(c)
            }
            Err(e) => {
                println!("failed: {:?}", e);
                None
            }
        }
    });

    Ok(())
}

pub fn handle(event: &Event, egui_state: &mut EguiStateHandler) {
    match event {
        Event::ControllerButtonDown { button, .. } => {
            egui_state.input.events.push(egui::Event::Key {
                key: *Key::from(*button),
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            });
        }
        _ => {}
    }
}
