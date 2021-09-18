use std::{env, fs, path::Path};

fn main() {
    let avformat = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libavformat")
        .unwrap();

    let avcodec = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libavcodec")
        .unwrap();

    let avutil = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libavutil")
        .unwrap();

    let avfilter = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libavfilter")
        .unwrap();

    let swresample = pkg_config::Config::new()
        .cargo_metadata(false)
        .probe("libswresample")
        .unwrap();

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_file = Path::new(&out_dir).join("libversions.rs");

    let mut output = String::new();

    for (lib, var) in [
        (avformat, "LIBAVFORMAT"),
        (avcodec, "LIBAVCODEC"),
        (avutil, "LIBAVUTIL"),
        (avfilter, "LIBAVFILTER"),
        (swresample, "LIBSWRESAMPLE"),
    ] {
        let parts: Vec<_> = lib.version.split('.').collect();

        let major = parts.get(0).unwrap_or(&"0");
        let minor = parts.get(1).unwrap_or(&"0");
        let patch = parts.get(2).unwrap_or(&"0");

        output.push_str(&format!(
            r#"
            
const {}: Version = Version {{
major: {},
minor: {},
patch: {},
    }};"#,
            var, major, minor, patch
        ));
    }

    fs::write(out_file, output).expect("Failed to write to version.rs");
}
