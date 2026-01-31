use lap_simulation::tracks::base_track::Track;
use lap_simulation::tracks::circle::CircleTrack;
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
    assert!((point_90.0).abs() < 0.1);
    assert!((point_90.1 - 50.0).abs() < 0.1);
}
