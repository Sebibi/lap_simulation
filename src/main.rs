use lap_simulation::models::base_model::Model;
use lap_simulation::models::point_mass::PointMass;
use lap_simulation::tracks::base_track::Track;
use lap_simulation::tracks::circle::CircleTrack;
use lap_simulation::plotting;

fn main() {
    // Create a circular track with center radius of 50m and 10m track width
    let circle_track = CircleTrack::new(50.0, 10.0, 100);
    println!("Track created: {}\n", circle_track);
    
    // Plot the track using the plotting module
    if let Err(e) = plotting::track::plot_track(&circle_track, "circle_track.svg") {
        eprintln!("Error plotting circle track: {}", e);
    }
    
    // Create a point mass at origin with zero initial velocity
    let mut model = PointMass::new();
    
    // Initialize the model
    model.init();
    
    // Set the model to the track's starting position
    let start_pos = circle_track.start_position();
    model.set_position(start_pos.0, start_pos.1, 0.0);
    println!("Model starting position: ({:.3}, {:.3})\n", start_pos.0, start_pos.1);
    
    // Set constant acceleration inputs (e.g., 2 m/s^2 in x, 1 m/s^2 in y)
    model.set_controls(2.0, 1.0);
    
    let dt = 0.1; // Time step in seconds
    
    println!("Simulating point mass motion on circle track:\n");
    println!("Initial state:");
    println!("  {}\n", model);
    
    // Step 100 times and print state after each step
    for i in 1..=30 {
        model.step(dt);
        let state = model.get_state();
        let in_track = circle_track.is_in_track(state.x, state.y);
        println!("Step {}: {} [in_track: {}]", i, state, in_track);
    }
    
    println!("\nFinal model state:");
    println!("  {}", model);
    
    // Plot the model at its final position using the plotting module
    if let Err(e) = plotting::model::plot_model(&model, "model_position.svg") {
        eprintln!("Error plotting model: {}", e);
    } else {
        println!("\nModel position plotted to model_position.svg");
    }
}

