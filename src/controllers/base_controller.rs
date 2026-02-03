use crate::models::base_model::Model;
use crate::tracks::base_track::Track;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlInput {
    pub ax: f64,
    pub yaw_rate: f64,
}

impl ControlInput {
    pub fn new(ax: f64, yaw_rate: f64) -> Self {
        Self { ax, yaw_rate }
    }
}

impl fmt::Display for ControlInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ControlInput {{ ax: {:.3} m/s², yaw_rate: {:.3} rad/s }}",
            self.ax, self.yaw_rate
        )
    }
}

/// Trait for controller implementations with standard lifecycle methods.
pub trait Controller {
    type Model: Model;
    type Track: Track;
    type Parameters: fmt::Display + Clone;

    /// Initialize the controller with the current model, track, and parameters.
    fn init(&mut self, model: &Self::Model, track: Self::Track, parameters: Self::Parameters);

    /// Compute control inputs for the provided model state.
    fn step(&self, model: &Self::Model) -> ControlInput;

    /// Reset the controller to its initial state.
    fn reset(&mut self);
}

pub(crate) fn normalize_angle(angle: f64) -> f64 {
    let mut wrapped = angle;
    while wrapped > std::f64::consts::PI {
        wrapped -= 2.0 * std::f64::consts::PI;
    }
    while wrapped < -std::f64::consts::PI {
        wrapped += 2.0 * std::f64::consts::PI;
    }
    wrapped
}

pub(crate) fn find_nearest_point_index(
    center_line: &[(f64, f64)],
    x: f64,
    y: f64,
) -> Option<usize> {
    if center_line.is_empty() {
        return None;
    }

    let mut nearest_index = 0usize;
    let mut nearest_dist = f64::INFINITY;

    for (index, &(cx, cy)) in center_line.iter().enumerate() {
        let dx = cx - x;
        let dy = cy - y;
        let dist = dx * dx + dy * dy;
        if dist < nearest_dist {
            nearest_dist = dist;
            nearest_index = index;
        }
    }

    Some(nearest_index)
}
