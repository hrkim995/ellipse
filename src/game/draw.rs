extern crate sdl2;

use std::f32;

use sdl2::render::WindowCanvas;
use crate::game::math::{Vec2, Circle, Ellipse};

pub fn draw_circle(cvs: &mut WindowCanvas, c: &Circle) {
    for i in 0..360 {
        let t1 = i as f32 * f32::consts::PI / 180.0;
        let t2 = (i + 1) as f32 * f32::consts::PI / 180.0;
        let p1 = c.center + Vec2 {x: f32::cos(t1), y: f32::sin(t1)} * c.r;
        let p2 = c.center + Vec2 {x: f32::cos(t2), y: f32::sin(t2)} * c.r;
        cvs.draw_line(p1.to_point(), p2.to_point()).unwrap();
    }
}

pub fn draw_fiiled_circle(cvs: &mut WindowCanvas, c: &Circle) {
    for y in -c.r as i32..(c.r + 1.0) as i32 {
        for x in -c.r as i32..(c.r + 1.0) as i32 {
            if x * x + y * y <= (c.r * c.r) as i32 {
                cvs.draw_point((c.center + Vec2 {x: x as f32, y: y as f32}).to_point()).unwrap();
            }
        }
    }
}

pub fn draw_ellipse(cvs: &mut WindowCanvas, e: &Ellipse) {
    for i in 0..360 {
        let t1 = i as f32 * f32::consts::PI / 180.0;
        let t2 = (i + 1) as f32 * f32::consts::PI / 180.0;
        let p1 = e.center + Vec2 {x: f32::cos(t1) * e.a, y: f32::sin(t1) * e.b};
        let p2 = e.center + Vec2 {x: f32::cos(t2) * e.a, y: f32::sin(t2) * e.b};
        cvs.draw_line(p1.to_point(), p2.to_point()).unwrap();
    }
}