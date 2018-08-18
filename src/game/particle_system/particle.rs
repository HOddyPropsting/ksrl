use tcod::colors::Color;
use tcod::console::Root;
use tcod::console::Console;
use super::Message::Message;
use super::movement_strategy::*;
use super::tween_type::*;


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

  pub fn new() -> ParticleBuilder{
    ParticleBuilder::default()
  }

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
      MovementStrategy::Tween(t) => {let (x,y) = t.tween(dt as f32); self.x = x; self.y = y;},
    }
    if self.duration == 0 {
      Message::RemoveSelf
    } else {
      Message::None
    }    
  }
}

#[derive(Default)]
pub struct ParticleBuilder {
  from_x : f32,
  from_y : f32,
  to_x : f32,
  to_y : f32,
  tween : TweenType,
  in_seconds : f32,
  duration : f32,
  movement : Vec<(f32,f32,f32,f32,f32,TweenType)>,
}

// TODO: this code is a copy of the one in emitter, perhaps I can abstract it into a generic movement / path idea?
// perhaps a generic builder<Particle>?
impl ParticleBuilder{
  pub fn from(mut self, x : f32, y : f32) -> Self{
    self.from_x = x;
    self.from_y = y;
    self
  }

  pub fn for_units(mut self, x : f32, y : f32) -> Self{
    self.to_x = self.from_x + x;
    self.to_y = self.from_y + y;
    self
  }

  pub fn to(mut self, x : f32, y : f32) -> Self{
    self.to_x = x;
    self.to_y = y;
    self
  }

  pub fn in_seconds(mut self, t : f32) -> Self{
    self.in_seconds = t;
    self
  }

  pub fn then(mut self) -> Self{
    self.movement.push((self.from_x,self.from_y,self.to_x,self.to_y,self.in_seconds,self.tween));
    self.from_x = self.to_x;
    self.from_y = self.to_y;
    self.duration += self.in_seconds;
    self
  }

  pub fn tween(mut self, tween : TweenType) -> Self{
    self.tween = tween;
    self
  }

  pub fn build(mut self) -> Particle{
    Particle::default()
  }
}