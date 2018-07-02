use tcod::colors::Color;
use tcod::console::Root;
use tcod::console::Console;
use super::Message::Message;
use super::movement_strategy::*;


#[derive(Default)]
pub struct Particle {
  pub duration : u32,
  pub x : f32,
  pub y : f32,
  pub movement_strategy : MovementStrategy,
  pub bg : Color,
  pub fg : Color,
  pub char : char,
}

impl Particle {

  pub fn display(&self, console : &mut Root){
    if self.x > 0f32 && self.y > 0f32 && self.x < console.width() as f32 && self.y < console.height() as f32 {
      console.put_char_ex(self.x.trunc() as i32, self.y.trunc() as i32, self.char, self.fg, self.bg);
    }
  }

  pub fn update(&mut self, dt : u32) -> Message{
    self.duration = self.duration.saturating_sub(dt);
    match &mut self.movement_strategy{
      MovementStrategy::Stationary => {},
      MovementStrategy::Physics(p) => {let (x,y) = p.update(dt as f32); self.x = x; self.y = y;},
      MovementStrategy::Tween(t) => { let (x,y) = t.tween(dt as f32); self.x = x; self.y = y;},
    }
    if self.duration == 0 {
      Message::RemoveSelf
    } else {
      Message::None
    }    
  }
}