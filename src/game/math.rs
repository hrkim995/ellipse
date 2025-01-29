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

// x^2 / a^2 + y^2 / b^2 = 1
pub struct Ellipse {
    pub center: Vec2,
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Ellipse {
    pub fn new(center: Vec2, a: f32, b: f32) -> Self {
        let c = f32::sqrt(f32::abs(a * a - b * b));
        Ellipse {center, a, b, c}
    }

    pub fn focus(&self) -> (Vec2, Vec2) {
        if self.a > self.b {
            (self.center + Vec2 {x: self.c, y: 0.0}, self.center + Vec2 {x: -self.c, y: 0.0}) // right, left
        } else { // don't think ellipse is circle
            (self.center + Vec2 {x: 0.0, y: -self.c}, self.center + Vec2 {x: self.c, y: 0.0}) // top, bottom
        }
    }
}
