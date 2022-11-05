use bevy::prelude::Vec3;




struct Particle {
    position: Vec3,
    velocity: Vec3,
    
    // don't know the correct word for it.
    // holds values like mass, charge, etc.
    physical_values: Vec<f32>

}

struct ParticleForceRegistry {

    registrations: Vec<ParticleForceRegistration>,

}

struct ParticleForceRegistration {

  // same then the ID that they got registered.
  particle_type_id: usize,

  // force implementations that apply forces to the particles.
  // forces: Vec<dyn ParticleForce>,

}

trait ParticleForce : Sized {

  fn apply_force(&self, particle1: &Particle, particle2: &mut Particle, duration: f32);

}