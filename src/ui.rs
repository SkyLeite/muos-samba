use sdl2::{gfx::primitives::DrawRenderer, pixels::Color, render::Canvas, video::Window};
use taffy::prelude::*;

use crate::{
    util,
    widgets::{Border, Button, Widget},
};

pub struct Context {}

pub struct Ui {
    tree: TaffyTree<Box<dyn Widget>>,
    root_node: NodeId,
    size: Size<AvailableSpace>,
    context: Context,
}

impl Ui {
    pub fn new() -> Result<Self, taffy::TaffyError> {
        let size = Size {
            width: length(640. - 1.),
            height: length(480. - 1.),
        };
        let mut taffy: TaffyTree<Box<dyn Widget>> = TaffyTree::new();

        let root_style = Style {
            size,
            display: Display::Grid,
            grid_template_columns: vec![length(250.0), fr(1.0), length(250.0)],
            grid_template_rows: vec![length(80.0), fr(1.0), length(80.0)],
            gap: Size {
                width: length(10.),
                height: length(10.),
            },
            ..Default::default()
        };

        // Define the child nodes
        let header_button = Button::new(
            &mut taffy,
            Style {
                size: Size {
                    width: length(50.),
                    height: length(50.),
                },
                ..Default::default()
            },
            "Help".to_string(),
        )?;

        let header = taffy.new_with_children(
            Style {
                grid_row: line(1),
                grid_column: span(3),
                border: Rect {
                    left: length(2.),
                    right: length(2.),
                    top: length(2.),
                    bottom: length(2.),
                },
                ..Default::default()
            },
            &[header_button],
        )?;

        let left_sidebar = taffy.new_leaf(Style {
            grid_row: line(2),
            grid_column: line(1),
            ..Default::default()
        })?;
        let content_area = taffy.new_leaf(Style {
            grid_row: line(2),
            grid_column: line(2),
            ..Default::default()
        })?;
        let right_sidebar = taffy.new_leaf(Style {
            grid_row: line(2),
            grid_column: line(3),
            ..Default::default()
        })?;
        let footer = taffy.new_leaf(Style {
            grid_row: line(3),
            grid_column: span(3),
            ..Default::default()
        })?;

        // Create the container with the children
        let root = taffy.new_with_children(
            root_style,
            &[header, left_sidebar, content_area, right_sidebar, footer],
        )?;

        Ok(Self {
            tree: taffy,
            root_node: root,
            size: Size {
                width: length(640.0 - 1.),
                height: length(480.0 - 1.),
            },
        })
    }

    fn compute(&mut self) -> Result<(), taffy::TaffyError> {
        self.tree.compute_layout(self.root_node, self.size)
    }

    pub fn render(
        &mut self,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.compute()?;
        self.tree.print_tree(self.root_node);

        self.render_node(self.root_node, canvas)?;

        Ok(())
    }

    pub fn render_node(
        &mut self,
        node: NodeId,
        canvas: &mut Canvas<Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for node in self.tree.children(node)? {
            let layout = self.tree.layout(node)?;

            let node_context = self.tree.get_node_context(node);
            if let Some(context) = node_context {
                let absolute_position = util::get_absolute_position(&self.tree, node)?;
                context.render(absolute_position, layout, canvas)?;
            }

            self.render_node(node, canvas)?;
        }

        Ok(())
    }
}
