use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub fn write_open_loop_html_preview<P: AsRef<Path>>(
    output_dir: P,
    video_filename: &str,
    initial_svg: Option<&str>,
    final_svg: Option<&str>,
) -> Result<PathBuf, Box<dyn Error>> {
    let output_dir = output_dir.as_ref();
    let html_path = output_dir.join("open_loop_preview.html");

    let video_path = output_dir.join(video_filename);
    if !video_path.exists() {
        return Err(format!("missing video file: {}", video_path.display()).into());
    }

    let mut html = String::new();
    html.push_str("<!doctype html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("  <meta charset=\"utf-8\">\n");
    html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n");
    html.push_str("  <title>Open-loop preview</title>\n");
    html.push_str("  <style>\n");
    html.push_str("    body { font-family: system-ui, -apple-system, sans-serif; margin: 24px; }\n");
    html.push_str("    .media { display: grid; gap: 16px; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); }\n");
    html.push_str("    figure { margin: 0; }\n");
    html.push_str("    img, video { max-width: 100%; height: auto; border: 1px solid #ddd; border-radius: 6px; }\n");
    html.push_str("  </style>\n");
    html.push_str("</head>\n<body>\n");
    html.push_str("  <h1>Open-loop simulation preview</h1>\n");
    html.push_str("  <p>Video preview:</p>\n");
    html.push_str(&format!(
        "  <video controls src=\"{}\"></video>\n",
        escape_html(video_filename)
    ));

    let initial_ref = initial_svg.and_then(|path| normalize_media_path(output_dir, path));
    let final_ref = final_svg.and_then(|path| normalize_media_path(output_dir, path));

    if initial_ref.is_some() || final_ref.is_some() {
        html.push_str("  <h2>Frame previews</h2>\n");
        html.push_str("  <div class=\"media\">\n");
        if let Some(path) = initial_ref {
            html.push_str("    <figure>\n");
            html.push_str("      <figcaption>Initial state</figcaption>\n");
            html.push_str(&format!(
                "      <img alt=\"Initial state\" src=\"{}\">\n",
                escape_html(&path)
            ));
            html.push_str("    </figure>\n");
        }
        if let Some(path) = final_ref {
            html.push_str("    <figure>\n");
            html.push_str("      <figcaption>Final state</figcaption>\n");
            html.push_str(&format!(
                "      <img alt=\"Final state\" src=\"{}\">\n",
                escape_html(&path)
            ));
            html.push_str("    </figure>\n");
        }
        html.push_str("  </div>\n");
    }

    html.push_str("</body>\n</html>\n");

    fs::write(&html_path, html)?;
    Ok(html_path)
}

fn escape_html(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn normalize_media_path(output_dir: &Path, path: &str) -> Option<String> {
    let candidate = Path::new(path);
    if candidate.is_absolute() {
        if candidate.exists() {
            candidate
                .file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.to_string())
        } else {
            None
        }
    } else {
        let resolved = output_dir.join(candidate);
        if resolved.exists() {
            Some(path.to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::write_open_loop_html_preview;
    use std::fs;

    #[test]
    fn test_write_open_loop_html_preview() {
        let temp_dir = tempfile::tempdir().expect("failed to create temp dir");
        let output_dir = temp_dir.path();
        fs::write(output_dir.join("open_loop.mp4"), b"video").expect("write video");
        fs::write(output_dir.join("initial_state.svg"), b"initial").expect("write initial svg");
        fs::write(output_dir.join("final_state.svg"), b"final").expect("write final svg");
        let html_path = write_open_loop_html_preview(
            output_dir,
            "open_loop.mp4",
            Some("initial_state.svg"),
            Some("final_state.svg"),
        )
        .expect("write html preview");

        let html = fs::read_to_string(html_path).expect("read html preview");
        assert!(html.contains("open_loop.mp4"));
        assert!(html.contains("initial_state.svg"));
        assert!(html.contains("final_state.svg"));
        assert!(html.contains("Open-loop simulation preview"));
    }
}
