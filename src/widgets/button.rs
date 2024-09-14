use taffy::{prelude::percent, NodeId, Point, Size, Style, TaffyResult, TaffyTree};

use super::{border::Border, Text, Widget};

pub struct Button {
    text: NodeId,
    pub border: Option<Border>,
}

impl Button {
    pub fn new(
        taffy: &mut TaffyTree<Box<dyn Widget>>,
        layout: taffy::Style,
        label: String,
    ) -> TaffyResult<NodeId> {
        let text = Text::new(
            taffy,
            Style {
                max_size: Size {
                    width: percent(100.),
                    height: percent(100.),
                },
                ..Default::default()
            },
            label,
        )?;

        let _self = Button { text, border: None };
        let node_id = taffy.new_leaf_with_context(layout, Box::new(_self))?;
        taffy.add_child(node_id, text)?;

        Ok(node_id)
    }
}

impl Widget for Button {
    fn render(
        &self,
        absolute_position: Point<f32>,
        layout: &taffy::Layout,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(border) = &self.border {
            println!("rendering border");
            border.render(absolute_position, layout, canvas)?;
        }

        Ok(())
    }
}
