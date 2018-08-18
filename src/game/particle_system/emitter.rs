use super::particle::*;
use tcod::console::Root;
use tcod::colors::Color;
use super::Message::*;
use super::tween_type::*;
use super::movement_strategy::*;


pub enum ParticleType {
    Empty,
    Rocket,
    Smoke,
    BigSmoke,
    Explosion,
}

impl Default for ParticleType{
  fn default() -> Self{
    ParticleType::Empty
  }
}


#[derive(Default)]
pub struct Emitter {
  pub duration : u32,
  pub st : u32,
  pub tween : TweenType,
  pub x : f32,
  pub y : f32,
  pub particles : Vec<Particle>,
}

impl Emitter {
  pub fn new() -> EmitterBuilder{
    let mut emitter_builder = EmitterBuilder{..Default::default()};
    emitter_builder
  }

  pub fn display(&self, console : &mut Root){
    for p in &self.particles {
      p.display(console);
    }
  }

  pub fn update(&mut self, dt : u32) -> Message{
    self.duration = self.duration.saturating_sub(dt);
    self.st = self.st.saturating_sub(dt);
    let bg = Color{ r: 10, g: 10, b: 10};
    let fg = Color{ r: 150, g: 10, b: 10};

    // TODO: re-implement particle spawning system


    // TODO: move particles out of emitter and into a single owner?
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

#[derive(Default)]
pub struct EmitterBuilder{
  from_x : f32,
  from_y : f32,
  to_x : f32,
  to_y : f32,
  tween : TweenType,
  in_seconds : f32,
  duration : f32,
  movement : Vec<(f32,f32,f32,f32,f32,TweenType)>,
  every_timeline : Vec<(f32,ParticleType)>,
  once_timeline : Vec<(f32,ParticleType)>,
  on_death : ParticleType,
}

impl EmitterBuilder{
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

  pub fn spawn_every(mut self, seconds : f32, emitter : ParticleType) -> Self{
    self.every_timeline.push((seconds,emitter));
    self
  }

  pub fn spawn_once(mut self, seconds : f32, emitter : ParticleType) -> Self{
    self.once_timeline.push((seconds,emitter));
    self
  }

  pub fn on_death(mut self, emitter : ParticleType) -> Self{
    self.on_death = emitter;
    self
  }

  pub fn build(mut self) -> Emitter{
    self.then();
    Emitter::default()
    // need to finish this.
  }

  pub fn test_builder() -> Emitter {
    let mut emitter_builder = EmitterBuilder{..Default::default()};
    emitter_builder
    .from(0f32,0f32)
    .to(50f32,50f32)
    .to(50f32,50f32)
    .in_seconds(2f32)
    .then()
    .for_units(-20f32,50f32)
    .build()
  }
}