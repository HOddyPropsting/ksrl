use super::particle::*;
use tcod::console::Root;
use tcod::colors::Color;
use tcod::colors::*;
use super::Message::*;
use super::movement_strategy::*;

#[derive(Default)]
pub struct Emitter {
  pub duration : u32,
  pub spawn_time : u32,
  pub st : u32,
  pub x : f32,
  pub y : f32,
  pub particles : Vec<Particle>,
}

impl Emitter {
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
    if self.st == 0 {
      self.particles.push(
        Particle{
          duration : 1000,
          x : self.x,
          y : self.y,
          bg : bg,
          fg : fg,
          char : '@', 
          movement_strategy : MovementStrategy::Tween(
            PositionTween{
              start_x : self.x, 
              start_y : self.y, 
              change_x : 50.0f32, 
              change_y : 50.0f32, 
              tween_x : TweenType::QuadraticIn, 
              tween_y : TweenType::CubicOut,
              finish_time: 1000f32, 
              ..Default::default()
            }
          ),
          ..Default::default()
        });
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