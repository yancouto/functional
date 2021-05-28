use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::super::base::*;
use crate::{
    drawables::{BasicTextEditor, TextEditor, TextEditorInner}, prelude::*
};

const WORKSHOP_FILE: &str = "workshop.yaml";
#[derive(Debug, Serialize, Deserialize)]
struct WorkshopConfig {
    title:        String,
    description:  String,
    published_id: Option<u64>,
}

impl Default for WorkshopConfig {
    fn default() -> Self {
        Self {
            title:        "some title here".to_string(),
            description:  "some description here".to_string(),
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
}

impl<Editor: TextEditor> EditorState<Editor> {
    pub fn new(root: PathBuf) -> Self {
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
        let mut main_editor = Editor::new(
            "Level config".to_string(),
            Rect::new(editor_i + 2, 1, w, H - editor_i - 3 - buttons_h),
            String::new(),
        );
        main_editor.set_cursor(false);
        let mut this = Self {
            root,
            title_editor,
            description_editor,
            main_editor,
            selected_editor: Editors::Title,
        };
        this.reload_config();
        this
    }

    fn workshop_file(&self) -> PathBuf { self.root.join(WORKSHOP_FILE) }

    fn reload_config(&mut self) {
        let config = self.read_config();
        self.title_editor.load_string(config.title);
        self.description_editor.load_string(config.description);
        // TODO: load main editor
    }

    fn read_config(&self) -> WorkshopConfig {
        match std::fs::File::open(self.workshop_file()).map(|f| serde_yaml::from_reader(f)) {
            Ok(Ok(config)) => config,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => WorkshopConfig::default(),
            err @ _ => {
                log::warn!("Failed to read workshop config! {:?}", err);
                debug_assert!(false);
                WorkshopConfig::default()
            },
        }
    }

    fn write_config(&self, config: &WorkshopConfig) {
        match std::fs::File::create(self.workshop_file()).map(|f| serde_yaml::to_writer(f, config))
        {
            Ok(Ok(_)) => {},
            err @ _ => {
                log::warn!("Failed to write workshop config! {:?}", err);
                debug_assert!(false);
            },
        }
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

impl<Editor: TextEditor> GameState for EditorState<Editor> {
    fn name(&self) -> &'static str { "LevelEditor" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        self.title_editor.draw(&mut data);
        self.description_editor.draw(&mut data);
        self.main_editor.draw(&mut data);
        GameStateEvent::None
    }

    fn on_event(&mut self, event: bl::BEvent, input: &bl::Input) {
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
