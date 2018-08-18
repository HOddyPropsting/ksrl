pub mod emitter;
pub mod particle;
pub mod tween_type;
pub mod movement_strategy;
use self::particle::Particle;
use self::emitter::Emitter;
use self::emitter::EmitterBuilder;

pub mod Message {

  pub enum Message{
    None,
    RemoveSelf,
  }

}

// TODO: make this the parent / entry point for keeping track of particles and emitters. So that the main.rs doesn't have to think about it.
struct ParticleSystem{
  emitters : Vec<Emitter>,
  particles : Vec<Particle>,
}

impl ParticleSystem{
  fn update(&self, dt : u32){

  }
}