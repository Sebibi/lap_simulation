use crate::models::base_model::Model;
use crate::models::point_mass::{PointMass, PointMassState};
use crate::plotting;
use crate::tracks::base_track::Track;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct OpenLoopArtifacts {
    pub initial_svg: PathBuf,
    pub final_svg: PathBuf,
    pub video_path: PathBuf,
    pub html_path: PathBuf,
}

pub fn render_open_loop_outputs<P: AsRef<Path>>(
    output_dir: P,
    track: &dyn Track,
    states: &[PointMassState],
    model_size: (f64, f64),
    dt: f64,
    duration: f64,
    fps: u32,
) -> Result<OpenLoopArtifacts, Box<dyn Error>> {
    if states.is_empty() {
        return Err("no states to render".into());
    }
    if fps == 0 {
        return Err("fps must be greater than zero".into());
    }

    let output_dir = output_dir.as_ref();
    fs::create_dir_all(output_dir)?;

    let initial_svg = output_dir.join("initial_state.svg");
    let final_svg = output_dir.join("final_state.svg");
    let video_path = output_dir.join("open_loop.mp4");

    let mut model = PointMass::new();
    model.set_size(model_size.0, model_size.1);

    let initial_state = &states[0];
    model.set_position(initial_state.x, initial_state.y, initial_state.yaw);
    plotting::plot(track, &model, path_as_str(&initial_svg)?)?;

    let frame_times = scheduled_frame_times(duration, fps);
    let state_times = build_state_times(states.len(), dt, duration);
    let mut step_svgs: Vec<PathBuf> = Vec::new();
    let mut frame_index = 1usize;
    let mut next_frame_index = 0usize;

    for (state, time) in states.iter().zip(state_times.iter()) {
        while next_frame_index < frame_times.len()
            && *time + 1e-9 >= frame_times[next_frame_index]
        {
            let step_svg = output_dir.join(format!("step_{:03}.svg", frame_index));
            model.set_position(state.x, state.y, state.yaw);
            plotting::plot(track, &model, path_as_str(&step_svg)?)?;
            step_svgs.push(step_svg);
            frame_index += 1;
            next_frame_index += 1;
        }
    }

    let final_state = states
        .last()
        .expect("states should not be empty when rendering output");
    model.set_position(final_state.x, final_state.y, final_state.yaw);
    plotting::plot(track, &model, path_as_str(&final_svg)?)?;

    let mut frames: Vec<PathBuf> = Vec::with_capacity(step_svgs.len() + 2);
    frames.push(initial_svg.clone());
    frames.extend(step_svgs.iter().cloned());
    frames.push(final_svg.clone());

    plotting::create_video_from_svgs(&frames, &video_path, fps)?;

    let html_path = plotting::write_open_loop_html_preview(
        output_dir,
        "open_loop.mp4",
        Some("initial_state.svg"),
        Some("final_state.svg"),
    )?;

    for step_svg in &step_svgs {
        fs::remove_file(step_svg)?;
    }

    Ok(OpenLoopArtifacts {
        initial_svg,
        final_svg,
        video_path,
        html_path,
    })
}

fn build_state_times(states_len: usize, dt: f64, duration: f64) -> Vec<f64> {
    if states_len == 0 {
        return Vec::new();
    }
    if states_len == 1 {
        return vec![0.0];
    }

    let steps = states_len - 1;
    if dt > 0.0 && duration > 0.0 {
        let expected_steps = (duration / dt).floor() as usize;
        let remainder = duration - (expected_steps as f64) * dt;
        let expected_len = if remainder > 0.0 {
            expected_steps + 2
        } else {
            expected_steps + 1
        };

        if states_len == expected_len {
            let mut times = Vec::with_capacity(states_len);
            let mut current_time = 0.0f64;
            times.push(current_time);

            for step_index in 0..steps {
                let step_dt = if remainder > 0.0 && step_index == steps - 1 {
                    remainder
                } else {
                    dt
                };
                current_time += step_dt;
                times.push(current_time);
            }

            return times;
        }
    }

    let total_duration = if duration > 0.0 {
        duration
    } else if dt > 0.0 {
        dt * steps as f64
    } else {
        0.0
    };
    let step_dt = total_duration / steps as f64;

    (0..states_len)
        .map(|idx| idx as f64 * step_dt)
        .collect()
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

fn path_as_str(path: &Path) -> Result<&str, std::io::Error> {
    path.to_str().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "path contains non-UTF-8 characters",
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{render_open_loop_outputs, scheduled_frame_times};
    use crate::tracks::circle::CircleTrack;
    use crate::models::point_mass::PointMassState;

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

    #[test]
    fn test_render_open_loop_outputs_rejects_empty_states() {
        let track = CircleTrack::new(50.0, 10.0, 100);
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");

        let err = render_open_loop_outputs(
            temp_dir.path(),
            &track,
            &[],
            (4.5, 2.0),
            0.1,
            1.0,
            10,
        )
        .expect_err("expected error for empty states");
        assert!(err.to_string().contains("no states"));
    }

    #[test]
    fn test_render_open_loop_outputs_rejects_zero_fps() {
        let track = CircleTrack::new(50.0, 10.0, 100);
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let states = vec![PointMassState {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
            yaw: 0.0,
        }];

        let err = render_open_loop_outputs(
            temp_dir.path(),
            &track,
            &states,
            (4.5, 2.0),
            0.1,
            1.0,
            0,
        )
        .expect_err("expected error for zero fps");
        assert!(err.to_string().contains("fps"));
    }
}
