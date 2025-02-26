use bevy::prelude::*;

use crate::{
    rect::{Rect, Velocity},
    tree::{
        QuadTree,
        node::{NodeState, QuadTreeConfig, QuadTreeNode},
    },
};

pub fn draw_quadtree_nodes(mut gizmos: Gizmos, quad_tree: Res<QuadTree>) {
    fn draw_node(gizmos: &mut Gizmos, node: &QuadTreeNode) {
        node.bound.draw(gizmos, Color::srgba(0.4, 0.4, 0.4, 0.5));

        if let NodeState::Branch(branches) = &node.state {
            for branch in branches {
                draw_node(gizmos, branch);
            }
        }
    }

    draw_node(&mut gizmos, &quad_tree.root);
}

pub fn draw_entities_rect(mut gizmos: Gizmos, rect: Query<&Rect>) {
    for rect in rect.iter() {
        rect.draw(&mut gizmos, Color::srgba(0.8, 0.8, 0.8, 0.5));
    }
}

pub fn move_entities(mut query: Query<(&mut Rect, &Velocity)>, time: Res<Time>) {
    for (mut rect, velocity) in query.iter_mut() {
        rect.center.x += velocity.x * time.delta_secs();
        rect.center.y += velocity.y * time.delta_secs();
    }
}

pub fn check_collision_with_root(
    mut query: Query<(&mut Rect, &mut Velocity)>,
    quad_tree: Res<QuadTree>,
) {
    let root = &quad_tree.root.bound;
    for (rect, mut velocity) in query.iter_mut() {
        if rect.center.x - rect.width / 2.0 < root.center.x - root.width / 2.0 {
            velocity.x = velocity.x.abs();
        } else if rect.center.x + rect.width / 2.0 > root.center.x + root.width / 2.0 {
            velocity.x = -velocity.x.abs();
        }

        if rect.center.y - rect.height / 2.0 < root.center.y - root.height / 2.0 {
            velocity.y = velocity.y.abs();
        } else if rect.center.y + rect.height / 2.0 > root.center.y + root.height / 2.0 {
            velocity.y = -velocity.y.abs();
        }
    }
}


// 有问题的实现，需要更改，不会
pub fn update_quadtree(
    mut quad_tree: ResMut<QuadTree>,
    querys: Query<(Entity, &Rect), With<Velocity>>,
) {
    let mut re_insert = vec![];
    let config = quad_tree.config.clone();

    update_node(&mut quad_tree.root, &config, &querys, &mut re_insert);

    fn update_node(
        node: &mut QuadTreeNode,
        config: &QuadTreeConfig,
        querys: &Query<(Entity, &Rect), With<Velocity>>,
        re_insert: &mut Vec<(Entity, Rect)>,
    ) {
        if let NodeState::Leaf(entities) = &mut node.state {
            let mut i = 0;
            while i < entities.len() {
                if let Ok((_, rect)) = querys.get(entities[i]) {
                    if !node.bound.contains(rect) {
                        // 检查实体是否已经存在于 re_insert 列表中
                        if !re_insert.iter().any(|&(e, _)| e == entities[i]) {
                            re_insert.push((entities[i], *rect));
                        }
                        entities.remove(i);
                    } else {
                        i += 1;
                    }
                } else {
                    entities.remove(i);
                }
            }
        } else if let NodeState::Branch(branches) = &mut node.state {
            for branch in branches {
                update_node(branch, config, querys, re_insert);
            }
        }
    }

    merge_nodes(&mut quad_tree.root, &config, &querys, &mut re_insert);

    fn merge_nodes(
        node: &mut QuadTreeNode,
        config: &QuadTreeConfig,
        querys: &Query<(Entity, &Rect), With<Velocity>>,
        re_insert: &mut Vec<(Entity, Rect)>,
    ) {
        let entities = node.get_all_entities();
        if let NodeState::Branch(branches) = &mut node.state {
            if entities.len() <= config.max_entities as usize {
                node.clear();
                for entity in entities {
                    if let Ok((_, rect)) = querys.get(entity) {
                        if !re_insert.iter().any(|&(e, _)| e == entity) {
                            re_insert.push((entity, *rect));
                        }
                    }
                }
            } else {
                for branch in branches.iter_mut() {
                    merge_nodes(branch, config, querys, re_insert);
                }
            }
        }
    }

    for (entity, rect) in re_insert.iter().rev() {
        quad_tree.root.insert(entity, rect, &config);
    }
}
