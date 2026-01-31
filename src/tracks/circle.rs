use super::base_track::Track;
use std::f64::consts::PI;
use std::fmt;

/// Circular track defined by center line radius and track width
pub struct CircleTrack {
    center_line: Vec<(f64, f64)>,
    inside_border: Vec<(f64, f64)>,
    outside_border: Vec<(f64, f64)>,
    start_pos: (f64, f64, f64),
    center_radius: f64,
    track_width: f64,
}

impl CircleTrack {
    /// Create a new circular track
    /// 
    /// # Arguments
    /// * `center_radius` - Radius of the center line circle
    /// * `track_width` - Width of the track (distance from inside to outside boundary)
    /// * `num_points` - Number of points to generate for each boundary (default: 100)
    pub fn new(center_radius: f64, track_width: f64, num_points: usize) -> Self {
        let mut track = Self {
            center_line: Vec::new(),
            inside_border: Vec::new(),
            outside_border: Vec::new(),
            start_pos: (center_radius, 0.0, 0.0),
            center_radius,
            track_width,
        };
        
        // Generate the circles
        track.generate_circles(num_points);
        track
    }
    
    fn generate_circles(&mut self, num_points: usize) {
        let inside_radius = self.center_radius - self.track_width / 2.0;
        let outside_radius = self.center_radius + self.track_width / 2.0;
        
        self.center_line.clear();
        self.inside_border.clear();
        self.outside_border.clear();
        
        for i in 0..num_points {
            let angle = 2.0 * PI * (i as f64) / (num_points as f64);
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            
            // Center line
            self.center_line.push((
                self.center_radius * cos_a,
                self.center_radius * sin_a,
            ));
            
            // Inside boundary
            self.inside_border.push((
                inside_radius * cos_a,
                inside_radius * sin_a,
            ));
            
            // Outside boundary
            self.outside_border.push((
                outside_radius * cos_a,
                outside_radius * sin_a,
            ));
        }
        
        // Set start position at angle 0 (pointing in positive X direction)
        self.start_pos = (self.center_radius, 0.0, 0.0);
    }
}

impl Track for CircleTrack {
    fn init(
        &mut self,
        center_line: Vec<(f64, f64)>,
        inside_border: Vec<(f64, f64)>,
        outside_border: Vec<(f64, f64)>,
        get_start_position: (f64, f64, f64),
    ) {
        self.center_line = center_line;
        self.inside_border = inside_border;
        self.outside_border = outside_border;
        self.start_pos = get_start_position;
    }
    
    fn is_in_track(&self, x: f64, y: f64) -> bool {
        let distance_from_center = (x * x + y * y).sqrt();
        let inside_radius = self.center_radius - self.track_width / 2.0;
        let outside_radius = self.center_radius + self.track_width / 2.0;
        
        distance_from_center >= inside_radius && distance_from_center <= outside_radius
    }
    
    fn get_start_position(&self) -> (f64, f64, f64) {
        self.start_pos
    }
    
    fn get_center_line(&self) -> &[(f64, f64)] {
        &self.center_line
    }
    
    fn get_inside_boundary(&self) -> &[(f64, f64)] {
        &self.inside_border
    }
    
    fn get_outside_boundary(&self) -> &[(f64, f64)] {
        &self.outside_border
    }
    
    fn get_track_name(&self) -> &str {
        "Circle Track"
    }
    
    fn get_plot_range(&self) -> (f64, f64) {
        let margin = self.track_width;
        let max_coord = self.center_radius + self.track_width / 2.0 + margin;
        let min_coord = -(self.center_radius + self.track_width / 2.0 + margin);
        (min_coord, max_coord)
    }
}

impl fmt::Display for CircleTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CircleTrack {{ radius: {:.3} m, track_width: {:.3} m, num_points: {} }}",
            self.center_radius,
            self.track_width,
            self.center_line.len()
        )
    }
}
