use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::super::base::*;
use crate::{
    drawables::{BasicTextEditor, TextEditor}, prelude::*
};

const WORKSHOP_FILE: &str = "workshop.yaml";
#[derive(Debug, Default, Serialize, Deserialize)]
struct WorkshopConfig {
    title:        String,
    description:  String,
    published_id: Option<u64>,
}

#[derive(Debug, Clone, Copy)]
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
        Self {
            root,
            title_editor: BasicTextEditor::new(
                "Title".to_string(),
                Rect::new(title_i + 2, 1, 20, 1),
                "untitled".to_string(),
            ),
            description_editor: BasicTextEditor::new(
                "Description".to_string(),
                Rect::new(desc_i + 2, 1, w, desc_h),
                "Some description".to_string(),
            ),
            main_editor: Editor::new(
                "Level config".to_string(),
                Rect::new(editor_i + 2, 1, w, H - editor_i - 3 - buttons_h),
                "".to_string(),
            ),
            selected_editor: Editors::Title,
        }
    }

    fn workshop_file(&self) -> PathBuf { self.root.join(WORKSHOP_FILE) }

    fn read_config(&self) -> WorkshopConfig {
        match std::fs::File::open(self.workshop_file()).map(|f| serde_yaml::from_reader(f)) {
            Ok(Ok(config)) => config,
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
            if mouse.inside(self.title_editor.rect()) {
                self.selected_editor = Editors::Title;
            } else if mouse.inside(self.description_editor.rect()) {
                self.selected_editor = Editors::Description;
            } else if mouse.inside(self.main_editor.rect()) {
                self.selected_editor = Editors::Main;
            }
        }
        match self.selected_editor {
            Editors::Title => self.title_editor.on_event(&event, &input),
            Editors::Description => self.description_editor.on_event(&event, &input),
            Editors::Main => self.main_editor.on_event(&event, &input),
        }
    }
}
