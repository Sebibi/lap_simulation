use super::base_track::{compute_center_line_yaw, Track};
use std::f64::consts::PI;
use std::fmt;

/// Circular track defined by center line radius and track width
pub struct CircleTrack {
    center_line: Vec<(f64, f64)>,
    center_line_yaw: Vec<f64>,
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
            center_line_yaw: Vec::new(),
            inside_border: Vec::new(),
            outside_border: Vec::new(),
            start_pos: (center_radius, 0.0, PI / 2.0),
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
        self.center_line_yaw.clear();
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

        self.center_line_yaw = compute_center_line_yaw(&self.center_line);
        if let (Some(&(x, y)), Some(&yaw)) = (self.center_line.first(), self.center_line_yaw.first()) {
            self.start_pos = (x, y, yaw);
        }
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
        self.center_line_yaw = compute_center_line_yaw(&self.center_line);
        self.inside_border = inside_border;
        self.outside_border = outside_border;
        self.start_pos = get_start_position;
        if let (Some(&(x, y)), Some(&yaw)) = (self.center_line.first(), self.center_line_yaw.first()) {
            self.start_pos = (x, y, yaw);
        }
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

    fn get_center_line_yaw(&self) -> &[f64] {
        &self.center_line_yaw
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

#[cfg(test)]
mod tests {
    use super::CircleTrack;
    use crate::tracks::base_track::Track;
    use std::f64::consts::PI;

    #[test]
    fn test_circle_track_creation() {
        let track = CircleTrack::new(50.0, 10.0, 100);

        assert_eq!(track.get_center_line().len(), 100);
        assert_eq!(track.get_inside_boundary().len(), 100);
        assert_eq!(track.get_outside_boundary().len(), 100);
    }

    #[test]
    fn test_circle_track_get_start_position() {
        let track = CircleTrack::new(50.0, 10.0, 100);
        let start = track.get_start_position();
        let yaw = track.get_center_line_yaw()[0];

        assert!((start.0 - 50.0).abs() < 1e-10);
        assert!((start.1 - 0.0).abs() < 1e-10);
        assert!((start.2 - yaw).abs() < 1e-10);
    }

    #[test]
    fn test_circle_track_center_line_first_point() {
        let track = CircleTrack::new(50.0, 10.0, 100);
        let center_line = track.get_center_line();

        let first_point = center_line[0];
        assert!((first_point.0 - 50.0).abs() < 1e-10);
        assert!((first_point.1 - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_circle_track_center_line_yaw() {
        let track = CircleTrack::new(50.0, 10.0, 360);
        let center_line = track.get_center_line();
        let yaw = track.get_center_line_yaw();

        assert_eq!(yaw.len(), center_line.len());

        // At angle 0, tangent should point upward (~pi/2)
        assert!((yaw[0] - PI / 2.0).abs() < 0.1);
    }

    #[test]
    fn test_circle_track_is_in_track_on_center_line() {
        let track = CircleTrack::new(50.0, 10.0, 100);

        // Point on center line at 50m radius should be in track
        assert!(track.is_in_track(50.0, 0.0));
    }

    #[test]
    fn test_circle_track_is_in_track_inside_boundary() {
        let track = CircleTrack::new(50.0, 10.0, 100);

        // Point inside boundary at 45m radius should be in track
        assert!(track.is_in_track(45.0, 0.0));
    }

    #[test]
    fn test_circle_track_is_in_track_outside_boundary() {
        let track = CircleTrack::new(50.0, 10.0, 100);

        // Point outside boundary at 55m radius should be in track
        assert!(track.is_in_track(55.0, 0.0));
    }

    #[test]
    fn test_circle_track_is_not_in_track_too_inner() {
        let track = CircleTrack::new(50.0, 10.0, 100);

        // Point at 40m radius (inside the inner boundary) should NOT be in track
        assert!(!track.is_in_track(40.0, 0.0));
    }

    #[test]
    fn test_circle_track_is_not_in_track_too_outer() {
        let track = CircleTrack::new(50.0, 10.0, 100);

        // Point at 60m radius (outside the outer boundary) should NOT be in track
        assert!(!track.is_in_track(60.0, 0.0));
    }

    #[test]
    fn test_circle_track_boundaries_radii() {
        let center_radius = 50.0;
        let track_width = 10.0;
        let track = CircleTrack::new(center_radius, track_width, 100);

        let inside_boundary = track.get_inside_boundary();
        let outside_boundary = track.get_outside_boundary();

        // Check first point of inside boundary (angle 0)
        let inside_first = inside_boundary[0];
        let inside_radius = (inside_first.0.powi(2) + inside_first.1.powi(2)).sqrt();
        assert!((inside_radius - (center_radius - track_width / 2.0)).abs() < 1e-10);

        // Check first point of outside boundary (angle 0)
        let outside_first = outside_boundary[0];
        let outside_radius = (outside_first.0.powi(2) + outside_first.1.powi(2)).sqrt();
        assert!((outside_radius - (center_radius + track_width / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_circle_track_circular_geometry() {
        let track = CircleTrack::new(50.0, 10.0, 8);
        let center_line = track.get_center_line();

        // Each point on center line should be at 50m radius
        for point in center_line {
            let radius = (point.0.powi(2) + point.1.powi(2)).sqrt();
            assert!((radius - 50.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_circle_track_with_different_sizes() {
        let track1 = CircleTrack::new(30.0, 5.0, 50);
        let track2 = CircleTrack::new(100.0, 20.0, 200);

        let start1 = track1.get_start_position();
        let start2 = track2.get_start_position();
        let yaw1 = track1.get_center_line_yaw()[0];
        let yaw2 = track2.get_center_line_yaw()[0];

        assert!((start1.0 - 30.0).abs() < 1e-10);
        assert!((start2.0 - 100.0).abs() < 1e-10);
        assert!((start1.2 - yaw1).abs() < 1e-10);
        assert!((start2.2 - yaw2).abs() < 1e-10);
    }

    #[test]
    fn test_circle_track_point_at_angle() {
        let track = CircleTrack::new(50.0, 10.0, 360);
        let center_line = track.get_center_line();

        // At 360 points, each point is 1 degree apart
        // Point at 90 degrees should be approximately at (0, 50)
        let point_90 = center_line[90];
        assert!(point_90.0.abs() < 0.1);
        assert!((point_90.1 - 50.0).abs() < 0.1);
    }
}
