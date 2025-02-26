use bevy::prelude::*;

use crate::rect::Rect;

const TOP_RIGHT: u8 = 0b0001;
const TOP_LEFT: u8 = 0b0010;
const BOTTOM_LEFT: u8 = 0b0100;
const BOTTOM_RIGHT: u8 = 0b1000;

#[derive(PartialEq, Clone, Debug)]
pub enum NodeState {
    Leaf(Vec<Entity>),
    Branch([Box<QuadTreeNode>; 4]),
}

#[derive(PartialEq, Clone, Debug)]
pub struct QuadTreeNode {
    pub bound: Rect,
    pub state: NodeState,
    pub depth: u8,
}

#[derive(Debug, Clone)]
pub struct QuadTreeConfig {
    pub rect: Rect,
    pub max_depth: u8,
    pub max_entities: u8,
}

impl QuadTreeNode {
    pub fn root(rect: Rect) -> Self {
        Self {
            depth: 0,
            state: NodeState::Leaf(vec![]),
            bound: rect,
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self.state {
            NodeState::Leaf(_) => true,
            NodeState::Branch(_) => false,
        }
    }

    // 获取物体所在的象限
    pub fn get_index(&self, rect: &Rect) -> u8 {
        if !self.bound.intersects(rect) {
            return 0;
        }
        let min = rect.min();
        let max = rect.max();

        let mut idx: u8 = 0;
        let is_left = min.x <= self.bound.center.x;
        let is_right = max.x >= self.bound.center.x;
        let is_top = max.y >= self.bound.center.y;
        let is_bottom = min.y <= self.bound.center.y;

        if is_left {
            if is_top {
                idx |= TOP_LEFT;
            }
            if is_bottom {
                idx |= BOTTOM_LEFT;
            }
        }
        if is_right {
            if is_top {
                idx |= TOP_RIGHT;
            }
            if is_bottom {
                idx |= BOTTOM_RIGHT;
            }
        }

        idx
    }

    pub fn insert(&mut self, entity: &Entity, rect: &Rect, config: &QuadTreeConfig) {
        if let NodeState::Leaf(entities) = &mut self.state {
            if entities.len() + 1 > config.max_entities.into() && self.depth < config.max_depth {
                self.split(rect, config);
                return;
            } else {
                entities.push(entity.clone());

            }
            return;
        }
        let idx = self.get_index(rect);
        if let NodeState::Branch(branches) = &mut self.state {
            if idx & TOP_RIGHT != 0 {
                branches[0].insert(entity, rect, config);
            }
            if idx & TOP_LEFT != 0 {
                branches[1].insert(entity, rect, config);
            }
            if idx & BOTTOM_LEFT != 0 {
                branches[2].insert(entity, rect, config);
            }
            if idx & BOTTOM_RIGHT != 0 {
                branches[3].insert(entity, rect, config);
            }
        }
    }

    pub fn split(&mut self, rect: &Rect, config: &QuadTreeConfig) {
        let half_width = self.bound.width / 2.0;
        let half_height = self.bound.height / 2.0;
        let quad_width = half_width / 2.0;
        let quad_height = half_height / 2.0;
        let center = self.bound.center;

        let sub_bounds = [
            Rect::new(
                Vec2::new(center.x + quad_width, center.y + quad_height),
                half_width,
                half_height,
            ),
            Rect::new(
                Vec2::new(center.x - quad_width, center.y + quad_height),
                half_width,
                half_height,
            ),
            Rect::new(
                Vec2::new(center.x - quad_width, center.y - quad_height),
                half_width,
                half_height,
            ),
            Rect::new(
                Vec2::new(center.x + quad_width, center.y - quad_height),
                half_width,
                half_height,
            ),
        ];

        let branches = sub_bounds.map(|sub_bound| {
            Box::new(QuadTreeNode {
                bound: sub_bound,
                state: NodeState::Leaf(vec![]),
                depth: self.depth + 1,
            })
        });

        let mut entities = vec![];

        if let NodeState::Leaf(datas) = &self.state {
            entities = datas.clone();
        }
        self.state = NodeState::Branch(branches);

        for entity in entities.iter().rev() {
            self.insert(entity, rect, config);
        }
    }

    // 获取节点内所有物体
    pub fn get_all_entities(&self) -> Vec<Entity> {
        if let NodeState::Leaf(entities) = &self.state {
            return entities.clone();
        }
        let mut entities = vec![];
        if let NodeState::Branch(branches) = &self.state {
            for branch in branches.iter() {
                entities.extend(branch.get_all_entities());
            }
        }
        entities
    }

    // 获取周围物体
    pub fn get_around_entity(&self, rect: &Rect) -> Vec<Entity> {
        if let NodeState::Leaf(entities) = &self.state {
            return entities.clone();
        }
        let idx = self.get_index(rect);
        let mut entities = vec![];
        if let NodeState::Branch(branches) = &self.state {
            if idx & TOP_RIGHT != 0 {
                entities.extend(branches[0].get_around_entity(rect));
            }
            if idx & TOP_LEFT != 0 {
                entities.extend(branches[1].get_around_entity(rect));
            }
            if idx & BOTTOM_LEFT != 0 {
                entities.extend(branches[2].get_around_entity(rect));
            }
            if idx & BOTTOM_RIGHT != 0 {
                entities.extend(branches[3].get_around_entity(rect));
            }
        }
        return entities;
    }

    pub fn clear(&mut self) {
        if let NodeState::Leaf(_) = &mut self.state {
            return;
        }
        if let NodeState::Branch(_) = self.state {
            self.state = NodeState::Leaf(vec![]);
        }
    }
}
