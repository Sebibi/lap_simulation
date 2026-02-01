use crate::models::base_model::Model;
use crate::models::point_mass::PointMass;
use crate::tracks::base_track::Track;
use crate::tracks::circle::CircleTrack;
use crate::simulation::base_simulation::{Simulation, SimulationResult};

/// Open-loop simulation implementation
pub struct OpenLoopSimulation<M: Model, T: Track> {
    model: Option<M>,
    track: Option<T>,
    dt: f64,
    duration: f64,
}

impl<M: Model, T: Track> OpenLoopSimulation<M, T> {
    /// Create a new open-loop simulation with given parameters
    /// 
    /// # Arguments
    /// * `dt` - Simulation timestep in seconds
    /// * `duration` - Total simulated time in seconds
    pub fn new(dt: f64, duration: f64) -> Self {
        Self {
            model: None,
            track: None,
            dt,
            duration,
        }
    }
    
    /// Get a reference to the model
    pub fn get_model(&self) -> Option<&M> {
        self.model.as_ref()
    }
    
    /// Get a reference to the track
    pub fn get_track(&self) -> Option<&T> {
        self.track.as_ref()
    }
}

impl<M: Model, T: Track> Simulation<M, T> for OpenLoopSimulation<M, T> {
    fn init(&mut self, mut model: M, track: T) {
        // Initialize the model
        model.init();
        
        // Set the model to the track's starting position
        let start_pos = track.get_start_position();
        model.set_position(start_pos.0, start_pos.1, start_pos.2);
        
        self.model = Some(model);
        self.track = Some(track);
    }
    
    fn run(&mut self) -> SimulationResult<M::State>
    where
        M::State: Clone,
    {
        let model = self.model.as_mut().expect("Model not initialized");
        let track = self.track.as_ref().expect("Track not initialized");
        
        let mut result = SimulationResult::new(self.dt);
        
        // Record initial state
        let initial_state = model.get_state().clone();
        let (x, y, _) = model.get_position();
        result.add_snapshot(initial_state, 0.0, track.is_in_track(x, y));
        
        let steps = (self.duration / self.dt).floor() as usize;
        let mut current_time = 0.0;
        
        // Step through the simulation
        for _ in 1..=steps {
            model.step(self.dt);
            current_time += self.dt;
            
            let state = model.get_state().clone();
            let (x, y, _) = model.get_position();
            let in_track = track.is_in_track(x, y);
            
            result.add_snapshot(state, current_time, in_track);
        }
        
        // Handle remaining time
        let remaining = self.duration - current_time;
        if remaining > 0.0 {
            model.step(remaining);
            current_time += remaining;
            
            let state = model.get_state().clone();
            let (x, y, _) = model.get_position();
            let in_track = track.is_in_track(x, y);
            
            result.add_snapshot(state, current_time, in_track);
        }
        
        result
    }
    
    fn reset(&mut self) {
        if let Some(model) = &mut self.model {
            model.reset();
            if let Some(track) = &self.track {
                let start_pos = track.get_start_position();
                model.set_position(start_pos.0, start_pos.1, start_pos.2);
            }
        }
    }
    
    fn clean(&mut self) {
        // No resources to clean in this implementation
        // This would be used if the simulation creates temporary files
    }
}

/// Legacy function for backward compatibility - runs simulation and creates output
/// 
/// This function combines simulation execution with plotting and video creation.
/// 
/// # Arguments
/// - output_dir: directory where SVG frames and the MP4 are written.
/// - dt: simulation timestep in seconds (controls physics integration).
/// - duration: total simulated time in seconds (controls how many frames are saved at 10 FPS).
pub fn open_loop(output_dir: &str, dt: f64, duration: f64) {
    
    
    // Create a circular track with center radius of 50m and 10m track width
    let circle_track = CircleTrack::new(50.0, 10.0, 100);
    println!("Track created: {}\n", circle_track);
    
    // Create a point mass at origin with zero initial velocity
    let mut model = PointMass::new();
    
    // Set constant acceleration and yaw rate inputs
    model.set_controls(2.0, 0.4);
    
    // Create and initialize simulation
    let mut sim = OpenLoopSimulation::new(dt, duration);
    sim.init(model, circle_track);
    
    println!("Simulating point mass motion on circle track:\n");
    println!("Initial state:");
    if let Some(model) = sim.get_model() {
        println!("  {}\n", model);
    }
    
    // Run the simulation
    let result = sim.run();
    
    // Print simulation results
    for (i, snapshot) in result.states.iter().enumerate() {
        println!(
            "Step {}: {} [in_track: {}]",
            i, snapshot.state, snapshot.in_track
        );
    }
    
    println!("\nFinal model state:");
    if let Some(model) = sim.get_model() {
        println!("  {}", model);
    }
    
    // Generate outputs (plotting and video)
    generate_outputs(output_dir, &sim, &result, 10);
}

/// Generate SVG plots and video from simulation results
fn generate_outputs(
    output_dir: &str,
    sim: &OpenLoopSimulation<PointMass, CircleTrack>,
    result: &SimulationResult<crate::models::point_mass::PointMassState>,
    fps: u32,
) {
    use crate::plotting;
    use std::fs;
    
    let model = sim.get_model().expect("Model not initialized");
    let track = sim.get_track().expect("Track not initialized");
    
    let output_dir_ready = fs::create_dir_all(output_dir).is_ok();
    if !output_dir_ready {
        eprintln!("Failed to create output directory");
        return;
    }
    
    let initial_svg = "initial_state.svg";
    let final_svg = "final_state.svg";
    
    // Create a temporary model for plotting at each frame
    let mut temp_model = PointMass::new();
    temp_model.set_size(model.get_size().0, model.get_size().1);
    
    // Plot initial state
    let initial_snapshot = &result.states[0];
    temp_model.set_position(
        initial_snapshot.state.x,
        initial_snapshot.state.y,
        initial_snapshot.state.yaw,
    );
    
    let initial_out = format!("{}/{}", output_dir, initial_svg);
    if let Err(e) = plotting::plot(track, &temp_model, &initial_out) {
        eprintln!("Error plotting: {}", e);
        return;
    }
    
    // Generate frame SVGs at target FPS
    let frame_times = scheduled_frame_times(result.total_time, fps);
    let mut step_svgs: Vec<String> = Vec::new();
    
    for (frame_index, &frame_time) in frame_times.iter().enumerate() {
        // Find the closest state snapshot for this frame time
        let snapshot = find_closest_snapshot(&result.states, frame_time);
        
        // Update temp model with this state
        temp_model.set_position(snapshot.state.x, snapshot.state.y, snapshot.state.yaw);
        
        let step_svg = format!("step_{:03}.svg", frame_index + 1);
        let step_path = format!("{}/{}", output_dir, step_svg);
        step_svgs.push(step_path.clone());
        
        if let Err(e) = plotting::plot(track, &temp_model, &step_path) {
            eprintln!("Error plotting: {}", e);
        }
    }
    
    // Plot final state
    let final_snapshot = result.states.last().unwrap();
    temp_model.set_position(
        final_snapshot.state.x,
        final_snapshot.state.y,
        final_snapshot.state.yaw,
    );
    
    if let Err(e) = plotting::plot(track, &temp_model, &format!("{}/{}", output_dir, final_svg)) {
        eprintln!("Error plotting: {}", e);
        return;
    }
    
    // Create video
    let mut frames: Vec<String> = Vec::with_capacity(step_svgs.len() + 2);
    frames.push(initial_out.clone());
    frames.extend(step_svgs.iter().cloned());
    frames.push(format!("{}/{}", output_dir, final_svg));
    
    let video_path = format!("{}/open_loop.mp4", output_dir);
    if let Err(e) = plotting::create_video_from_svgs(&frames, &video_path, fps) {
        eprintln!("Error creating video: {}", e);
        return;
    }
    
    // Create HTML preview
    if let Err(e) = plotting::write_open_loop_html_preview(
        output_dir,
        "open_loop.mp4",
        Some(initial_svg),
        Some(final_svg),
    ) {
        eprintln!("Error creating HTML preview: {}", e);
    }
    
    // Clean up step SVGs
    for step_svg in &step_svgs {
        if let Err(e) = fs::remove_file(step_svg) {
            eprintln!("Error deleting {}: {}", step_svg, e);
        }
    }
}

fn find_closest_snapshot(
    snapshots: &[crate::simulation::base_simulation::StateSnapshot<crate::models::point_mass::PointMassState>],
    time: f64,
) -> &crate::simulation::base_simulation::StateSnapshot<crate::models::point_mass::PointMassState> {
    snapshots
        .iter()
        .min_by(|a, b| {
            let diff_a = (a.time - time).abs();
            let diff_b = (b.time - time).abs();
            diff_a.partial_cmp(&diff_b).unwrap()
        })
        .unwrap()
}

fn scheduled_frame_times(duration: f64, fps: u32) -> Vec<f64> {
    if duration <= 0.0 || fps == 0 {
        return Vec::new();
    }

    let frame_interval = 1.0f64 / fps as f64;
    let mut times = Vec::new();
    let mut t = frame_interval;
    while t + 1e-9 < duration {
        times.push(t);
        t += frame_interval;
    }

    times
}

#[cfg(test)]
mod tests {
    use super::scheduled_frame_times;

    #[test]
    fn test_scheduled_frame_times_zero_duration() {
        let times = scheduled_frame_times(0.0, 10);
        assert!(times.is_empty());
    }

    #[test]
    fn test_scheduled_frame_times_short_duration() {
        let times = scheduled_frame_times(0.05, 10);
        assert!(times.is_empty());
    }

    #[test]
    fn test_scheduled_frame_times_exact_second() {
        let times = scheduled_frame_times(1.0, 10);
        assert_eq!(times.len(), 9);
        assert!((times[0] - 0.1).abs() < 1e-9);
        assert!((times[times.len() - 1] - 0.9).abs() < 1e-9);
    }

    #[test]
    fn test_scheduled_frame_times_after_second() {
        let times = scheduled_frame_times(1.01, 10);
        assert_eq!(times.len(), 10);
        assert!((times[9] - 1.0).abs() < 1e-9);
    }
}
