use lap_simulation::models::base_model::Model;
use lap_simulation::models::point_mass::PointMass;
use lap_simulation::tracks::base_track::Track;
use lap_simulation::tracks::circle::CircleTrack;
use lap_simulation::plotting;

fn main() {
    let output_dir = std::env::var("OUTPUT_DIR").unwrap_or_else(|_| "results/images".to_string());
    if let Err(e) = std::fs::create_dir_all(&output_dir) {
        eprintln!("Error creating output directory {}: {}", output_dir, e);
    }

    // Create a circular track with center radius of 50m and 10m track width
    let circle_track = CircleTrack::new(50.0, 10.0, 100);
    println!("Track created: {}\n", circle_track);
    
    // Create a point mass at origin with zero initial velocity
    let mut model = PointMass::new();
    
    // Initialize the model
    model.init();
    
    // Set the model to the track's starting position
    let start_pos = circle_track.get_start_position();
    model.set_position(start_pos.0, start_pos.1, start_pos.2);
    println!("Model starting position: ({:.3}, {:.3}, {:.3})\n", start_pos.0, start_pos.1, start_pos.2);
    
    // Set constant acceleration inputs (e.g., 2 m/s^2 in x, 0 m/s^2 in y)
    model.set_controls(2.0, 1.0);
    
    let dt = 0.1; // Time step in seconds
    
    println!("Simulating point mass motion on circle track:\n");
    println!("Initial state:");
    println!("  {}\n", model);

    // Plot initial track and model
    let initial_path = format!("{}/initial_state.svg", output_dir);
    if let Err(e) = plotting::plot(&circle_track, &model, &initial_path) {
        eprintln!("Error plotting: {}", e);
    }
    
    // Step 50 times and print state after each step
    for i in 1..=50 {
        model.step(dt);
        let state = model.get_state();
        let in_track = circle_track.is_in_track(state.x, state.y);
        println!("Step {}: {} [in_track: {}]", i, state, in_track);

        if i % 5 == 0 {
            let filename = format!("{}/state_step_{:02}.svg", output_dir, i);
            if let Err(e) = plotting::plot(&circle_track, &model, &filename) {
                eprintln!("Error plotting: {}", e);
            }
        }
    }
    
    println!("\nFinal model state:");
    println!("  {}", model);
    
    // Plot the track and model together in a single plot
    let final_path = format!("{}/final_state.svg", output_dir);
    if let Err(e) = plotting::plot(&circle_track, &model, &final_path) {
        eprintln!("Error plotting: {}", e);
    }
}
