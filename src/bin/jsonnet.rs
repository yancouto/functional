use jsonnet::JsonnetVm;

fn main() {
    let mut vm = JsonnetVm::new();
    let result = vm.evaluate_file("src/levels/config/level_config.jsonnet");
    println!("{}", result.unwrap());
}
