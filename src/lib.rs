use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_wasm_bindgen;
use js_sys;
use web_sys::console;

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn main_js() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub mass: f64,
}

#[wasm_bindgen]
pub struct Simulation {
    particles: Vec<Particle>,
    dt: f64,
    surface_tension: f64,
}

#[wasm_bindgen]
impl Simulation {
    #[wasm_bindgen(constructor)]
    pub fn new(num_particles: usize, width: f64, height: f64) -> Simulation {
        let mut particles = Vec::with_capacity(num_particles);
        for _ in 0..num_particles {
            particles.push(Particle {
                x: js_sys::Math::random() * width,
                y: js_sys::Math::random() * height,
                vx: (js_sys::Math::random() - 0.5) * 10.0, // Increased initial velocity
                vy: (js_sys::Math::random() - 0.5) * 10.0,
                mass: 1.0,
            });
        }
        Simulation {
            particles,
            dt: 0.05, // Increased time step
            surface_tension: 10.0, // Amplified surface tension
        }
    }

    fn powi(base: f64, exp: i32) -> f64 {
        base.powi(exp)
    }

    pub fn step(&mut self) {
        let len = self.particles.len();
        let epsilon = 1.0; // Depth of the potential well
        let sigma = 10.0; // Finite distance at which the inter-particle potential is zero

        for i in 0..len {
            for j in (i + 1)..len {
                let dx = self.particles[j].x - self.particles[i].x;
                let dy = self.particles[j].y - self.particles[i].y;
                let distance_sq = dx * dx + dy * dy + 0.01; // Avoid division by zero
                let distance = distance_sq.sqrt();

                // Lennard-Jones force
                let force_scalar = 48.0 * epsilon * (Self::powi(sigma / distance, 12) - 0.5 * Self::powi(sigma / distance, 6)) / distance_sq;

                let fx = force_scalar * dx;
                let fy = force_scalar * dy;

                self.particles[i].vx += fx * self.dt / self.particles[i].mass;
                self.particles[i].vy += fy * self.dt / self.particles[i].mass;
                self.particles[j].vx -= fx * self.dt / self.particles[j].mass;
                self.particles[j].vy -= fy * self.dt / self.particles[j].mass;
            }
        }

        // Update positions based on velocities
        for p in &mut self.particles {
            p.x += p.vx * self.dt;
            p.y += p.vy * self.dt;

            // Simple boundary conditions
            if p.x < 0.0 {
                p.x = 0.0;
                p.vx *= -0.5;
            }
            if p.x > 800.0 {
                p.x = 800.0;
                p.vx *= -0.5;
            }
            if p.y < 0.0 {
                p.y = 0.0;
                p.vy *= -0.5;
            }
            if p.y > 600.0 {
                p.y = 600.0;
                p.vy *= -0.5;
            }
        }

        // Log the first particle's position and velocity for debugging
        if let Some(first) = self.particles.first() {
            let log_message = format!(
                "Particle 0 - Position: ({:.2}, {:.2}), Velocity: ({:.2}, {:.2})",
                first.x, first.y, first.vx, first.vy
            );
            console::log_1(&log_message.into());
        }
    }

    pub fn get_particles(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.particles).unwrap_or(JsValue::NULL)
    }

    pub fn set_dt(&mut self, dt: f64) {
        self.dt = dt;
    }

    pub fn set_surface_tension(&mut self, st: f64) {
        self.surface_tension = st;
    }
}
