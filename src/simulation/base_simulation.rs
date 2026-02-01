use crate::models::base_model::Model;
use crate::tracks::base_track::Track;

/// Trait for simulations with a standard lifecycle.
pub trait Simulation {
    type Track: Track;
    type Model: Model;

    /// Initialize the simulation with a track and a model.
    fn init(&mut self, track: Self::Track, model: Self::Model);

    /// Run the simulation and return the model states over the trajectory.
    fn run(&mut self, dt: f64, duration: f64) -> Vec<<Self::Model as Model>::State>;

    /// Reset the simulation to its initial state.
    fn reset(&mut self);

    /// Clean up resources owned by the simulation.
    fn clean(&mut self);
}
