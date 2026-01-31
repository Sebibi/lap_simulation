use lap_simulation::tracks::base_track::Track;
use lap_simulation::tracks::square::SquareTrack;

#[test]
fn test_square_track_creation() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    
    // 4 sides * 25 points per side = 100 total points
    assert_eq!(track.get_center_line().len(), 100);
    assert_eq!(track.get_inside_boundary().len(), 100);
    assert_eq!(track.get_outside_boundary().len(), 100);
}

#[test]
fn test_square_track_get_start_position() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    let start = track.get_start_position();
    let yaw = track.get_center_line_yaw()[0];
    
    // Start position should be at first center line point with matching yaw
    assert!((start.0 - 50.0).abs() < 1e-10);
    assert!((start.1 - (-50.0)).abs() < 1e-10);
    assert!((start.2 - yaw).abs() < 1e-10);
}

#[test]
fn test_square_track_center_line_first_point() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    let center_line = track.get_center_line();
    
    // First point should be at (50, -50) - middle of right side, bottom
    let first_point = center_line[0];
    assert!((first_point.0 - 50.0).abs() < 1e-10);
    assert!((first_point.1 - (-50.0)).abs() < 1e-10);
}

#[test]
fn test_square_track_center_line_yaw() {
    let points_per_side = 25;
    let track = SquareTrack::new(100.0, 10.0, points_per_side);
    let center_line = track.get_center_line();
    let yaw = track.get_center_line_yaw();

    assert_eq!(yaw.len(), center_line.len());

    // Right side (moving up)
    assert!((yaw[0] - std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    // Top side (moving left)
    assert!((yaw[points_per_side] - std::f64::consts::PI).abs() < 1e-10);
    // Left side (moving down)
    assert!((yaw[2 * points_per_side] + std::f64::consts::FRAC_PI_2).abs() < 1e-10);
    // Bottom side (moving right)
    assert!(yaw[3 * points_per_side].abs() < 1e-10);
}

#[test]
fn test_square_track_is_in_track_on_center_line() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    
    // Point on center line at (50, 0) should be in track
    assert!(track.is_in_track(50.0, 0.0));
}

#[test]
fn test_square_track_is_in_track_inside_boundary() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    
    // Point inside (closer to center) should be in track
    // half_inside = (100 - 10) / 2 = 45
    // This point is at 46m distance but inside the boundary tolerance
    assert!(track.is_in_track(46.0, 0.0));
}

#[test]
fn test_square_track_is_in_track_outside_boundary() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    
    // Point outside (farther from center) should be in track
    // half_outside = (100 + 10) / 2 = 55
    assert!(track.is_in_track(54.0, 0.0));
}

#[test]
fn test_square_track_is_not_in_track_too_inner() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    
    // Point at center (0, 0) should NOT be in track (inside inner boundary)
    assert!(!track.is_in_track(0.0, 0.0));
}

#[test]
fn test_square_track_is_not_in_track_too_outer() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    
    // Point far outside should NOT be in track
    // half_outside = 55
    assert!(!track.is_in_track(56.0, 0.0));
}

#[test]
fn test_square_track_corners() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    let center_line = track.get_center_line();
    
    // Should have 100 points (25 per side * 4)
    // Points per side = 25
    // Side 0 (right): points 0-24, y goes from -50 to 50
    // Side 1 (top): points 25-49, x goes from 50 to -50
    // Side 2 (left): points 50-74, y goes from 50 to -50
    // Side 3 (bottom): points 75-99, x goes from -50 to 50
    
    // Last point of right side (i=24, t=24/25=0.96) is at y = -50 + 0.96*100 = 46
    let last_right = center_line[24];
    assert!((last_right.0 - 50.0).abs() < 1e-10);
    assert!((last_right.1 - 46.0).abs() < 1.0);
}

#[test]
fn test_square_track_symmetry() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    let center_line = track.get_center_line();
    
    // Check x-coordinates on right and left sides
    // Right side (first point): should have positive x
    assert!(center_line[0].0 > 0.0);
    
    // Left side (50th point area): should have negative x approximately
    assert!(center_line[50].0 < 0.0);
}

#[test]
fn test_square_track_with_different_sizes() {
    let track1 = SquareTrack::new(80.0, 8.0, 20);
    let track2 = SquareTrack::new(200.0, 20.0, 50);
    
    let start1 = track1.get_start_position();
    let start2 = track2.get_start_position();
    let yaw1 = track1.get_center_line_yaw()[0];
    let yaw2 = track2.get_center_line_yaw()[0];
    
    // start should match first center line point and yaw
    assert!((start1.0 - 40.0).abs() < 1e-10);
    assert!((start1.1 - (-40.0)).abs() < 1e-10);
    assert!((start2.0 - 100.0).abs() < 1e-10);
    assert!((start2.1 - (-100.0)).abs() < 1e-10);
    assert!((start1.2 - yaw1).abs() < 1e-10);
    assert!((start2.2 - yaw2).abs() < 1e-10);
}

#[test]
fn test_square_track_narrow_track() {
    let track = SquareTrack::new(100.0, 2.0, 25);
    
    // With narrow track, boundaries should be closer
    let inside_boundary = track.get_inside_boundary();
    let outside_boundary = track.get_outside_boundary();
    
    assert_eq!(inside_boundary.len(), 100);
    assert_eq!(outside_boundary.len(), 100);
    
    // Inside and outside points should be very close
    let inside_first = inside_boundary[0];
    let outside_first = outside_boundary[0];
    
    let inside_dist = inside_first.0.abs() + inside_first.1.abs();
    let outside_dist = outside_first.0.abs() + outside_first.1.abs();
    
    // Distance difference should be small due to narrow track
    assert!((outside_dist - inside_dist - 2.0).abs() < 1.0);
}

#[test]
fn test_square_track_point_in_center() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    
    // Point in the hollow center should NOT be in track
    assert!(!track.is_in_track(5.0, 5.0));
}

#[test]
fn test_square_track_point_on_edge() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    
    // Point very close to edge
    // half_inside = 45, half_outside = 55
    assert!(track.is_in_track(45.1, 0.0));
}
