use tempfile::TempDir;

pub fn temp_output_dir(prefix: &str) -> TempDir {
    tempfile::Builder::new()
        .prefix(prefix)
        .tempdir()
        .expect("failed to create temp dir")
}
