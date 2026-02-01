use crate::models::base_model::Model;
use crate::models::point_mass::PointMass;
use crate::tracks::base_track::Track;
use crate::tracks::circle::CircleTrack;
use crate::plotting;
use std::fs;

/// Run the open-loop simulation.
///
/// Arguments:
/// - output_dir: directory where SVG frames and the MP4 are written.
/// - dt: simulation timestep in seconds (controls physics integration).
/// - duration: total simulated time in seconds (controls how many frames are saved at 10 FPS).
pub fn open_loop(output_dir: &str, dt: f64, duration: f64) {
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
    
    let fps = 10u32;
    let frame_interval = 1.0f64 / fps as f64;
    
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
    
    let mut current_time = 0.0f64;
    let mut next_frame_time = frame_interval;
    let mut frame_index = 1usize;
    let steps = (duration / dt).floor() as usize;

    // Step through the simulation and capture frames at the target fps.
    for i in 1..=steps {
        model.step(dt);
        current_time += dt;
        let state = model.get_state();
        let in_track = circle_track.is_in_track(state.x, state.y);
        println!("Step {}: {} [in_track: {}]", i, state, in_track);

        if output_dir_ready {
            while current_time + 1e-9 >= next_frame_time && next_frame_time + 1e-9 < duration {
                let step_svg = format!("step_{:03}.svg", frame_index);
                let step_path = format!("{}/{}", output_dir, step_svg);
                step_svgs.push(step_path.clone());
                if let Err(e) = plotting::plot(&circle_track, &model, &step_path) {
                    eprintln!("Error plotting: {}", e);
                }
                frame_index += 1;
                next_frame_time += frame_interval;
            }
        }
    }

    let remaining = duration - current_time;
    if remaining > 0.0 {
        model.step(remaining);
        current_time += remaining;
        let state = model.get_state();
        let in_track = circle_track.is_in_track(state.x, state.y);
        println!("Step {}: {} [in_track: {}]", steps + 1, state, in_track);

        if output_dir_ready {
            while current_time + 1e-9 >= next_frame_time && next_frame_time + 1e-9 < duration {
                let step_svg = format!("step_{:03}.svg", frame_index);
                let step_path = format!("{}/{}", output_dir, step_svg);
                step_svgs.push(step_path.clone());
                if let Err(e) = plotting::plot(&circle_track, &model, &step_path) {
                    eprintln!("Error plotting: {}", e);
                }
                frame_index += 1;
                next_frame_time += frame_interval;
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
