use std::path::Path;

use build_deps::rerun_if_changed_paths;
use jsonnet::JsonnetVm;

fn get_level_config_json() -> String {
    match std::process::Command::new("jsonnet")
        .args(&[
            "-J",
            "src/levels/config",
            "src/levels/config/level_config.jsonnet",
        ])
        .output()
    {
        Ok(o) if !o.stdout.is_empty() => String::from_utf8(o.stdout).unwrap(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let mut vm = JsonnetVm::new();
            let out = vm
                .evaluate_file("src/levels/config/level_config.jsonnet")
                .expect("Failed to parse jsonnet")
                .to_string();
            out
        },
        Ok(o) => panic!("{}", String::from_utf8_lossy(&o.stderr)),
        Err(e) => panic!("Failed to run {:?}", e),
    }
}

fn main() {
    rerun_if_changed_paths("src/levels/config/**/*.jsonnet").unwrap();
    rerun_if_changed_paths("src/levels/config/**/*.libsonnet").unwrap();
    rerun_if_changed_paths("src/levels/config/**/*.json").unwrap();
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("level_config.json");
    std::fs::write(&dest_path, &get_level_config_json()).unwrap();
}
