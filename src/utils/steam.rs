use parking_lot::{Condvar, Mutex};
use steamworks::*;

use crate::{levels::LEVELS, prelude::*, save_system::SaveProfile};

static LOADED: (Mutex<bool>, Condvar) = (Mutex::new(false), Condvar::new());

pub fn configure_user_stats(client: Arc<Client>) {
    let handle = box client
        .clone()
        .register_callback(move |s: UserStatsReceived| {
            *LOADED.0.lock() = true;
            LOADED.1.notify_all();
            if let Err(err) = s.result {
                log::error!("Failed to fetch user stats, ignoring: {}", err);
            } else {
                log::info!("Successfully loaded user stats");
            }
        });
    // It's fine for this to live forever
    Box::leak(handle);
    client.user_stats().request_current_stats();
}

pub fn update_all_achievements(client: Arc<Client>, profile: Arc<SaveProfile>) {
    std::thread::spawn(move || update_all_achievements_impl(client, profile));
}
fn update_all_achievements_impl(client: Arc<Client>, profile: Arc<SaveProfile>) {
    let mut lock = LOADED.0.lock();
    if *lock == false {
        LOADED.1.wait(&mut lock);
    }
    let info = profile.get_levels_info();
    let mut any = false;
    LEVELS.iter().for_each(|section| {
        let completed_all = section.levels.iter().all(|l| {
            info.get(&l.base.name)
                .map(|i| i.result.is_success())
                .unwrap_or(false)
        });
        let ach_name = format!("SECTION_{}", section.name)
            .to_uppercase()
            .replace(' ', "_");
        let user_stats = client.user_stats();
        let ach = user_stats.achievement(&ach_name);
        if completed_all && ach.get().debug_unwrap_or(false) == false {
            log::info!("Achieved all levels in section {}", section.name);
            ach.set().debug_unwrap();
            any = true;
        }
    });
    if any {
        client.user_stats().store_stats().debug_unwrap();
        log::debug!("Achievements updated");
    }
}
