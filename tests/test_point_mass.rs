use lap_simulation::models::base_model::Model;
use lap_simulation::models::point_mass::PointMass;

#[test]
fn test_point_mass_creation() {
    let model = PointMass::new();
    let state = model.get_state();
    
    assert_eq!(state.x, 0.0);
    assert_eq!(state.y, 0.0);
    assert_eq!(state.vx, 0.0);
    assert_eq!(state.vy, 0.0);
    assert_eq!(state.yaw, 0.0);
}

#[test]
fn test_point_mass_with_initial_state() {
    let model = PointMass::with_initial_state(10.0, 20.0, 5.0, 3.0, 0.5);
    let state = model.get_state();
    
    assert_eq!(state.x, 10.0);
    assert_eq!(state.y, 20.0);
    assert_eq!(state.vx, 5.0);
    assert_eq!(state.vy, 3.0);
    assert_eq!(state.yaw, 0.5);
}

#[test]
fn test_point_mass_init() {
    let mut model = PointMass::with_initial_state(10.0, 20.0, 5.0, 3.0, 0.5);
    model.init();
    
    let state = model.get_state();
    assert_eq!(state.x, 10.0);
    assert_eq!(state.y, 20.0);
    assert_eq!(state.vx, 5.0);
    assert_eq!(state.vy, 3.0);
    assert_eq!(state.yaw, 0.5);
}

#[test]
fn test_point_mass_set_controls() {
    let mut model = PointMass::new();
    model.set_controls(2.0, 3.0);
    
    // Acceleration should be stored internally
    model.step(1.0);
    let state = model.get_state();
    
    assert_eq!(state.vx, 2.0);
    assert_eq!(state.vy, 3.0);
}

#[test]
fn test_point_mass_step_velocity() {
    let mut model = PointMass::new();
    model.set_controls(2.0, 1.0);
    
    // After one step with dt=0.1
    // vx (body frame) should be 2.0 * 0.1 = 0.2
    // vy (body frame) should be 1.0 * 0.1 = 0.1
    model.step(0.1);
    let state = model.get_state();
    
    assert!((state.vx - 0.2).abs() < 1e-10);
    assert!((state.vy - 0.1).abs() < 1e-10);
}

#[test]
fn test_point_mass_step_position() {
    let mut model = PointMass::new();
    model.set_controls(2.0, 1.0);
    
    let dt = 0.1;
    model.step(dt);
    
    // After first step: vx=0.2, vy=0.1, yaw=0
    // With yaw=0: vx_world = 0.2, vy_world = 0.1
    // x = 0 + 0.2 * 0.1 = 0.02
    // y = 0 + 0.1 * 0.1 = 0.01
    let state = model.get_state();
    assert!((state.x - 0.02).abs() < 1e-10);
    assert!((state.y - 0.01).abs() < 1e-10);
}

#[test]
fn test_point_mass_multiple_steps() {
    let mut model = PointMass::new();
    model.set_controls(2.0, 1.0);
    
    let dt = 0.1;
    for _ in 0..10 {
        model.step(dt);
    }
    
    let state = model.get_state();
    
    // After 10 steps:
    // vx should be 2.0 * 0.1 * 10 = 2.0
    // vy should be 1.0 * 0.1 * 10 = 1.0
    assert!((state.vx - 2.0).abs() < 1e-9);
    assert!((state.vy - 1.0).abs() < 1e-9);
}

#[test]
fn test_point_mass_reset() {
    let mut model = PointMass::with_initial_state(5.0, 10.0, 2.0, 3.0, 0.5);
    model.set_controls(1.0, 2.0);
    
    model.step(0.1);
    model.reset();
    
    let state = model.get_state();
    assert_eq!(state.x, 5.0);
    assert_eq!(state.y, 10.0);
    assert_eq!(state.vx, 2.0);
    assert_eq!(state.vy, 3.0);
    assert_eq!(state.yaw, 0.5);
}

#[test]
fn test_point_mass_set_position() {
    let mut model = PointMass::new();
    model.set_position(15.0, 25.0, 0.5);
    
    let state = model.get_state();
    assert_eq!(state.x, 15.0);
    assert_eq!(state.y, 25.0);
    assert_eq!(state.yaw, 0.5);
}

#[test]
fn test_point_mass_set_pos() {
    let mut model = PointMass::new();
    model.set_pos(7.5, 12.5);
    
    let state = model.get_state();
    assert_eq!(state.x, 7.5);
    assert_eq!(state.y, 12.5);
}

#[test]
fn test_point_mass_kinematics() {
    let mut model = PointMass::new();
    model.set_controls(4.0, 2.0);
    
    let dt = 0.5;
    model.step(dt);
    
    // vx (body) = 4.0 * 0.5 = 2.0
    // vy (body) = 2.0 * 0.5 = 1.0
    // With yaw=0: vx_world = 2.0, vy_world = 1.0
    // x = 0 + 2.0 * 0.5 = 1.0
    // y = 0 + 1.0 * 0.5 = 0.5
    let state = model.get_state();
    
    assert!((state.vx - 2.0).abs() < 1e-10);
    assert!((state.x - 1.0).abs() < 1e-10);
    assert!((state.vy - 1.0).abs() < 1e-10);
    assert!((state.y - 0.5).abs() < 1e-10);
}

#[test]
fn test_point_mass_with_yaw() {
    use std::f64::consts::PI;
    
    // Test with yaw = PI/2 (90 degrees, pointing in +y direction)
    let mut model = PointMass::with_initial_state(0.0, 0.0, 0.0, 0.0, PI / 2.0);
    model.set_controls(10.0, 0.0); // Accelerate forward in body frame
    
    let dt = 0.1;
    model.step(dt);
    
    // vx (body) = 10.0 * 0.1 = 1.0
    // vy (body) = 0.0
    // yaw = PI/2: cos(yaw) = 0, sin(yaw) = 1
    // vx_world = 1.0 * 0 - 0.0 * 1 = 0.0
    // vy_world = 1.0 * 1 + 0.0 * 0 = 1.0
    // x = 0 + 0.0 * 0.1 = 0.0
    // y = 0 + 1.0 * 0.1 = 0.1
    let state = model.get_state();
    
    assert!((state.vx - 1.0).abs() < 1e-10);
    assert!(state.x.abs() < 1e-10);
    assert!((state.y - 0.1).abs() < 1e-10);
}

#[test]
fn test_point_mass_get_size() {
    let model = PointMass::new();
    let (length, width) = model.get_size();
    
    // Check default dimensions
    assert_eq!(length, 4.5);
    assert_eq!(width, 2.0);
}

#[test]
fn test_point_mass_set_size() {
    let mut model = PointMass::new();
    model.set_size(5.0, 2.5);
    
    let (length, width) = model.get_size();
    assert_eq!(length, 5.0);
    assert_eq!(width, 2.5);
}

#[test]
fn test_point_mass_plot_model() {
    use std::f64::consts::PI;
    use lap_simulation::plotting;
    
    // Create a model at position (10, 20) with yaw = PI/4 (45 degrees)
    let mut model = PointMass::with_initial_state(10.0, 20.0, 0.0, 0.0, PI / 4.0);
    model.set_size(5.0, 2.0);
    
    // Plot the model
    let result = plotting::model::plot_model(&model, "test_model_plot.svg");
    assert!(result.is_ok(), "Failed to plot model: {:?}", result.err());
    
    // Verify file was created
    let path = std::path::Path::new("test_model_plot.svg");
    assert!(path.exists(), "Plot file was not created");
    
    // Clean up
    std::fs::remove_file("test_model_plot.svg").ok();
}
