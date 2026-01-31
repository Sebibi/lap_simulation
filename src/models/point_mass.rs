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
