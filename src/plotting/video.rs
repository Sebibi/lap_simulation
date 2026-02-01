use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Create a video from a list of SVG frames using ffmpeg.
///
/// Requires `ffmpeg` to be available on PATH with SVG decoding support.
pub fn create_video_from_svgs<P: AsRef<Path>, Q: AsRef<Path>>(
    svgs: &[P],
    output_path: Q,
    fps: u32,
) -> Result<(), Box<dyn Error>> {
    if svgs.is_empty() {
        return Err("no SVG frames provided".into());
    }
    if fps == 0 {
        return Err("fps must be greater than zero".into());
    }

    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let concat_path = concat_list_path(output_path);
    write_concat_list(svgs, &concat_path, fps)?;

    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(&concat_path)
        .arg("-vsync")
        .arg("vfr")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg(output_path)
        .status()?;

    let _ = fs::remove_file(&concat_path);

    if !status.success() {
        return Err(format!(
            "ffmpeg failed with status {} (output: {})",
            status,
            output_path.display()
        )
        .into());
    }

    println!("Video saved to {}", output_path.display());
    Ok(())
}

fn concat_list_path(output_path: &Path) -> PathBuf {
    let mut path = output_path.to_path_buf();
    let stem = output_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("frames");
    let filename = format!("{}_frames.txt", stem);
    path.set_file_name(filename);
    path
}

fn write_concat_list<P: AsRef<Path>>(
    svgs: &[P],
    list_path: &Path,
    fps: u32,
) -> Result<(), Box<dyn Error>> {
    let frame_duration = 1.0f64 / fps as f64;
    let mut contents = String::new();
    for (index, svg) in svgs.iter().enumerate() {
        let svg_path = svg.as_ref();
        if !svg_path.exists() {
            return Err(format!("missing SVG frame: {}", svg_path.display()).into());
        }
        let abs_path = svg_path.canonicalize()?;
        let path_str = abs_path
            .to_str()
            .ok_or("SVG path contains non-UTF-8 characters")?;

        contents.push_str("file '");
        contents.push_str(path_str);
        contents.push_str("'\n");

        if index + 1 != svgs.len() {
            contents.push_str(&format!("duration {}\n", frame_duration));
        }
    }

    fs::write(list_path, contents)?;
    Ok(())
}
