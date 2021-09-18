use std::{env, fs, path::Path};

fn main() {
    let avformat = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libavformat")
        .unwrap();

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_file = Path::new(&out_dir).join("libversions.rs");

    let parts: Vec<_> = avformat.version.split(".").collect();
    if parts.len() != 3 {
        eprintln!(
            "libavformat version does not contain expected Major.Minor.Patch format ({})",
            avformat.version
        );
    }

    fs::write(
        out_file,
        format!(
            r#"const LIBAVFORMAT: Version = Version {{
major: {},
minor: {},
patch: {},
    }};"#,
            parts[0], parts[1], parts[2]
        ),
    )
    .expect("Failed to write to version.rs");
}
