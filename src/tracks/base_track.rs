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

    /// Get the yaw orientation along the center line
    ///
    /// # Returns
    /// Reference to the list of yaw angles (radians) corresponding to each center line point
    fn get_center_line_yaw(&self) -> &[f64];
    
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

/// Compute yaw angles for a closed center line using forward differences.
pub fn compute_center_line_yaw(center_line: &[(f64, f64)]) -> Vec<f64> {
    let n = center_line.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![0.0];
    }

    let mut yaw = Vec::with_capacity(n);
    for i in 0..n {
        let (x0, y0) = center_line[i];
        let (x1, y1) = center_line[(i + 1) % n];
        let dx = x1 - x0;
        let dy = y1 - y0;
        yaw.push(dy.atan2(dx));
    }
    yaw
}
