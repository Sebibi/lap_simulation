use crate::models::base_model::Model;
use crate::models::point_mass::{PointMass, PointMassState};
use crate::simulation::base_simulation::Simulation;
use crate::tracks::base_track::Track;
use crate::tracks::circle::CircleTrack;

pub struct OpenLoopSimulation {
    track: Option<CircleTrack>,
    model: Option<PointMass>,
    controls: (f64, f64),
}

impl OpenLoopSimulation {
    pub fn new() -> Self {
        Self {
            track: None,
            model: None,
            controls: (2.0, 0.4),
        }
    }

    pub fn with_controls(ax: f64, yaw_rate: f64) -> Self {
        Self {
            track: None,
            model: None,
            controls: (ax, yaw_rate),
        }
    }

    pub fn track(&self) -> Option<&CircleTrack> {
        self.track.as_ref()
    }

    pub fn model(&self) -> Option<&PointMass> {
        self.model.as_ref()
    }

    pub fn set_controls(&mut self, ax: f64, yaw_rate: f64) {
        self.controls = (ax, yaw_rate);
        if let Some(model) = self.model.as_mut() {
            model.set_controls(ax, yaw_rate);
        }
    }
}

impl Default for OpenLoopSimulation {
    fn default() -> Self {
        Self::new()
    }
}

impl Simulation for OpenLoopSimulation {
    type Track = CircleTrack;
    type Model = PointMass;

    fn init(&mut self, track: CircleTrack, mut model: PointMass) {
        model.init();
        let start_pos = track.get_start_position();
        model.set_position(start_pos.0, start_pos.1, start_pos.2);
        model.set_controls(self.controls.0, self.controls.1);
        self.track = Some(track);
        self.model = Some(model);
    }

    fn run(&mut self, dt: f64, duration: f64) -> Vec<PointMassState> {
        let model = self
            .model
            .as_mut()
            .expect("OpenLoopSimulation must be initialized before run");
        model.set_controls(self.controls.0, self.controls.1);

        let mut states = Vec::new();
        states.push(model.get_state().clone());

        if dt <= 0.0 || duration <= 0.0 {
            return states;
        }

        let steps = (duration / dt).floor() as usize;
        let mut current_time = 0.0f64;

        for _ in 0..steps {
            model.step(dt);
            current_time += dt;
            states.push(model.get_state().clone());
        }

        let remaining = duration - current_time;
        if remaining > 0.0 {
            model.step(remaining);
            states.push(model.get_state().clone());
        }

        states
    }

    fn reset(&mut self) {
        if let (Some(track), Some(model)) = (self.track.as_ref(), self.model.as_mut()) {
            model.reset();
            let start_pos = track.get_start_position();
            model.set_position(start_pos.0, start_pos.1, start_pos.2);
            model.set_controls(self.controls.0, self.controls.1);
        }
    }

    fn clean(&mut self) {
        self.track = None;
        self.model = None;
    }
}

#[cfg(test)]
mod tests {
    use super::OpenLoopSimulation;
    use crate::models::base_model::Model;
    use crate::models::point_mass::PointMass;
    use crate::simulation::base_simulation::Simulation;
    use crate::tracks::base_track::Track;
    use crate::tracks::circle::CircleTrack;

    #[test]
    fn test_open_loop_run_returns_states() {
        let track = CircleTrack::new(50.0, 10.0, 100);
        let model = PointMass::new();
        let mut sim = OpenLoopSimulation::new();
        sim.init(track, model);

        let states = sim.run(0.1, 0.25);
        assert_eq!(states.len(), 4);
    }

    #[test]
    fn test_open_loop_reset_returns_to_start() {
        let track = CircleTrack::new(50.0, 10.0, 100);
        let start_pos = track.get_start_position();
        let model = PointMass::new();
        let mut sim = OpenLoopSimulation::new();
        sim.init(track, model);

        let _ = sim.run(0.1, 0.5);
        sim.reset();

        let model = sim.model().expect("model missing after reset");
        let (x, y, yaw) = model.get_position();
        assert!((x - start_pos.0).abs() < 1e-9);
        assert!((y - start_pos.1).abs() < 1e-9);
        assert!((yaw - start_pos.2).abs() < 1e-9);
    }
}
