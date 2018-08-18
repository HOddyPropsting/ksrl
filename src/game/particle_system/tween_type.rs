#[derive(Clone, Copy)]
pub enum TweenType{
  Static,
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

impl TweenType{
  pub fn tween(&self, start : f32, change : f32, t : f32) -> f32{
    match self {
      TweenType::Static => { start },
      TweenType::Linear => { start + (t * change)},
      TweenType::QuadraticIn => { start + (t.powi(2) * change)},
      TweenType::CubicIn => { start + (t.powi(3) * change)},
      TweenType::QuadraticOut => { start + ((1f32-(1f32-t).powi(2)) * change)},
      TweenType::CubicOut => { start + ((1f32-(1f32-t).powi(3)) * change)},
    }
  }

  pub fn tween_to(&self, start : f32, end : f32, t : f32) -> f32{
    self.tween(start, end-start, t)
  }
}