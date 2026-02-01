# Lap Simulation

A Rust-based lap simulation framework for modeling vehicle dynamics on various track geometries.

## Coordinate Convention

### World Frame
- **X-axis**: Positive direction points to the right (east)
- **Y-axis**: Positive direction points up (north)
- **Origin**: (0, 0) is at the bottom-left corner of the coordinate system

### Yaw Angle (Orientation)
- **yaw = 0**: Vehicle points in the positive X-axis direction (right/east)
- **yaw = π/2**: Vehicle points in the positive Y-axis direction (up/north)
- **yaw = π**: Vehicle points in the negative X-axis direction (left/west)
- **yaw = 3π/2 or -π/2**: Vehicle points in the negative Y-axis direction (down/south)
- **Positive rotation**: Counter-clockwise (right-hand rule about Z-axis)

### Body Frame
- **Body X-axis**: Points forward along the vehicle's longitudinal axis
- **Body Y-axis**: Points to the left side of the vehicle
- When yaw = 0, the body frame aligns with the world frame

### Velocity and Acceleration
- **Body frame velocities** (`vx`, `vy`): Defined relative to the vehicle's orientation
  - `vx`: Velocity along the vehicle's forward direction (body x-axis)
  - `vy`: Velocity perpendicular to the vehicle (body y-axis, positive = left)
- **Body frame accelerations** (`ax`, `ay`): Control inputs in body frame coordinates
- **World frame velocities**: Computed by rotating body frame velocities by the yaw angle

### Rotation Transform
Body frame to world frame:
```
x_world = x_body * cos(yaw) - y_body * sin(yaw)
y_world = x_body * sin(yaw) + y_body * cos(yaw)
```

## Project Structure

```
src/
├── models/           # Vehicle dynamics models
│   ├── base_model.rs # Model trait definition
│   └── point_mass.rs # Point mass implementation
├── tracks/           # Track definitions
│   ├── base_track.rs # Track trait definition
│   ├── circle.rs     # Circular track
│   └── square.rs     # Square track
└── plotting/         # Visualization module
    ├── track.rs      # Track plotting functions
    ├── model.rs      # Model plotting functions
    └── create.rs     # Combined plotting
```

## Usage

Run the simulation:
```bash
cargo run
```

Run tests:
```bash
cargo test
```

Note: `ffmpeg` must be installed and available on PATH to generate videos
and to run the `open_loop` test.
