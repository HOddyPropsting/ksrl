mod game;

fn main() {
  let mut g = game::Game::init();
  g.game_loop();
}