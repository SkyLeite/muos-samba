use taffy::{NodeId, Point, TaffyTree};

use crate::widgets::Widget;

pub fn get_absolute_position(
    taffy: &TaffyTree<Box<dyn Widget>>,
    node: NodeId,
) -> Result<Point<f32>, Box<dyn std::error::Error>> {
    let mut parent_node: Option<NodeId> = taffy.parent(node);
    let mut location = taffy.layout(node)?.location.clone();

    while let Some(parent) = parent_node {
        let parent_location = taffy.layout(parent)?.location;
        location.x += parent_location.x;
        location.y += parent_location.y;

        parent_node = taffy.parent(parent);
    }

    Ok(location)
}
