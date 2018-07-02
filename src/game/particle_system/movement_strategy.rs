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

pub enum TweenType{
  Linear,
  QuadraticIn,
  CubicIn,
  QuadraticOut,
  CubicOut,
}

impl Default for TweenType{
  fn default() -> TweenType{
    TweenType::Linear
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

  fn tween_internal(t : f32, start : f32, change:f32, tween_type: &TweenType) -> f32{
    match tween_type{
      TweenType::Linear => { start + (t * change)},
      TweenType::QuadraticIn => { start + (t.powi(2) * change)},
      TweenType::CubicIn => { start + (t.powi(3) * change)},
      TweenType::QuadraticOut => { start + ((1f32-(1f32-t).powi(2)) * change)},
      TweenType::CubicOut => { start + ((1f32-(1f32-t).powi(3)) * change)},
    }
  }

  pub fn tween(&mut self, dt : f32) -> (f32, f32){
    self.current_time += dt;
    let t = self.current_time/self.finish_time;
    let x = PositionTween::tween_internal(t, self.start_x, self.change_x, &self.tween_x);
    let y = PositionTween::tween_internal(t, self.start_y, self.change_y, &self.tween_y);
    return (x,y);
  }
}



