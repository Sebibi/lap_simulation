use crate::controllers::base_controller::{
    ControlInput, Controller, find_nearest_point_index, normalize_angle,
};
use crate::models::base_model::Model;
use crate::models::point_mass::PointMass;
use crate::tracks::base_track::Track;
use std::fmt;

#[derive(Debug, Clone)]
pub struct StanleyParameters {
    pub gain: f64,
    pub softening: f64,
    pub target_speed: f64,
    pub ax: f64,
}

impl StanleyParameters {
    pub fn new(gain: f64, softening: f64, target_speed: f64, ax: f64) -> Self {
        Self {
            gain,
            softening,
            target_speed,
            ax,
        }
    }
}

impl Default for StanleyParameters {
    fn default() -> Self {
        Self {
            gain: 1.0,
            softening: 1.0,
            target_speed: 5.0,
            ax: 0.0,
        }
    }
}

impl fmt::Display for StanleyParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "StanleyParameters {{ gain: {:.3}, softening: {:.3}, target_speed: {:.3} m/s, ax: {:.3} m/s² }}",
            self.gain, self.softening, self.target_speed, self.ax
        )
    }
}

pub struct StanleyController<T: Track> {
    track: Option<T>,
    parameters: StanleyParameters,
    initial_parameters: StanleyParameters,
}

impl<T: Track> StanleyController<T> {
    pub fn new(parameters: StanleyParameters) -> Self {
        Self {
            track: None,
            parameters: parameters.clone(),
            initial_parameters: parameters,
        }
    }

    pub fn track(&self) -> Option<&T> {
        self.track.as_ref()
    }

    pub fn parameters(&self) -> &StanleyParameters {
        &self.parameters
    }
}

impl<T: Track> Default for StanleyController<T> {
    fn default() -> Self {
        Self::new(StanleyParameters::default())
    }
}

impl<T: Track> Controller for StanleyController<T> {
    type Model = PointMass;
    type Track = T;
    type Parameters = StanleyParameters;

    fn init(&mut self, _model: &Self::Model, track: Self::Track, parameters: Self::Parameters) {
        self.track = Some(track);
        self.parameters = parameters.clone();
        self.initial_parameters = parameters;
    }

    fn step(&self, model: &Self::Model) -> ControlInput {
        let track = match self.track.as_ref() {
            Some(track) => track,
            None => return ControlInput::new(0.0, 0.0),
        };

        let center_line = track.get_center_line();
        let center_line_yaw = track.get_center_line_yaw();
        let (x, y, yaw) = model.get_position();

        let nearest_index = match find_nearest_point_index(center_line, x, y) {
            Some(index) => index,
            None => return ControlInput::new(0.0, 0.0),
        };

        let (cx, cy) = center_line[nearest_index];
        let path_yaw = center_line_yaw.get(nearest_index).copied().unwrap_or(0.0);

        let heading_error = normalize_angle(path_yaw - yaw);
        let dx = x - cx;
        let dy = y - cy;
        let cross_track_error = dx * (-path_yaw.sin()) + dy * path_yaw.cos();
        let denom = self.parameters.softening + self.parameters.target_speed.abs();
        let correction = (self.parameters.gain * cross_track_error).atan2(denom);
        let yaw_rate = heading_error + correction;

        ControlInput::new(self.parameters.ax, yaw_rate)
    }

    fn reset(&mut self) {
        self.track = None;
        self.parameters = self.initial_parameters.clone();
    }
}

impl<T: Track> fmt::Display for StanleyController<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.track.as_ref() {
            Some(track) => write!(
                f,
                "StanleyController {{ track: {}, parameters: {} }}",
                track.get_track_name(),
                self.parameters
            ),
            None => write!(
                f,
                "StanleyController {{ track: uninitialized, parameters: {} }}",
                self.parameters
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{StanleyController, StanleyParameters};
    use crate::controllers::base_controller::Controller;
    use crate::models::base_model::Model;
    use crate::models::point_mass::PointMass;
    use crate::tracks::base_track::Track;
    use crate::tracks::circle::CircleTrack;

    #[test]
    fn test_stanley_controller_on_center_line() {
        let track = CircleTrack::new(50.0, 10.0, 180);
        let start = track.get_start_position();
        let mut model = PointMass::new();
        model.set_position(start.0, start.1, start.2);

        let params = StanleyParameters::new(1.0, 1.0, 5.0, 0.0);
        let mut controller = StanleyController::new(params.clone());
        controller.init(&model, track, params);

        let control = controller.step(&model);
        assert!(control.yaw_rate.abs() < 1e-6);
    }

    #[test]
    fn test_stanley_controller_returns_ax() {
        let track = CircleTrack::new(50.0, 10.0, 180);
        let start = track.get_start_position();
        let mut model = PointMass::new();
        model.set_position(start.0, start.1, start.2);

        let params = StanleyParameters::new(2.0, 1.0, 5.0, 1.5);
        let mut controller = StanleyController::new(params.clone());
        controller.init(&model, track, params);

        let control = controller.step(&model);
        assert!((control.ax - 1.5).abs() < 1e-9);
    }
}
