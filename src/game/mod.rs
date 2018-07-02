mod particle_system;

extern crate time;
use tcod::console::{Root,FontLayout};
use game::particle_system::emitter::Emitter;
use tcod::console::Console;
use tcod::input::{KeyCode};
use game::time::{PreciseTime};
use std::thread;
use std::time::Duration as StdDuration;
use game::particle_system::Message::Message;

#[derive(Debug)]
pub enum GameState {
  InGame,
  Animating,
  Quit,
}

pub struct Game{
	state : GameState,
  console : Root,
  particles : Vec<Emitter>,
}

impl Game {

  pub fn init() -> Game{
    let (width,height) = (80,80);
    let mut c = Root::initializer()
      .size(width,height)
      .font_dimensions(16,16)
      .font("terminal.png", FontLayout::AsciiInCol)
      .fullscreen(false)
      .init();

    Game{ 
      state : GameState::InGame,
      console : c,
      particles : Vec::new(),
    }
  }

  pub fn game_loop(& mut self){
    let mut prev = PreciseTime::now();

    'game_loop: loop {
      match self.state {
        GameState::InGame => {
          self.console.clear();
          self.console.flush();
          let k = self.console.wait_for_keypress(true);
          if k.code == KeyCode::Enter {
            prev = PreciseTime::now();
            self.particles.push(Emitter{ duration : 5000, spawn_time : 200, x : 20f32, y : 20f32, ..Default::default()});
            self.state = GameState::Animating;
          }
          if k.code == KeyCode::Escape {
            self.state = GameState::Quit;
          }
        },
        GameState::Animating => {
          let now = PreciseTime::now();
          let dt = prev.to(now);
          let mut dt_ms : u32;
          if dt.num_milliseconds() < 0 {
            continue 'game_loop;
          } else {
              dt_ms = dt.num_milliseconds() as u32;
          }
          if self.particles.len() > 0 {
            for i in self.particles.len()-1..=0 {
              match self.particles[i].update(dt_ms){
                Message::None => {},
                Message::RemoveSelf => {self.particles.remove(i);},
              };
            }
          }
          self.console.clear();
          for p in &self.particles {
            p.display(&mut self.console);
          }
          self.console.flush();
          prev = now;
          if self.particles.len() == 0 {
            self.state = GameState::InGame;
          }
          thread::sleep(StdDuration::from_millis((16) as u64));
        },
        GameState::Quit => {
          break 'game_loop;
        },
      }
    }
  }
}

