extern crate tcod;
use game::tcod::console::{Root,FontLayout};
use game::tcod::console::Console;
use game::tcod::input::{KeyCode};

#[derive(Debug)]
pub enum GameState {
  InGame,
  Quit,
}

pub struct Game{
	state : GameState,
  console : Root,
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
      console : c
    }
  }

  pub fn game_loop(& mut self){
    'game_loop: loop {
      match self.state {
        GameState::InGame => {
          self.console.clear();
          self.console.flush();
          let k = self.console.wait_for_keypress(true);
          if k.code == KeyCode::Escape {
            self.state = GameState::Quit;
          }
        },
        GameState::Quit => {
          break 'game_loop;
        },
        _ => {},
      }  
    }    
  }
}