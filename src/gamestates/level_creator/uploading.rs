use crossbeam::channel::*;
use thiserror::Error;

use super::{super::base::*, WorkshopConfig};
use crate::prelude::*;

#[derive(Debug)]
enum State {
    Starting,
    CreatingItem,
    Uploading,
    Finished,
    Error(UploadError),
}

impl State {
    fn is_final(&self) -> bool {
        match &self {
            Self::Finished | Self::Error(..) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct UploadingLevelState {
    state:      State,
    state_recv: Receiver<State>,
}

#[derive(Error, Debug, Clone)]
enum UploadError {
    #[error("Channel got disconnected")]
    ChannelDisconnected,
    #[cfg(feature = "steam")]
    #[error("Error calling Steam: {0}")]
    SteamError(#[from] steamworks::SteamError),
    #[allow(dead_code)]
    #[error("Unknown error")]
    UnknownError,
}

#[cfg(feature = "steam")]
fn upload_level_impl(
    id_sender: Sender<u64>,
    state_sender: Sender<State>,
    client: Arc<SteamClient>,
    config: WorkshopConfig,
) -> Result<(), UploadError> {
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
                    client.friends().activate_game_overlay_to_web_page(&format!(
                        "steam://url/CommunityFilePage/{}",
                        id.0
                    ));
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
    client
        .ugc()
        .start_item_update(client.utils().app_id(), PublishedFileId(published_id))
        .title(&config.title)
        .description(&config.description)
        .submit(
            Some(&format!("Updating by {}", client.user().steam_id().raw())),
            move |r| send.send(r).debug_unwrap(),
        );
    recv.recv()
        .map_err(|_| UploadError::ChannelDisconnected)??;
    Ok(())
}

fn upload_level(
    id_sender: Sender<u64>,
    state_sender: Sender<State>,
    client: Arc<SteamClient>,
    config: WorkshopConfig,
) {
    #[cfg(feature = "steam")]
    {
        std::thread::spawn(move || {
            if let Err(err) = upload_level_impl(id_sender, state_sender.clone(), client, config) {
                log::error!("Failed to upload: {}", err);
                state_sender.send(State::Error(err)).debug_unwrap();
            } else {
                state_sender.send(State::Finished).debug_unwrap();
            }
        });
    }
}

impl UploadingLevelState {
    pub fn new(client: Arc<SteamClient>, config: WorkshopConfig) -> (Self, Receiver<u64>) {
        let (send_id, recv_id) = bounded(1);
        let (send, recv) = unbounded();
        upload_level(send_id, send, client, config);
        (
            Self {
                state:      State::Starting,
                state_recv: recv,
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
        data.text_box(
            "Uploading to steam",
            &match &self.state {
                State::Starting => "Starting...".to_string(),
                State::CreatingItem => "Creating item...".to_string(),
                State::Uploading => "Uploading...".to_string(), // If necessary, we can do percentage
                State::Finished => "Upload complete!".to_string(),
                State::Error(err) => format!("Error uploading: {}", err),
            },
            Rect::centered(60, 70),
            true,
        );
        if self.state.is_final() {
            data.instructions(&["Press ESC to go back"]);
            if data.pressed_key == Some(Key::Escape) {
                return GameStateEvent::Pop(1);
            }
        }
        GameStateEvent::None
    }
}
