use lap_simulation::models::base_model::Model;
use lap_simulation::models::point_mass::PointMass;
use lap_simulation::plotting::render_open_loop_outputs;
use lap_simulation::simulation::base_simulation::Simulation;
use lap_simulation::simulation::open_loop::OpenLoopSimulation;
use lap_simulation::tracks::circle::CircleTrack;
use std::process::Command;

#[test]
fn test_open_loop_simulation_outputs_svgs_and_video() {
    let ffmpeg_ok = Command::new("ffmpeg")
        .arg("-version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    assert!(ffmpeg_ok, "ffmpeg is required but was not found on PATH");

    let output_dir = crate::common::temp_output_dir("open_loop");
    let output_path = output_dir.path().join("results");

    let track = CircleTrack::new(50.0, 10.0, 100);
    let model = PointMass::new();
    let mut simulation = OpenLoopSimulation::new();
    simulation.init(track, model);

    let dt = 0.1;
    let duration = 3.0;
    let fps = 10;
    let states = simulation.run(dt, duration);

    let track = simulation.track().expect("track missing after run");
    let model = simulation.model().expect("model missing after run");
    render_open_loop_outputs(
        &output_path,
        track,
        &states,
        model.get_size(),
        dt,
        duration,
        fps,
    )
    .expect("failed to render open-loop outputs");

    let initial_svg = output_path.join("initial_state.svg");
    let final_svg = output_path.join("final_state.svg");
    let video_mp4 = output_path.join("open_loop.mp4");

    assert!(initial_svg.exists(), "missing initial_state.svg in output dir");
    assert!(final_svg.exists(), "missing final_state.svg in output dir");
    assert!(video_mp4.exists(), "missing open_loop.mp4 in output dir");
}
