use lap_simulation::tracks::circle::CircleTrack;
use lap_simulation::tracks::square::SquareTrack;
use lap_simulation::plotting;
use std::fs;

#[test]
fn test_circle_track_plot() {
    let track = CircleTrack::new(50.0, 10.0, 100);
    let filename = "test_circle_track.svg";
    
    let result = plotting::track::plot_track(&track, filename);
    assert!(result.is_ok());
    
    // Verify file was created
    assert!(fs::metadata(filename).is_ok());
    
    // Clean up
    let _ = fs::remove_file(filename);
}

#[test]
fn test_square_track_plot() {
    let track = SquareTrack::new(100.0, 10.0, 25);
    let filename = "test_square_track.svg";
    
    let result = plotting::track::plot_track(&track, filename);
    assert!(result.is_ok());
    
    // Verify file was created
    assert!(fs::metadata(filename).is_ok());
    
    // Clean up
    let _ = fs::remove_file(filename);
}
