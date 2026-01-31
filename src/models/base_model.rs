/// Trait for simulation models with standard lifecycle methods
pub trait Model {
    /// State type returned by get_state
    type State;
    
    /// Initialize the model with default or provided parameters
    fn init(&mut self);
    
    /// Perform one simulation step with the given time delta
    fn step(&mut self, dt: f64);
    
    /// Reset the model to its initial state
    fn reset(&mut self);
    
    /// Set the position and yaw of the model
    fn set_position(&mut self, x: f64, y: f64, yaw: f64);
    
    /// Get the size of the model
    /// 
    /// # Returns
    /// Tuple of (length, width) in meters of the model (for plotting purposes for example)
    fn get_size(&self) -> (f64, f64);
    
    /// Get the current position and yaw angle of the model
    /// 
    /// # Returns
    /// Tuple of (x, y, yaw) where x and y are coordinates in meters and yaw is in radians
    fn get_position(&self) -> (f64, f64, f64);
    
    /// Get the current state of the model
    fn get_state(&self) -> &Self::State;
}