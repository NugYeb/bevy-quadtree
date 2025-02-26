use bevy::prelude::*;

#[derive(Component, Default, PartialEq, Clone, Copy, Debug)]
pub struct Rect {
    pub center: Vec2,
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Default, PartialEq, Clone, Copy, Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Rect {
    pub fn new(center: Vec2, width: f32, height: f32) -> Self {
        Self {
            center,
            width,
            height,
        }
    }

    pub fn draw(&self, gizmos: &mut Gizmos, color: Color) {
        let min = self.min();
        let max = self.max();

        gizmos.linestrip_2d(
            [
                Vec2::new(min.x, min.y),
                Vec2::new(min.x, max.y),
                Vec2::new(max.x, max.y),
                Vec2::new(max.x, min.y),
                Vec2::new(min.x, min.y),
            ],
            color,
        );
    }

    pub fn min(&self) -> Vec2 {
        self.center - Vec2::new(self.width / 2.0, self.height / 2.0)
    }
    pub fn max(&self) -> Vec2 {
        self.center + Vec2::new(self.width / 2.0, self.height / 2.0)
    }

    pub fn contains(&self, other: &Rect) -> bool {
        let self_min = self.min();
        let self_max = self.max();
        let other_min = other.min();
        let other_max = other.max();

        self_min.x <= other_min.x && self_min.y <= other_min.y &&
        self_max.x >= other_max.x && self_max.y >= other_max.y
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        let self_min = self.min();
        let self_max = self.max();
        let other_min = other.min();
        let other_max = other.max();

        !(self_max.x < other_min.x || self_min.x > other_max.x ||
          self_max.y < other_min.y || self_min.y > other_max.y)
    }
}
