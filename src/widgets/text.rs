use taffy::{prelude::length, NodeId, Point, Size, Style, TaffyResult, TaffyTree};

use super::Widget;

pub struct Text {
    label: String,
}

impl Text {
    pub fn new(
        taffy: &mut TaffyTree<Box<dyn Widget>>,
        layout: taffy::Style,
        label: String,
    ) -> TaffyResult<NodeId> {
        let _self = Text { label };
        taffy.new_leaf_with_context(
            Style {
                display: taffy::Display::Block,
                size: Size {
                    width: length(100.),
                    height: length(50.),
                },
                ..Default::default()
            },
            Box::new(_self),
        )
    }
}

impl Widget for Text {
    fn render(
        &self,
        absolute_position: Point<f32>,
        layout: &taffy::Layout,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        dbg!(absolute_position);

        Ok(())
    }
}
