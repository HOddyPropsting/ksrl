pub mod emitter;
pub mod particle;
pub mod movement_strategy;

use tcod::colors::*;

pub mod Message {

  pub enum Message{
    None,
    RemoveSelf,
  }

}