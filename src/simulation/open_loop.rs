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
    
    let dt: f64 = 0.1; // Time step in seconds
    let fps = (1.0f64 / dt).round().max(1.0) as u32;
    
    println!("Simulating point mass motion on circle track:\n");
    println!("Initial state:");
    println!("  {}\n", model);

    let initial_svg = "initial_state.svg";
    let final_svg = "final_state.svg";

    let output_dir_ready = fs::create_dir_all(output_dir).is_ok();
    let mut step_svgs: Vec<String> = Vec::new();
    let mut initial_path: Option<String> = None;
    if output_dir_ready {
        let initial_out = format!("{}/{}", output_dir, initial_svg);
        if let Err(e) = plotting::plot(&circle_track, &model, &initial_out) {
            eprintln!("Error plotting: {}", e);
        } else {
            initial_path = Some(initial_out);
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
            let step_path = format!("{}/{}", output_dir, step_svg);
            step_svgs.push(step_path.clone());
            if let Err(e) = plotting::plot(&circle_track, &model, &step_path) {
                eprintln!("Error plotting: {}", e);
            }
        }
    }
    
    println!("\nFinal model state:");
    println!("  {}", model);
    
    if output_dir_ready {
        if let Err(e) = plotting::plot(&circle_track, &model, &format!("{}/{}", output_dir, final_svg)) {
            eprintln!("Error plotting: {}", e);
        }

        if let Some(initial_svg) = initial_path {
            let mut frames: Vec<String> = Vec::with_capacity(step_svgs.len() + 2);
            frames.push(initial_svg);
            frames.extend(step_svgs.iter().cloned());
            frames.push(format!("{}/{}", output_dir, final_svg));
            let video_path = format!("{}/open_loop.mp4", output_dir);
            if let Err(e) = plotting::create_video_from_svgs(&frames, &video_path, fps) {
                eprintln!("Error creating video: {}", e);
            } else {
                for step_svg in &step_svgs {
                    if let Err(e) = fs::remove_file(step_svg) {
                        eprintln!("Error deleting {}: {}", step_svg, e);
                    }
                }
            }
        }
    }
}
