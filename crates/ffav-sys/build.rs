use bindgen::{EnumVariation, MacroTypeVariation};
use std::{env, path::PathBuf};

fn main() {
    // Tell cargo to watch our wrapper header
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("^AV.*")
        .allowlist_function("^(av_|avformat_|avcodec_|avfilter_).*")
        .allowlist_var("^AV_.*")
        .allowlist_var("^AVERROR_.*")
        .allowlist_var("^AVFILTER_FLAG.*")
        .default_enum_style(EnumVariation::Rust {
            non_exhaustive: true,
        })
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .constified_enum(".*_FLAG_.*")
        .generate()
        .expect("Unable to generate bindings");

    let err_bindings = bindgen::Builder::default()
        .header_contents("err_wrap.h", "#include <errno.h>")
        .allowlist_var("^E.*")
        .default_macro_constant_type(MacroTypeVariation::Signed)
        .generate()
        .expect("Failed to generate error bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write out bindings");
    err_bindings
        .write_to_file(out_path.join("err_bindings.rs"))
        .expect("Failed to generate error bindings");

    pkg_config::Config::new()
        .atleast_version("58")
        //.statik(true)
        .probe("libavcodec")
        .unwrap();

    pkg_config::Config::new()
        .atleast_version("58")
        //.statik(true)
        .probe("libavformat")
        .unwrap();

    pkg_config::Config::new()
        .atleast_version("3")
        //.statik(true)
        .probe("libswresample")
        .unwrap();

    pkg_config::Config::new()
        .atleast_version("56")
        //.statik(true)
        .probe("libavutil")
        .unwrap();

    pkg_config::Config::new()
        .atleast_version("7")
        //.statik(true)
        .probe("libavfilter")
        .unwrap();
}
