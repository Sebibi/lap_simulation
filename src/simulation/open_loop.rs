use crate::models::base_model::Model;
use crate::models::point_mass::PointMass;
use crate::tracks::base_track::Track;
use crate::tracks::circle::CircleTrack;
use crate::plotting;
use std::fs;

pub fn open_loop(output_dir: &str) {
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
    model.set_controls(2.0, 0.0);
    
    let dt = 0.1; // Time step in seconds
    
    println!("Simulating point mass motion on circle track:\n");
    println!("Initial state:");
    println!("  {}\n", model);

    let initial_svg = "initial_state.svg";
    let final_svg = "final_state.svg";

    // Plot initial track and model
    if let Err(e) = plotting::plot(&circle_track, &model, initial_svg) {
        eprintln!("Error plotting: {}", e);
    }
    let output_dir_ready = fs::create_dir_all(output_dir).is_ok();
    if output_dir_ready {
        if let Err(e) = plotting::plot(&circle_track, &model, &format!("{}/{}", output_dir, initial_svg)) {
            eprintln!("Error plotting: {}", e);
        }
    }
    
    // Step 100 times and print state after each step
    for i in 1..=30 {
        model.step(dt);
        let state = model.get_state();
        let in_track = circle_track.is_in_track(state.x, state.y);
        println!("Step {}: {} [in_track: {}]", i, state, in_track);

        if output_dir_ready {
            let step_svg = format!("step_{:03}.svg", i);
            if let Err(e) = plotting::plot(&circle_track, &model, &format!("{}/{}", output_dir, step_svg)) {
                eprintln!("Error plotting: {}", e);
            }
        }
    }
    
    println!("\nFinal model state:");
    println!("  {}", model);
    
    // Plot the track and model together in a single plot
    if let Err(e) = plotting::plot(&circle_track, &model, final_svg) {
        eprintln!("Error plotting: {}", e);
    }
    if output_dir_ready {
        if let Err(e) = plotting::plot(&circle_track, &model, &format!("{}/{}", output_dir, final_svg)) {
            eprintln!("Error plotting: {}", e);
        }
    }
}

