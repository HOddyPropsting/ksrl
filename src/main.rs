mod game;
extern crate tcod;

fn main() {
  let mut g = game::Game::init();
  g.game_loop();
}