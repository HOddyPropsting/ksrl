extern crate tcod;
extern crate time;
use game::tcod::console::{Root,FontLayout};
use game::tcod::console::Console;
use game::tcod::input::{KeyCode};
use game::time::{Duration, PreciseTime};
use std::thread;
use std::time::Duration as StdDuration;

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
            self.particles.push(Emitter::new(20000, 1000, 20,20));
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
            let mut i = (self.particles.len()-1) as i32;
            while i >= 0 {
              match self.particles[i as usize].update(dt_ms){
                Message::None => {},
                Message::RemoveSelf => {self.particles.remove(i as usize);},
              };
              i -= 1;
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

pub enum Message{
  None,
  RemoveSelf,
}

trait Updateable{
  fn update(&mut self, dt : u32) -> Message;
}

trait Displayable{
  fn display(&self, console : &mut Root);
}

struct Particle {
  duration : u32,
  x : f32,
  y : f32,
  ax : f32,
  ay : f32,
  vx : f32,
  vy : f32,
  char : char,
}

impl Particle {
  fn new(duration : u32
    , x : f32
    , y : f32
    , ax : f32
    , ay : f32
    , vx : f32
    , vy : f32
    , char : char)-> Particle{
    Particle {
      duration : duration
    , x : x
    , y : y
    , ax : ax
    , ay : ay
    , vx : vx
    , vy : vy
    , char : char
    }
  }
}

impl Displayable for Particle {
  fn display(&self, console : &mut Root){
    if self.x > 0f32 && self.y > 0f32 {
      console.set_char(self.x.trunc() as i32, self.y.trunc() as i32, self.char);
    }
  }
}

impl Updateable for Particle {
  fn update(&mut self, dt : u32) -> Message{

    self.vx += self.ax;
    self.vy += self.ay;

    self.x += self.vx;
    self.y += self.vy;

    self.duration = self.duration.saturating_sub(dt);
    if self.duration == 0 {
      Message::RemoveSelf
    } else {
      Message::None
    }    
  }
}

struct Emitter {
  duration : u32,
  spawn_time : u32,
  st : u32,
  x : i32,
  y : i32,
  particles : Vec<Particle>,
}

impl Emitter {
  fn new(duration : u32, spawn_time : u32, x : i32, y : i32) -> Emitter{
    Emitter{
      duration : duration,
      spawn_time : spawn_time,
      st : spawn_time,
      x : x,
      y : y,
      particles : Vec::new()
    }
  }
}

impl Displayable for Emitter {
  fn display(&self, console : &mut Root){
    for p in &self.particles {
      p.display(console);
    }
  }
}

impl Updateable for Emitter {
  fn update(&mut self, dt : u32) -> Message{
    self.duration = self.duration.saturating_sub(dt);
    self.st = self.st.saturating_sub(dt);
    if self.st == 0 {
      self.particles.push(Particle::new( 5000,
        self.x as f32, self.y as f32, -0.005f32, 0.002f32, 0.4f32, 0.00f32, '@' ));
      self.st = self.spawn_time;
    }

    if self.particles.len() > 0 {
      let mut i = (self.particles.len()-1) as i32;
      while i >= 0 {
        match self.particles[i as usize].update(dt){
          Message::None => {},
          Message::RemoveSelf => {self.particles.remove(i as usize);},
        };
        i -= 1;
      }  
    }
    
    if self.duration == 0 {
       Message::RemoveSelf
    } else {
      Message::None
    }
  }
}
