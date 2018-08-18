use game::particle_system::tween_type::TweenType;

pub enum MovementStrategy{
   Stationary,
   Physics(Physics),
   Tween(PositionTween),
}

impl Default for MovementStrategy{
  fn default() -> MovementStrategy{
    MovementStrategy::Stationary
  }
}

#[derive(Default)]
pub struct Physics{
  x  : f32,
  y  : f32,
  vx : f32,
  vy : f32,
  ax : f32,
  ay : f32,
}

impl Physics{
  pub fn update(&mut self, dt : f32) -> (f32,f32){
    self.vx += self.ax * dt;
    self.vy += self.ay * dt;
    self.x  += self.vx * dt;
    self.y  += self.vy * dt;
    return (self.x, self.y);
  }
}

#[derive(Default)]
pub struct PositionTween{
  pub start_x  : f32,
  pub change_x : f32,
  pub start_y  : f32,
  pub change_y : f32,
  pub current_time : f32,
  pub finish_time  : f32,
  pub tween_x : TweenType,
  pub tween_y : TweenType,
}

impl PositionTween {

  pub fn tween(&mut self, dt : f32) -> (f32, f32){
    self.current_time += dt;
    let fraction_done = self.current_time/self.finish_time;
    let x = self.tween_x.tween(self.start_x, self.change_x, fraction_done);
    let y = self.tween_y.tween(self.start_y, self.change_y, fraction_done);
    return (x,y);
  }
}



