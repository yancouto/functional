use functional::text_editor::interface::{start_xi_thread, CoreNotification};
use simplelog::*;

fn main() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    log::info!("hello!");
    let (s, r) = start_xi_thread();
    s.send(
        CoreNotification::ClientStarted {
            config_dir:        None,
            client_extras_dir: None,
        }
        .into(),
    )
    .unwrap();
    println!("Sent message");
    let t2 = std::thread::spawn(move || loop {
        let r = r.recv().unwrap();
        println!("Message: {:?}", r);
    });
    t2.join().unwrap();
}
