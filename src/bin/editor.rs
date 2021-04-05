use functional::text_editor::interface::{start_xi_thread, CoreNotification, CoreRequest};
use simplelog::*;
use xi_core_lib::rpc::{EditCommand, EditRequest};

fn main() {
    TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    log::info!("hello!");
    let (s, r) = start_xi_thread();
    s.send_notification(CoreNotification::ClientStarted {
        config_dir:        None,
        client_extras_dir: None,
    });
    let re = s.send_request_block(CoreRequest::NewView {
        file_path: std::env::args().skip(1).next(),
    });
    println!("Got response {:?}", re);
    let id = re
        .unwrap()
        .as_str()
        .unwrap()
        .strip_prefix("view-id-")
        .unwrap()
        .to_owned()
        .parse::<usize>()
        .unwrap()
        .into();
    let re = s.send_request_block(CoreRequest::Edit(EditCommand {
        view_id: id,
        cmd:     EditRequest::Copy,
    }));
    println!("Got response {:?}", re);
    loop {
        while let Some(n) = r.next_notif() {
            println!("Notif: {:?}", n);
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    //t2.join().unwrap();
}
