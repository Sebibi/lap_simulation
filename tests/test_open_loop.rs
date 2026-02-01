use lap_simulation::simulation::open_loop::open_loop;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_open_loop_simulation_outputs_svgs() {
    let ffmpeg_ok = Command::new("ffmpeg")
        .arg("-version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    assert!(ffmpeg_ok, "ffmpeg is required but was not found on PATH");

    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();

    let output_dir: PathBuf = ["tests", "results", "open_loop", &format!("{}", unique)]
        .iter()
        .collect();

    let output_dir_str = output_dir
        .to_str()
        .expect("failed to build output dir path");

    // Run the simulation and ensure it produces the expected SVG files.
    open_loop(output_dir_str);

    let initial_svg = output_dir.join("initial_state.svg");
    let final_svg = output_dir.join("final_state.svg");
    let video_mp4 = output_dir.join("open_loop.mp4");

    assert!(initial_svg.exists(), "missing initial_state.svg in output dir");
    assert!(final_svg.exists(), "missing final_state.svg in output dir");
    assert!(video_mp4.exists(), "missing open_loop.mp4 in output dir");

    // Cleanup (best-effort)
    let _ = fs::remove_dir_all(&output_dir);
}
