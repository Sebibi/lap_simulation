/// Trait for track definitions with boundaries and center line
pub trait Track {
    /// Initialize the track from coordinate lists
    /// 
    /// # Arguments
    /// * `center_line` - List of (x, y) coordinates defining the center line
    /// * `inside_border` - List of (x, y) coordinates defining the inside boundary
    /// * `outside_border` - List of (x, y) coordinates defining the outside boundary
    /// * `get_start_position` - (x, y, yaw) coordinates of the starting position and orientation
    fn init(
        &mut self,
        center_line: Vec<(f64, f64)>,
        inside_border: Vec<(f64, f64)>,
        outside_border: Vec<(f64, f64)>,
        get_start_position: (f64, f64, f64),
    );
    
    /// Check if a given position is within the track boundaries
    /// 
    /// # Arguments
    /// * `x` - x-coordinate to check
    /// * `y` - y-coordinate to check
    /// 
    /// # Returns
    /// `true` if the position is inside the track, `false` otherwise
    fn is_in_track(&self, x: f64, y: f64) -> bool;
    
    /// Get the starting position and orientation on the track
    /// 
    /// # Returns
    /// Tuple of (x, y, yaw) coordinates for the start position and orientation in radians
    fn get_start_position(&self) -> (f64, f64, f64);
    
    /// Get the center line coordinates
    /// 
    /// # Returns
    /// Reference to the list of (x, y) coordinates defining the center line
    fn get_center_line(&self) -> &[(f64, f64)];
    
    /// Get the inside boundary coordinates
    /// 
    /// # Returns
    /// Reference to the list of (x, y) coordinates defining the inside boundary
    fn get_inside_boundary(&self) -> &[(f64, f64)];
    
    /// Get the outside boundary coordinates
    /// 
    /// # Returns
    /// Reference to the list of (x, y) coordinates defining the outside boundary
    fn get_outside_boundary(&self) -> &[(f64, f64)];
    
    /// Get the name of the track for plotting
    /// 
    /// # Returns
    /// String representing the track name
    fn get_track_name(&self) -> &str;
    
    /// Get the plot range for the track
    /// 
    /// # Returns
    /// Tuple of (min_coord, max_coord) for the plot range
    fn get_plot_range(&self) -> (f64, f64);
}
