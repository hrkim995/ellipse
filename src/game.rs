extern crate sdl2;

mod math;
mod draw;

use draw::{draw_circle, draw_ellipse, draw_fiiled_circle};
use sdl2::pixels::Color;
use sdl2::{event::Event, keyboard::Keycode, Sdl};
use sdl2::render::WindowCanvas;
use std::time::{Duration, Instant};
use math::{Circle, Ellipse, Vec2};

pub struct Game {
    sc: Sdl,
    run: bool,
    cvs: WindowCanvas,
    t: Instant,
}

impl Game {
    pub fn new() -> Self {
        let sc = sdl2::init().unwrap();
        let vs = sc.video().unwrap();
        let win = vs.window("Ellipse", 600, 600)
            .position_centered()
            .build()
            .unwrap();
        let cvs = win.into_canvas().build().unwrap();
        let run = true;
        let t = Instant::now();
        Game {sc, cvs, run, t}
    }

    pub fn run(&mut self) {
        while self.run {
            self.input();
            self.update();
            self.output();
        }
    }

    pub fn quit(&mut self) {

    }

    pub fn input(&mut self) {
        let mut ep = self.sc.event_pump().unwrap();
        for e in ep.poll_iter() {
            match e {
                Event::Quit { .. } => { self.run = false; }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    self.run = false;
                }
                _ => {}
            }
        }
    }

    fn sleep(&mut self) {
        let fps = Duration::new(0, 1e9 as u32 / 60);
        let gap = Instant::now() - self.t;
        if fps > gap {
            std::thread::sleep(fps - gap);
        }
    }

    pub fn update(&mut self) {
        self.sleep();
        let mut dt = (Instant::now() - self.t).as_secs_f32();
        self.t = Instant::now();
        if dt > 0.05 {
            dt = 0.05;
        }
    }

    pub fn output(&mut self) {
        self.cvs.set_draw_color(Color::WHITE);
        self.cvs.clear();

        // draw
        self.cvs.set_draw_color(Color::BLACK);
        // draw_circle(&mut self.cvs, Circle {center: Vec2 {x: 300.0, y: 300.0}, r: 150.0});
        let e = Ellipse::new(Vec2 {x: 300.0, y: 300.0}, 200.0, 150.0);
        draw_ellipse(&mut self.cvs, &e);
        draw_fiiled_circle(&mut self.cvs, &Circle {center: e.focus().0, r: 3.0});
        draw_fiiled_circle(&mut self.cvs, &Circle {center: e.focus().1, r: 3.0});

        self.cvs.present();
    }
}