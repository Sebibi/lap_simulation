use lap_simulation::models::base_model::Model;
use lap_simulation::models::point_mass::PointMass;
use lap_simulation::simulation::base_simulation::{Simulation, SimulationResult};
use lap_simulation::simulation::open_loop::OpenLoopSimulation;
use lap_simulation::tracks::base_track::Track;
use lap_simulation::tracks::circle::CircleTrack;

#[test]
fn test_open_loop_simulation_init() {
    let mut sim = OpenLoopSimulation::new(0.1, 1.0);
    let model = PointMass::new();
    let track = CircleTrack::new(50.0, 10.0, 100);
    
    sim.init(model, track);
    
    assert!(sim.get_model().is_some());
    assert!(sim.get_track().is_some());
}

#[test]
fn test_open_loop_simulation_run_returns_states() {
    let mut sim = OpenLoopSimulation::new(0.1, 1.0);
    let model = PointMass::new();
    let track = CircleTrack::new(50.0, 10.0, 100);
    
    sim.init(model, track);
    
    // Set controls to test actual movement
    if let Some(model) = sim.get_model_mut() {
        model.set_controls(2.0, 0.4);
    }
    
    let result = sim.run();
    
    // Should have initial state + 10 steps + 1 final step = 12 total
    assert_eq!(result.states.len(), 12);
    assert_eq!(result.dt, 0.1);
    assert!((result.total_time - 1.0).abs() < 1e-9);
    
    // Check that states are different (model is moving)
    let first_state = &result.states[0].state;
    let last_state = &result.states[result.states.len() - 1].state;
    assert_ne!(first_state.x, last_state.x);
    assert_ne!(first_state.y, last_state.y);
}

#[test]
fn test_open_loop_simulation_reset() {
    let mut sim = OpenLoopSimulation::new(0.1, 1.0);
    let model = PointMass::new();
    let track = CircleTrack::new(50.0, 10.0, 100);
    
    sim.init(model, track);
    
    // Set controls and run
    if let Some(model) = sim.get_model_mut() {
        model.set_controls(2.0, 0.4);
    }
    
    let result1 = sim.run();
    let final_pos1 = (
        result1.states.last().unwrap().state.x,
        result1.states.last().unwrap().state.y,
    );
    
    // Reset and run again
    sim.reset();
    if let Some(model) = sim.get_model_mut() {
        model.set_controls(2.0, 0.4);
    }
    
    let result2 = sim.run();
    let final_pos2 = (
        result2.states.last().unwrap().state.x,
        result2.states.last().unwrap().state.y,
    );
    
    // Final positions should be the same after reset
    assert!((final_pos1.0 - final_pos2.0).abs() < 1e-9);
    assert!((final_pos1.1 - final_pos2.1).abs() < 1e-9);
}

#[test]
fn test_simulation_result_snapshots() {
    let mut result = SimulationResult::new(0.1);
    
    assert_eq!(result.states.len(), 0);
    assert_eq!(result.total_time, 0.0);
    assert_eq!(result.dt, 0.1);
    
    result.add_snapshot(
        lap_simulation::models::point_mass::PointMassState {
            x: 1.0,
            y: 2.0,
            vx: 0.5,
            vy: 0.0,
            yaw: 0.0,
        },
        0.1,
        true,
    );
    
    assert_eq!(result.states.len(), 1);
    assert_eq!(result.total_time, 0.1);
    assert_eq!(result.states[0].time, 0.1);
    assert_eq!(result.states[0].in_track, true);
}

#[test]
fn test_open_loop_simulation_tracks_in_track_status() {
    let mut sim = OpenLoopSimulation::new(0.1, 3.0);  // Longer duration
    let model = PointMass::new();
    let track = CircleTrack::new(50.0, 10.0, 100);
    
    sim.init(model, track);
    
    // Set high controls to quickly go out of track
    if let Some(model) = sim.get_model_mut() {
        model.set_controls(10.0, 1.0);
    }
    
    let result = sim.run();
    
    // Initially should be in track
    assert!(result.states[0].in_track);
    
    // Eventually should go out of track with high acceleration
    let has_out_of_track = result.states.iter().any(|s| !s.in_track);
    assert!(has_out_of_track, "Model should eventually leave track with high acceleration");
}
