use super::base_track::Track;
use std::fmt;

/// Square track defined by height and track width
pub struct SquareTrack {
    center_line: Vec<(f64, f64)>,
    inside_border: Vec<(f64, f64)>,
    outside_border: Vec<(f64, f64)>,
    start_pos: (f64, f64),
    height: f64,
    track_width: f64,
}

impl SquareTrack {
    /// Create a new square track
    /// 
    /// # Arguments
    /// * `height` - Height (and width) of the square center line
    /// * `track_width` - Width of the track (distance from inside to outside boundary)
    /// * `points_per_side` - Number of points to generate per side (default: 25)
    pub fn new(height: f64, track_width: f64, points_per_side: usize) -> Self {
        let mut track = Self {
            center_line: Vec::new(),
            inside_border: Vec::new(),
            outside_border: Vec::new(),
            start_pos: (height / 2.0, 0.0),
            height,
            track_width,
        };
        
        // Generate the square paths
        track.generate_squares(points_per_side);
        track
    }
    
    fn generate_squares(&mut self, points_per_side: usize) {
        let half_center = self.height / 2.0;
        let half_inside = half_center - self.track_width / 2.0;
        let half_outside = half_center + self.track_width / 2.0;
        
        self.center_line.clear();
        self.inside_border.clear();
        self.outside_border.clear();
        
        // Generate points for each of the 4 sides
        // Right side (moving up)
        for i in 0..points_per_side {
            let t = i as f64 / points_per_side as f64;
            let y = -half_center + t * self.height;
            
            self.center_line.push((half_center, y));
            self.inside_border.push((half_inside, y));
            self.outside_border.push((half_outside, y));
        }
        
        // Top side (moving left)
        for i in 0..points_per_side {
            let t = i as f64 / points_per_side as f64;
            let x = half_center - t * self.height;
            
            self.center_line.push((x, half_center));
            self.inside_border.push((x, half_inside));
            self.outside_border.push((x, half_outside));
        }
        
        // Left side (moving down)
        for i in 0..points_per_side {
            let t = i as f64 / points_per_side as f64;
            let y = half_center - t * self.height;
            
            self.center_line.push((-half_center, y));
            self.inside_border.push((-half_inside, y));
            self.outside_border.push((-half_outside, y));
        }
        
        // Bottom side (moving right)
        for i in 0..points_per_side {
            let t = i as f64 / points_per_side as f64;
            let x = -half_center + t * self.height;
            
            self.center_line.push((x, -half_center));
            self.inside_border.push((x, -half_inside));
            self.outside_border.push((x, -half_outside));
        }
        
        // Set start position at the middle of the right side
        self.start_pos = (half_center, 0.0);
    }
}

impl Track for SquareTrack {
    fn init(
        &mut self,
        center_line: Vec<(f64, f64)>,
        inside_border: Vec<(f64, f64)>,
        outside_border: Vec<(f64, f64)>,
        start_position: (f64, f64),
    ) {
        self.center_line = center_line;
        self.inside_border = inside_border;
        self.outside_border = outside_border;
        self.start_pos = start_position;
    }
    
    fn is_in_track(&self, x: f64, y: f64) -> bool {
        let half_inside = (self.height - self.track_width) / 2.0;
        let half_outside = (self.height + self.track_width) / 2.0;
        
        // Check if point is within the outer square
        let in_outer = x.abs() <= half_outside && y.abs() <= half_outside;
        
        // Check if point is outside the inner square
        let out_inner = x.abs() >= half_inside || y.abs() >= half_inside;
        
        in_outer && out_inner
    }
    
    fn start_position(&self) -> (f64, f64) {
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
        "Square Track"
    }
    
    fn get_plot_range(&self) -> (f64, f64) {
        let margin = self.track_width;
        let max_coord = self.height / 2.0 + self.track_width / 2.0 + margin;
        let min_coord = -(self.height / 2.0 + self.track_width / 2.0 + margin);
        (min_coord, max_coord)
    }
}

impl fmt::Display for SquareTrack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SquareTrack {{ height: {:.3} m, track_width: {:.3} m, num_points: {} }}",
            self.height,
            self.track_width,
            self.center_line.len()
        )
    }
}
