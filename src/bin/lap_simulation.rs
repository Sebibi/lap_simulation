use lap_simulation::models::base_model::Model;
use lap_simulation::models::point_mass::PointMass;
use lap_simulation::plotting::render_open_loop_outputs;
use lap_simulation::simulation::base_simulation::Simulation;
use lap_simulation::simulation::open_loop::OpenLoopSimulation;
use lap_simulation::tracks::circle::CircleTrack;

fn main() {
    let track = CircleTrack::new(50.0, 10.0, 100);
    let model = PointMass::new();
    let mut simulation = OpenLoopSimulation::new();
    simulation.init(track, model);

    let dt = 0.1;
    let duration = 10.0;
    let fps = 10;
    let states = simulation.run(dt, duration);

    let Some(track) = simulation.track() else {
        eprintln!("Simulation track missing after run");
        return;
    };
    let Some(model) = simulation.model() else {
        eprintln!("Simulation model missing after run");
        return;
    };

    if let Err(err) = render_open_loop_outputs(
        "results/images",
        track,
        &states,
        model.get_size(),
        dt,
        duration,
        fps,
    ) {
        eprintln!("Failed to render open-loop outputs: {err}");
    }
}
