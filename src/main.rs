mod game;

extern crate sdl2;

use game::Game;

pub fn main() {
    let mut game = Game::new();
    game.run();
    game.quit();
}