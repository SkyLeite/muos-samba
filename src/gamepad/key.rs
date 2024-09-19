use crate::prelude::*;
use sdl2::controller::Button;

pub struct Key(egui::Key);

impl std::ops::Deref for Key {
    type Target = egui::Key;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Button> for Key {
    fn from(button: Button) -> Self {
        Key(match button {
            Button::A => egui::Key::Enter,
            Button::B => egui::Key::Escape,
            Button::X => egui::Key::X,
            Button::Y => egui::Key::Y,
            Button::Back => egui::Key::Y,
            Button::Guide => egui::Key::U,
            Button::Start => egui::Key::I,
            Button::LeftStick => egui::Key::O,
            Button::RightStick => egui::Key::P,
            Button::LeftShoulder => egui::Key::H,
            Button::RightShoulder => egui::Key::J,
            Button::DPadUp => egui::Key::ArrowUp,
            Button::DPadDown => egui::Key::ArrowDown,
            Button::DPadLeft => egui::Key::ArrowLeft,
            Button::DPadRight => egui::Key::ArrowRight,
            Button::Misc1 => todo!(),
            Button::Paddle1 => todo!(),
            Button::Paddle2 => todo!(),
            Button::Paddle3 => todo!(),
            Button::Paddle4 => todo!(),
            Button::Touchpad => todo!(),
        })
    }
}
