use swcal_x64_meta as meta;
use std::env;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=call from build.rs");

    let out_dir = env::var("OUT_DIR").expect("The OUT_DIR environment variable must be set");
    let out_dir = Path::new(&out_dir);

    meta::generate_inst_list(out_dir);
    meta::generate_inst_emit(out_dir);
}
