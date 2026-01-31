use std::fs;
use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> std::path::PathBuf {
    let mut dir = std::env::temp_dir();
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    dir.push(format!("lap_simulation_test_{}_{}", std::process::id(), nanos));
    fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn normalize_svg(bytes: &[u8]) -> String {
    let mut s = String::from_utf8_lossy(bytes).to_string();
    s = s.replace('\r', "");

    // Remove XML comments.
    while let Some(start) = s.find("<!--") {
        if let Some(end) = s[start + 4..].find("-->") {
            let end = start + 4 + end + 3;
            s.replace_range(start..end, "");
        } else {
            break;
        }
    }

    // Strip attributes that can be nondeterministic between runs.
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == 'i' && chars.peek() == Some(&'d') {
            let mut lookahead = chars.clone();
            if lookahead.next() == Some('d') && lookahead.next() == Some('=') && lookahead.next() == Some('"') {
                chars.next(); // d
                chars.next(); // =
                chars.next(); // "
                while let Some(ch) = chars.next() {
                    if ch == '"' {
                        break;
                    }
                }
                continue;
            }
        }
        if c == 'c' && chars.peek() == Some(&'l') {
            let mut lookahead = chars.clone();
            let prefix = ['l', 'i', 'p', '-', 'p', 'a', 't', 'h', '=', '"', 'u', 'r', 'l', '(', '#'];
            if prefix.iter().all(|p| lookahead.next() == Some(*p)) {
                for _ in 0..prefix.len() {
                    chars.next();
                }
                while let Some(ch) = chars.next() {
                    if ch == '"' {
                        break;
                    }
                }
                continue;
            }
        }
        out.push(c);
    }

    // Remove whitespace between tags to stabilize formatting differences.
    let mut compact = String::with_capacity(out.len());
    let mut iter = out.chars().peekable();
    while let Some(ch) = iter.next() {
        if ch == '>' {
            compact.push(ch);
            while matches!(iter.peek(), Some(' ' | '\n' | '\t')) {
                iter.next();
            }
            if let Some('<') = iter.peek() {
                continue;
            }
        } else {
            compact.push(ch);
        }
    }

    compact
}

#[test]
fn main_produces_expected_svgs() {
    let temp_dir = unique_temp_dir();
    let exe = env!("CARGO_BIN_EXE_lap_simulation");

    let status = Command::new(exe)
        .current_dir(&temp_dir)
        .status()
        .expect("failed to run lap_simulation binary");
    assert!(status.success(), "lap_simulation binary failed");

    let initial_path = temp_dir.join("initial_state.svg");
    let final_path = temp_dir.join("final_state.svg");

    let initial_bytes = fs::read(&initial_path).expect("missing initial_state.svg");
    let final_bytes = fs::read(&final_path).expect("missing final_state.svg");
    assert!(!initial_bytes.is_empty(), "initial_state.svg is empty");
    assert!(!final_bytes.is_empty(), "final_state.svg is empty");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let expected_initial = fs::read(
        manifest_dir
            .join("tests")
            .join("fixtures")
            .join("initial_state.svg"),
    )
    .expect("missing golden initial_state.svg");
    let expected_final = fs::read(
        manifest_dir
            .join("tests")
            .join("fixtures")
            .join("final_state.svg"),
    )
    .expect("missing golden final_state.svg");

    assert_eq!(
        normalize_svg(&initial_bytes),
        normalize_svg(&expected_initial),
        "initial_state.svg differs from golden output"
    );
    assert_eq!(
        normalize_svg(&final_bytes),
        normalize_svg(&expected_final),
        "final_state.svg differs from golden output"
    );

    let _ = fs::remove_dir_all(&temp_dir);
}
