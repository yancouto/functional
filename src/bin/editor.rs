use functional::text_editor::interface::{start_xi_thread, CoreNotification, CoreRequest};
use simplelog::*;

fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    log::info!("hello!");
    let (mut s, r) = start_xi_thread();
    s.send_notification(CoreNotification::ClientStarted {
        config_dir:        None,
        client_extras_dir: None,
    });
    s.send_request(
        CoreRequest::NewView {
            file_path: std::env::args().skip(1).next(),
        },
        |r| {
            println!("Got response {:?}", r);
        },
    );
    println!("Sent message");
    r.main_loop();
    //let t2 = std::thread::spawn(move || loop {
    //    let r = r.block_until_next_message();
    //    println!("Message: {:?}", r);
    //});
    //t2.join().unwrap();
}
