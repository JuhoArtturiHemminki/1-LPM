use crate::fractal_feedback::{PHI, H_C};

pub struct PressureModulator {
    pub ionization_potential: f64,
    pub particulate_repulsion: f64,
}

impl PressureModulator {
    pub const fn new() -> Self {
        Self {
            ionization_potential: 0.0,
            particulate_repulsion: 1.0,
        }
    }

    pub fn modulate_lattice(&mut self, air_jitter: f64, drift: &mut f64) 
-> f64 {
        let kinetic_sync = (air_jitter * PHI) / H_C;
        *drift += kinetic_sync * 0.22;
        self.ionization_potential = (kinetic_sync * 1000.0).abs();
        self.particulate_repulsion = 1.0 / (1.0 + (*drift).abs());
        self.particulate_repulsion
    }
}

