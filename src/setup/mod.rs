use bevy::prelude::*;
use quadtree::{
    rect::{Rect, Velocity},
    tree::QuadTree,
};
use rand::{Rng, rng};

pub fn setup(mut commands: Commands, mut quad_tree: ResMut<QuadTree>) {
    commands.spawn(Camera2d::default());

    let mut rng = rng();
    let num_entities = 100;

    for _ in 0..num_entities {
        let speed: f32 = rng.random_range(20.0..=80.0);
        let angle: f32 = rng.random_range(0.0..=360.0);
        let radian_angle = angle.to_radians();
        let velocity = Velocity {
            x: speed * radian_angle.cos(),
            y: speed * radian_angle.sin(),
        };
        let width: f32 = rng.random_range(5.0..=10.0);
        let height: f32 = rng.random_range(5.0..=10.0);
        let rect = Rect::new(
            Vec2::new(
                rng.random_range(-400.0..=400.0),
                rng.random_range(-400.0..=400.0),
            ),
            width,
            height,
        );

        let id = commands.spawn((rect, velocity)).id();

        let config = quad_tree.config.clone();
        quad_tree.root.insert(&id, &rect, &config);
    }
}
