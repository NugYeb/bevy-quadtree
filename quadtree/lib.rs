pub mod rect;
pub mod system;
pub mod tree;

use bevy::prelude::*;
use rect::Rect;
use system::{
    check_collision_with_root, draw_entities_rect, draw_quadtree_nodes, move_entities,
    update_quadtree,
};
use tree::{QuadTree, node::QuadTreeConfig};

pub struct QuadTreePlugin;

impl Plugin for QuadTreePlugin {
    fn build(&self, app: &mut App) {
        let config = QuadTreeConfig {
            rect: Rect::new(Vec2::ZERO, 800.0, 800.0),
            max_depth: 4,
            max_entities: 4,
        };
        app.insert_resource(QuadTree::new(config)).add_systems(
            Update,
            (
                move_entities,
                check_collision_with_root,
                update_quadtree,
                draw_quadtree_nodes,
                draw_entities_rect,
            )
                .chain(),
        );
    }
}
