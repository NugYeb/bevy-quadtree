pub mod node;

use bevy::prelude::*;
use node::{QuadTreeNode, QuadTreeConfig};


#[derive(Resource)]
pub struct QuadTree {
    pub root: QuadTreeNode,
    pub config: QuadTreeConfig,
}

impl QuadTree {
    pub fn new(config: QuadTreeConfig) -> Self {
        QuadTree {
            root: QuadTreeNode::root(config.rect.clone()),
            config,
        }
    }
}
