use jsonnet::JsonnetVm;
use std::path::Path;

fn get_level_config_json() -> String {
    let mut vm = JsonnetVm::new();
    let out = vm
        .evaluate_file("src/levels/config/level_config.jsonnet")
        .expect("Failed to parse jsonnet")
        .to_string();
    out
}

fn main() {
    println!("cargo:rerun-if-changed=src/levels/config");
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("level_config.json");
    std::fs::write(&dest_path, &get_level_config_json()).unwrap();
}
