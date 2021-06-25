use std::{io::Write, path::PathBuf};

use crossbeam::channel::Receiver;
use serde::{Deserialize, Serialize};

use super::{super::base::*, validate, ValidationState};
use crate::{
    drawables::{black, BasicTextEditor, TextEditor, TextEditorInner}, prelude::*, save_system::SaveProfile
};

const WORKSHOP_FILE: &str = "workshop.yaml";
const CONFIG_FILE: &str = "config.jsonnet";
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkshopConfig {
    pub title:        String,
    pub description:  String,
    pub published_id: Option<u64>,
}

impl Default for WorkshopConfig {
    fn default() -> Self {
        Self {
            title:        "".to_string(),
            description:  "".to_string(),
            published_id: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Editors {
    Title,
    Description,
    Main,
}

#[derive(Debug)]
pub struct EditorState<Editor: TextEditor> {
    root:               PathBuf,
    title_editor:       BasicTextEditor,
    description_editor: BasicTextEditor,
    main_editor:        Editor,
    selected_editor:    Editors,
    tips_screen:        Rect,
    exiting:            bool,
    save_profile:       Arc<SaveProfile>,
    id_recv:            Option<Receiver<u64>>,
}

impl<Editor: TextEditor> EditorState<Editor> {
    pub fn new(root: PathBuf, save_profile: Arc<SaveProfile>) -> Self {
        let desc_h = 4;
        let w = W / 2 - 2;
        let title_i = 0;
        let desc_i = title_i + 4;
        let editor_i = desc_i + desc_h + 3;
        let buttons_h = 3;
        let title_editor = BasicTextEditor::new(
            "Title".to_string(),
            Rect::new(title_i + 2, 1, w - 5, 1),
            String::new(),
        );
        let mut description_editor = BasicTextEditor::new(
            "Description".to_string(),
            Rect::new(desc_i + 2, 1, w, desc_h),
            String::new(),
        );
        description_editor.set_cursor(false);
        let editor_rect = Rect::new(editor_i + 2, 1, w, H - editor_i - 3 - buttons_h);
        let mut main_editor = Editor::new(
            "Level config".to_string(),
            editor_rect.clone(),
            String::new(),
        );
        main_editor.set_cursor(false);
        let mut this = Self {
            root,
            title_editor,
            description_editor,
            main_editor,
            tips_screen: Rect::new(
                2,
                editor_rect.right() + 2,
                W - editor_rect.right() - 2,
                H - 2 - 4,
            ),
            selected_editor: Editors::Title,
            exiting: false,
            save_profile,
            id_recv: None,
        };
        this.reload_config();
        this
    }

    fn workshop_file(&self) -> PathBuf { self.root.join(WORKSHOP_FILE) }

    fn config_file(&self) -> PathBuf { self.root.join(CONFIG_FILE) }

    fn reload_config(&mut self) {
        let config = self.read_config();
        self.title_editor.load_string(config.title);
        self.description_editor.load_string(config.description);
        self.main_editor
            .load_file(Some(self.config_file()))
            .debug_unwrap();
    }

    fn read_config(&self) -> WorkshopConfig {
        match std::fs::File::open(self.workshop_file()).map(|f| serde_yaml::from_reader(f)) {
            Ok(Ok(config)) => config,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => WorkshopConfig::default(),
            err @ _ => {
                log::warn!("Failed to read workshop config! {:?}", err);
                debug_unreachable!();
                WorkshopConfig::default()
            },
        }
    }

    fn save_config(&self, new_id: Option<u64>) {
        let mut config = self.read_config();
        config.title = self.title_editor.to_string();
        config.description = self.description_editor.to_string();
        config.published_id = config.published_id.or(new_id);
        match std::fs::File::create(self.workshop_file()).map(|f| serde_yaml::to_writer(f, &config))
        {
            Ok(Ok(_)) => {},
            err @ _ => {
                log::warn!("Failed to write workshop config! {:?}", err);
                debug_unreachable!();
            },
        }
        std::fs::File::create(self.config_file())
            .and_then(|mut f| write!(f, "{}", self.main_editor.to_string()))
            .debug_expect("Failed to write config file.");
    }

    fn editor(&mut self) -> &mut dyn TextEditorInner {
        match self.selected_editor {
            Editors::Title => &mut self.title_editor,
            Editors::Description => &mut self.description_editor,
            Editors::Main => &mut self.main_editor,
        }
    }
}
fn inside_consider_border(mouse: &Pos, rect: &Rect) -> bool {
    mouse.inside(&Rect::new(
        rect.pos.i - 2,
        rect.pos.j - 1,
        rect.size.w + 2,
        rect.size.h + 3,
    ))
}

const SAVE: &str = "Save";
const RELOAD: &str = "Reload";
const VALIDATE: &str = "Validate";

const INSTRUCTIONS: &str = r#"
To create a level, first give it a name and a description, which will be shown in the Steam Workshop config.

The level config should be a JSON object which describes the levels. The most important fields of it are:

- test_cases: A list, each element must be a list with exactly two strings, a test case. The first element is a function that is applied to the user's solution, and the second must be the expected result.

- solutions: A list with at least one valid solution to the problem.

The syntax is JSONNET, a superset of JSON that allows easier writing of JSON files. Plain JSON also works fine.

Here's a simple of a level that accepts identity functions:
{
    test_cases: [["f: f a", "a"]],
    solutions: ["x: x"]
}

Press Validate to validate the level config is valid. If it is, you can play the level to test alternate solutions, or upload it to Steam. Once uploaded, further uploads will just edit it.

See the full level schema for more ways to customise your level.
"#;

const INST_BUTTONS: &[(&str, &str)] = &[
    ("See JSONNET syntax", "https://jsonnet.org/"),
    ("See full level schema", "https://yancouto.github.io/functional/functional/gamestates/level_creator/level_config/struct.UserLevelConfig.html"),
];

impl<Editor: TextEditor> GameState for EditorState<Editor> {
    fn name(&self) -> &'static str { "LevelEditor" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        self.title_editor.draw(&mut data);
        self.description_editor.draw(&mut data);
        self.main_editor.draw(&mut data);

        if data.button(SAVE, Pos::new(H - 3, 0), black()) {
            self.save_config(None);
        }
        if data.button(RELOAD, Pos::new(H - 3, SAVE.len() as i32 + 2), black()) {
            self.reload_config();
        }
        if data.button(
            VALIDATE,
            Pos::new(H - 3, (SAVE.len() + RELOAD.len()) as i32 + 4),
            black(),
        ) || (data.ctrl && data.pressed_key == Some(Key::Return))
        {
            self.save_config(None);
            let workshop = self.read_config();
            let (validator, recv) = ValidationState::new(
                validate(workshop.clone(), self.config_file()),
                self.save_profile.clone(),
                workshop,
            );
            self.id_recv = Some(recv);
            return GameStateEvent::Push(box validator);
        }

        data.text_box("Instructions", INSTRUCTIONS, self.tips_screen.clone(), true);

        let mut i = self.tips_screen.bottom();

        for (text, link) in INST_BUTTONS {
            i -= 3;
            if data.button(text, Pos::new(i, self.tips_screen.left() + 1), black()) {
                #[cfg(feature = "steam")]
                if let Some(client) = &data.steam_client {
                    client.friends().activate_game_overlay_to_web_page(link);
                }
            }
        }

        data.instructions(&[
            "Press CTRL+ENTER or the button to validate",
            "Press ESC to go back",
        ]);

        // Only need to try once
        if let Some(new_id) = self.id_recv.take().and_then(|r| r.try_recv().ok()) {
            self.save_config(Some(new_id));
        }

        if self.exiting {
            let but = data.box_with_options(
                "Exiting",
                "Do you want to save your changes?",
                Rect::centered(30, 20),
                &["Save", "Discard", "Cancel"],
            );
            if but == Some(0) {
                self.save_config(None);
                return GameStateEvent::Pop(1);
            } else if but == Some(1) {
                return GameStateEvent::Pop(1);
            } else if but == Some(2) {
                self.exiting = false;
            }
        }
        if data.pressed_key == Some(Key::Escape) {
            self.exiting = !self.exiting;
        }
        GameStateEvent::None
    }

    fn on_event(&mut self, event: bl::BEvent, input: &bl::Input) {
        if self.exiting {
            return;
        }
        if let bl::BEvent::MouseButtonDown { button: 0 } = event {
            let mouse = Pos::from_xy(input.mouse_tile_pos(0));
            let new_editor = if inside_consider_border(&mouse, self.title_editor.rect()) {
                Some(Editors::Title)
            } else if inside_consider_border(&mouse, self.description_editor.rect()) {
                Some(Editors::Description)
            } else if inside_consider_border(&mouse, self.main_editor.rect()) {
                Some(Editors::Main)
            } else {
                None
            };
            if let Some(editor) = new_editor {
                self.editor().set_cursor(false);
                self.selected_editor = editor;
                self.editor().set_cursor(true);
            }
        }
        self.editor().on_event(&event, input);
    }
}
