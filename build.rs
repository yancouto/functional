use std::path::Path;

use build_deps::rerun_if_changed_paths;
use jsonnet::JsonnetVm;

fn get_level_config_json() -> String {
    let mut vm = JsonnetVm::new();
    vm.max_stack(1000);
    let out = vm
        .evaluate_file("src/levels/config/level_config.jsonnet")
        .expect("Failed to parse jsonnet")
        .to_string();
    out
}

fn main() {
    rerun_if_changed_paths("src/levels/config/**/*.jsonnet").unwrap();
    rerun_if_changed_paths("src/levels/config/**/*.libsonnet").unwrap();
    rerun_if_changed_paths("src/levels/config/**/*.json").unwrap();
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("level_config.json");
    std::fs::write(&dest_path, &get_level_config_json()).unwrap();
}
