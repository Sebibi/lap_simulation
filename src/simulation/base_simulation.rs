use crate::models::base_model::Model;
use crate::tracks::base_track::Track;

/// Trait for simulations with standard lifecycle methods
pub trait Simulation<M: Model, T: Track> {
    /// Initialize the simulation with a model and track
    /// 
    /// # Arguments
    /// * `model` - The model to simulate
    /// * `track` - The track on which the simulation runs
    fn init(&mut self, model: M, track: T);
    
    /// Run the simulation and return all states
    /// 
    /// # Returns
    /// Vector of all model states during the trajectory, along with metadata like time and track status
    fn run(&mut self) -> SimulationResult<M::State>
    where
        M::State: Clone;
    
    /// Reset the simulation to its initial state
    fn reset(&mut self);
    
    /// Clean up resources (e.g., temporary files, outputs)
    fn clean(&mut self);
}

/// Result of a simulation run containing all states and metadata
#[derive(Debug, Clone)]
pub struct SimulationResult<S> {
    /// All states recorded during the simulation
    pub states: Vec<StateSnapshot<S>>,
    /// Total simulated time
    pub total_time: f64,
    /// Simulation timestep used
    pub dt: f64,
}

/// A snapshot of the model state at a specific time
#[derive(Debug, Clone)]
pub struct StateSnapshot<S> {
    /// The state at this timestep
    pub state: S,
    /// Simulation time when this state was recorded
    pub time: f64,
    /// Whether the model was inside the track at this time
    pub in_track: bool,
}

impl<S> SimulationResult<S> {
    /// Create a new empty simulation result
    pub fn new(dt: f64) -> Self {
        Self {
            states: Vec::new(),
            total_time: 0.0,
            dt,
        }
    }
    
    /// Add a state snapshot to the result
    pub fn add_snapshot(&mut self, state: S, time: f64, in_track: bool) {
        self.states.push(StateSnapshot {
            state,
            time,
            in_track,
        });
        self.total_time = time;
    }
}
