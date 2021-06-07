use crossbeam::channel::*;
use derivative::Derivative;
use thiserror::Error;

use super::{super::base::*, ParsedUserLevelConfig, WorkshopConfig};
use crate::{drawables::black, prelude::*};

#[derive(Debug)]
enum State {
    Starting,
    CreatingItem,
    Uploading,
    Finished(u64),
    Error(UploadError),
}

impl State {
    fn is_final(&self) -> bool {
        match &self {
            Self::Finished(..) | Self::Error(..) => true,
            _ => false,
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct UploadingLevelState {
    #[derivative(Debug = "ignore")]
    client:     Arc<SteamClient>,
    state:      State,
    state_recv: Receiver<State>,
}

#[derive(Error, Debug)]
enum UploadError {
    #[error("Channel got disconnected")]
    ChannelDisconnected,
    #[cfg(feature = "steam")]
    #[error("Error calling Steam: {0}")]
    SteamError(#[from] steamworks::SteamError),
    #[error("Error interacting with filesystem: {0}")]
    FilesystemError(#[from] std::io::Error),
    #[error("Error serializing level: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[allow(dead_code)]
    #[error("Unknown error")]
    UnknownError,
}

fn open_file_page(client: &SteamClient, id: u64) {
    #[cfg(feature = "steam")]
    client
        .friends()
        .activate_game_overlay_to_web_page(&format!("steam://url/CommunityFilePage/{}", id));
}

#[cfg(feature = "steam")]
fn upload_level_impl(
    id_sender: Sender<u64>,
    state_sender: Sender<State>,
    level: ParsedUserLevelConfig,
    client: Arc<SteamClient>,
    config: WorkshopConfig,
) -> Result<u64, UploadError> {
    use steamworks::*;
    // Create item if it doesn't exist
    let published_id = if let Some(id) = config.published_id {
        Ok(id)
    } else {
        state_sender.send(State::CreatingItem).debug_unwrap();
        let (send, recv) = bounded(1);
        client
            .ugc()
            .create_item(client.utils().app_id(), FileType::Community, move |r| {
                send.send(r).debug_unwrap()
            });
        if let Ok(r) = recv.recv() {
            Ok(r.map(|(id, needs_legal_agreement)| {
                if needs_legal_agreement {
                    log::warn!("User needs to accept workshop legal agreement");
                    open_file_page(&client, id.0);
                }
                id.0
            })?)
        } else {
            Err(UploadError::ChannelDisconnected)
        }
    }?;
    id_sender.send(published_id).debug_unwrap();
    state_sender.send(State::Uploading).debug_unwrap();
    let (send, recv) = bounded(1);
    let cache = crate::save_system::PROJECT_DIR
        .cache_dir()
        .join(format!("{}", published_id));
    std::fs::create_dir_all(&cache)?;

    std::fs::write(cache.join("level.json"), serde_json::to_vec(&level)?)?;
    log::debug!("Level stored in dir {:?}", cache);
    client
        .ugc()
        .start_item_update(client.utils().app_id(), PublishedFileId(published_id))
        .title(&config.title)
        .description(&config.description)
        .content_path(&cache)
        .submit(
            Some(&format!("Updating by {}", client.user().steam_id().raw())),
            move |r| send.send(r).debug_unwrap(),
        );
    recv.recv()
        .map_err(|_| UploadError::ChannelDisconnected)??;
    std::fs::remove_dir_all(cache)?;
    Ok(published_id)
}

fn upload_level(
    id_sender: Sender<u64>,
    state_sender: Sender<State>,
    level: ParsedUserLevelConfig,
    client: Arc<SteamClient>,
    config: WorkshopConfig,
) {
    #[cfg(feature = "steam")]
    {
        std::thread::spawn(move || {
            match upload_level_impl(id_sender, state_sender.clone(), level, client, config) {
                Err(err) => {
                    log::error!("Failed to upload: {}", err);
                    state_sender.send(State::Error(err)).debug_unwrap();
                },
                Ok(id) => state_sender.send(State::Finished(id)).debug_unwrap(),
            }
        });
    }
}

impl UploadingLevelState {
    pub fn new(
        level: ParsedUserLevelConfig,
        client: Arc<SteamClient>,
        config: WorkshopConfig,
    ) -> (Self, Receiver<u64>) {
        let (send_id, recv_id) = bounded(1);
        let (send, recv) = unbounded();
        upload_level(send_id, send, level, client.clone(), config);
        (
            Self {
                state: State::Starting,
                state_recv: recv,
                client,
            },
            recv_id,
        )
    }
}

impl GameState for UploadingLevelState {
    fn name(&self) -> &'static str { "Uploading level" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        while let Ok(s) = self.state_recv.try_recv() {
            self.state = s;
        }
        let rect = Rect::centered(60, 30);
        data.text_box(
            "Uploading to steam",
            &match &self.state {
                State::Starting => "Starting...".to_string(),
                State::CreatingItem => "Creating item...".to_string(),
                State::Uploading => "Uploading...".to_string(), // If necessary, we can do percentage
                State::Finished(_) => "Upload complete!".to_string(),
                State::Error(err) => format!("Error uploading: {}", err),
            },
            rect,
            true,
        );
        if self.state.is_final() {
            data.instructions(&["Press ESC to go back"]);
            if data.pressed_key == Some(Key::Escape) {
                return GameStateEvent::Pop(1);
            }
        }
        if let State::Finished(id) = &self.state {
            if data.button(
                "Open page",
                Pos::new(rect.bottom() - 3, rect.pos.j + 1),
                black(),
            ) {
                open_file_page(&self.client, *id);
            }
        }
        GameStateEvent::None
    }
}
