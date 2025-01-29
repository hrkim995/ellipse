extern crate sdl2;

use std::f32;

use sdl2::render::WindowCanvas;
use crate::game::math::{Vec2, Circle};

pub fn draw_circle(cvs: &mut WindowCanvas, c: Circle) {
    for i in 0..360 {
        let t1 = i as f32 * f32::consts::PI / 180.0;
        let t2 = (i + 1) as f32 * f32::consts::PI / 180.0;
        let p1 = c.center + Vec2 {x: f32::cos(t1), y: f32::sin(t1)} * c.r;
        let p2 = c.center + Vec2 {x: f32::cos(t2), y: f32::sin(t2)} * c.r;
        cvs.draw_line(p1.to_point(), p2.to_point()).unwrap();
    }

}