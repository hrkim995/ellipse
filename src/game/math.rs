use std::ops;

use sdl2::rect::Point;

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Vec2 {
        Vec2 {x: rhs * self.x, y: rhs * self.y}
    }
}

impl Vec2 {
    pub fn to_point(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

pub struct Circle {
    pub center: Vec2,
    pub r: f32,
}
