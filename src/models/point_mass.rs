use super::base_model::Model;
use std::fmt;

/// State of a 2D point mass
#[derive(Debug, Clone)]
pub struct PointMassState {
    pub x: f64,    // World frame x position
    pub y: f64,    // World frame y position
    pub vx: f64,   // Body frame x velocity
    pub vy: f64,   // Body frame y velocity
    pub yaw: f64,  // Orientation angle (radians)
}

impl fmt::Display for PointMassState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pos: ({:.2}, {:.2}), Vel: ({:.2}, {:.2}), Yaw: {:.2}",
            self.x, self.y, self.vx, self.vy, self.yaw
        )
    }
}

/// Point mass model with 2D dynamics
pub struct PointMass {
    state: PointMassState,
    initial_state: PointMassState,
    ax: f64,     // Body frame x-axis acceleration input
    ay: f64,     // Body frame y-axis acceleration input
    length: f64, // Vehicle length in meters
    width: f64,  // Vehicle width in meters
}

impl PointMass {
    /// Create a new point mass at the origin with zero velocity
    pub fn new() -> Self {
        let initial_state = PointMassState {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            yaw: 0.0,
        };
        
        Self {
            state: initial_state.clone(),
            initial_state,
            ax: 0.0,
            ay: 0.0,
            length: 4.5,  // Default car length
            width: 2.0,   // Default car width
        }
    }
    
    /// Create a new point mass with initial position and velocity
    pub fn with_initial_state(x: f64, y: f64, vx: f64, vy: f64, yaw: f64) -> Self {
        let initial_state = PointMassState { x, y, vx, vy, yaw };
        
        Self {
            state: initial_state.clone(),
            initial_state,
            ax: 0.0,
            ay: 0.0,
            length: 4.5,  // Default car length
            width: 2.0,   // Default car width
        }
    }
    
    /// Set acceleration inputs
    pub fn set_controls(&mut self, ax: f64, ay: f64) {
        self.ax = ax;
        self.ay = ay;
    }
    
    /// Set the position
    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.state.x = x;
        self.state.y = y;
    }
    
    /// Set the size of the vehicle
    pub fn set_size(&mut self, length: f64, width: f64) {
        self.length = length;
        self.width = width;
    }
}

impl Model for PointMass {
    type State = PointMassState;
    
    fn init(&mut self) {
        self.state = self.initial_state.clone();
        self.ax = 0.0;
        self.ay = 0.0;
    }
    
    fn step(&mut self, dt: f64) {
        // Update velocities in body frame using acceleration inputs
        self.state.vx += self.ax * dt;
        self.state.vy += self.ay * dt;
        
        // Transform body frame velocities to world frame
        let cos_yaw = self.state.yaw.cos();
        let sin_yaw = self.state.yaw.sin();
        
        let vx_world = self.state.vx * cos_yaw - self.state.vy * sin_yaw;
        let vy_world = self.state.vx * sin_yaw + self.state.vy * cos_yaw;
        
        // Update yaw to point in the direction of motion (world frame)
        // Only update if there is significant velocity to avoid numerical issues
        if vx_world.abs() > 1e-10 || vy_world.abs() > 1e-10 {
            self.state.yaw = vy_world.atan2(vx_world);
        }
        
        // Update positions in world frame
        self.state.x += vx_world * dt;
        self.state.y += vy_world * dt;
    }
    
    fn reset(&mut self) {
        self.state = self.initial_state.clone();
        self.ax = 0.0;
        self.ay = 0.0;
    }
    
    fn set_position(&mut self, x: f64, y: f64, yaw: f64) {
        self.state.x = x;
        self.state.y = y;
        self.state.yaw = yaw;
    }
    
    fn get_size(&self) -> (f64, f64) {
        (self.length, self.width)
    }
    
    fn get_position(&self) -> (f64, f64, f64) {
        (self.state.x, self.state.y, self.state.yaw)
    }
    
    fn get_state(&self) -> &Self::State {
        &self.state
    }
}

impl fmt::Display for PointMass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PointMass {{ {}, Acceleration: ({:.3}, {:.3}) m/s² }}",
            self.state, self.ax, self.ay
        )
    }
}

#[cfg(test)]
mod tests {
    use super::PointMass;
    use crate::models::base_model::Model;

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
    fn test_point_mass_yaw_update() {
        let mut model = PointMass::new();
        model.set_controls(2.0, 0.0);

        // After one step, vx=0.2, vy=0.0 (body frame)
        // With initial yaw=0: vx_world=0.2, vy_world=0.0
        // yaw should be atan2(0.0, 0.2) = 0.0 (pointing in +x direction)
        model.step(0.1);
        let state = model.get_state();
        assert!((state.yaw - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_mass_yaw_update_with_lateral_velocity() {
        use std::f64::consts::PI;

        let mut model = PointMass::new();
        model.set_controls(0.0, 2.0); // Lateral acceleration only

        // After one step, vx=0.0, vy=0.2 (body frame)
        // With initial yaw=0: vx_world=0.0, vy_world=0.2
        // yaw should be atan2(0.2, 0.0) = PI/2 (pointing in +y direction)
        model.step(0.1);
        let state = model.get_state();
        assert!((state.yaw - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_mass_yaw_update_diagonal() {
        use std::f64::consts::PI;

        let mut model = PointMass::new();
        model.set_controls(1.0, 1.0); // Equal acceleration in both directions

        // After one step, vx=0.1, vy=0.1 (body frame)
        // With initial yaw=0: vx_world=0.1, vy_world=0.1
        // yaw should be atan2(0.1, 0.1) = PI/4 (45 degrees)
        model.step(0.1);
        let state = model.get_state();
        assert!((state.yaw - PI / 4.0).abs() < 1e-10);
    }
}
