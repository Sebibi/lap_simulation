use crate::controllers::base_controller::{
    ControlInput, Controller, find_nearest_point_index, normalize_angle,
};
use crate::models::base_model::Model;
use crate::models::point_mass::PointMass;
use crate::tracks::base_track::Track;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PurePursuitParameters {
    pub lookahead_distance: f64,
    pub gain: f64,
    pub ax: f64,
}

impl PurePursuitParameters {
    pub fn new(lookahead_distance: f64, gain: f64, ax: f64) -> Self {
        Self {
            lookahead_distance,
            gain,
            ax,
        }
    }
}

impl Default for PurePursuitParameters {
    fn default() -> Self {
        Self {
            lookahead_distance: 5.0,
            gain: 1.0,
            ax: 0.0,
        }
    }
}

impl fmt::Display for PurePursuitParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PurePursuitParameters {{ lookahead_distance: {:.3} m, gain: {:.3}, ax: {:.3} m/s² }}",
            self.lookahead_distance, self.gain, self.ax
        )
    }
}

pub struct PurePursuitController<T: Track> {
    track: Option<T>,
    parameters: PurePursuitParameters,
    initial_parameters: PurePursuitParameters,
}

impl<T: Track> PurePursuitController<T> {
    pub fn new(parameters: PurePursuitParameters) -> Self {
        Self {
            track: None,
            parameters: parameters.clone(),
            initial_parameters: parameters,
        }
    }

    pub fn track(&self) -> Option<&T> {
        self.track.as_ref()
    }

    pub fn parameters(&self) -> &PurePursuitParameters {
        &self.parameters
    }

    fn find_lookahead_point(&self, x: f64, y: f64) -> Option<(f64, f64)> {
        let track = self.track.as_ref()?;
        let center_line = track.get_center_line();
        let nearest_index = find_nearest_point_index(center_line, x, y)?;

        let n = center_line.len();
        if n == 0 {
            return None;
        }

        for offset in 0..n {
            let index = (nearest_index + offset) % n;
            let (tx, ty) = center_line[index];
            let dx = tx - x;
            let dy = ty - y;
            let distance = (dx * dx + dy * dy).sqrt();
            if distance >= self.parameters.lookahead_distance {
                return Some((tx, ty));
            }
        }

        Some(center_line[nearest_index])
    }
}

impl<T: Track> Default for PurePursuitController<T> {
    fn default() -> Self {
        Self::new(PurePursuitParameters::default())
    }
}

impl<T: Track> Controller for PurePursuitController<T> {
    type Model = PointMass;
    type Track = T;
    type Parameters = PurePursuitParameters;

    fn init(&mut self, _model: &Self::Model, track: Self::Track, parameters: Self::Parameters) {
        self.track = Some(track);
        self.parameters = parameters.clone();
        self.initial_parameters = parameters;
    }

    fn step(&self, model: &Self::Model) -> ControlInput {
        let (x, y, yaw) = model.get_position();
        let (tx, ty) = match self.find_lookahead_point(x, y) {
            Some(point) => point,
            None => return ControlInput::new(0.0, 0.0),
        };

        let target_yaw = (ty - y).atan2(tx - x);
        let heading_error = normalize_angle(target_yaw - yaw);
        let yaw_rate = self.parameters.gain * heading_error;

        ControlInput::new(self.parameters.ax, yaw_rate)
    }

    fn reset(&mut self) {
        self.track = None;
        self.parameters = self.initial_parameters.clone();
    }
}

impl<T: Track> fmt::Display for PurePursuitController<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.track.as_ref() {
            Some(track) => write!(
                f,
                "PurePursuitController {{ track: {}, parameters: {} }}",
                track.get_track_name(),
                self.parameters
            ),
            None => write!(
                f,
                "PurePursuitController {{ track: uninitialized, parameters: {} }}",
                self.parameters
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PurePursuitController, PurePursuitParameters};
    use crate::controllers::base_controller::Controller;
    use crate::models::base_model::Model;
    use crate::models::point_mass::PointMass;
    use crate::tracks::base_track::Track;
    use crate::tracks::circle::CircleTrack;

    #[test]
    fn test_pure_pursuit_controller_yaw_rate_positive() {
        let track = CircleTrack::new(50.0, 10.0, 180);
        let start = track.get_start_position();
        let mut model = PointMass::new();
        model.set_position(start.0, start.1, start.2);

        let params = PurePursuitParameters::new(5.0, 1.0, 0.0);
        let mut controller = PurePursuitController::new(params.clone());
        controller.init(&model, track, params);

        let control = controller.step(&model);
        assert!(control.yaw_rate > 0.0);
    }

    #[test]
    fn test_pure_pursuit_controller_returns_ax() {
        let track = CircleTrack::new(50.0, 10.0, 180);
        let start = track.get_start_position();
        let mut model = PointMass::new();
        model.set_position(start.0, start.1, start.2);

        let params = PurePursuitParameters::new(5.0, 1.0, 1.2);
        let mut controller = PurePursuitController::new(params.clone());
        controller.init(&model, track, params);

        let control = controller.step(&model);
        assert!((control.ax - 1.2).abs() < 1e-9);
    }
}
