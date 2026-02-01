use lap_simulation::simulation::open_loop::open_loop;
use std::fs;
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
    let output_path_str = output_path
        .to_str()
        .expect("failed to build output dir path");

    // Run the simulation and ensure it produces the expected SVG files.
    open_loop(output_path_str, 0.1, 3.0);

    let initial_svg = output_path.join("initial_state.svg");
    let final_svg = output_path.join("final_state.svg");
    let video_mp4 = output_path.join("open_loop.mp4");
    let preview_html = output_path.join("open_loop_preview.html");

    assert!(initial_svg.exists(), "missing initial_state.svg in output dir");
    assert!(final_svg.exists(), "missing final_state.svg in output dir");
    assert!(video_mp4.exists(), "missing open_loop.mp4 in output dir");
    assert!(preview_html.exists(), "missing open_loop_preview.html in output dir");

    let preview_contents = fs::read_to_string(&preview_html)
        .expect("failed to read open_loop_preview.html");
    assert!(
        preview_contents.contains("open_loop.mp4"),
        "preview html should reference open_loop.mp4"
    );
    assert!(
        preview_contents.contains("initial_state.svg"),
        "preview html should reference initial_state.svg"
    );
    assert!(
        preview_contents.contains("final_state.svg"),
        "preview html should reference final_state.svg"
    );

    let leftover_step_svgs: Vec<_> = fs::read_dir(&output_path)
        .expect("failed to read output dir")
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.starts_with("step_") && name.ends_with(".svg"))
        .collect();
    assert!(
        leftover_step_svgs.is_empty(),
        "expected step SVGs to be cleaned up, found: {:?}",
        leftover_step_svgs
    );
}
