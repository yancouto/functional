use super::base::*;
use crate::{
    drawables::{black, TextEditor}, interpreter::{
        interpret, interpret_itermediates, parse, tokenize, ConstantProvider, InterpretError, Interpreted, Node, ParseError, TokenizeError
    }, prelude::*
};
#[derive(Debug)]
struct DebugData {
    steps:       Vec<Box<Node>>,
    interpreted: Result<Interpreted, InterpretError>,
}

#[derive(Debug)]
pub struct PlaygroundState<Editor: TextEditor> {
    editor:   Editor,
    data:     Option<Result<Result<DebugData, ParseError>, TokenizeError>>,
    provider: ConstantProvider,
}

const EDITOR_W: i32 = 40;
const MAX_STEPS: usize = H as usize - 5;

impl<Editor: TextEditor> PlaygroundState<Editor> {
    pub fn new(initial_text: String, provider: ConstantProvider) -> Self {
        Self {
            editor: Editor::new(
                "Playground".to_string(),
                Rect::new(2, 1, EDITOR_W - 2, H - 6),
                initial_text,
            ),
            data: None,
            provider,
        }
    }

    fn print_run_details(&mut self, data: &mut TickData) {
        let txt = if let Some(eval) = &self.data {
            match eval {
                Ok(token) => match token {
                    Ok(d) => {
                        let steps_txt = d
                            .steps
                            .iter()
                            .map(|term| term.to_string())
                            .collect::<Vec<_>>()
                            .join("\n\n");
                        let mut txt = match &d.interpreted {
                            Ok(i) => {
                                let reds = i.stats.reductions;
                                let mut txt =
                                    format!("Interpreted successfully. Reductions: {}", reds);
                                if reds as usize > 10 {
                                    txt.push_str(&format!("\n\nFinal result: {}", i.term));
                                }
                                txt
                            },
                            Err(e) => format!("Failed to interpret: {}", e),
                        };
                        if !steps_txt.is_empty() {
                            txt.push_str(&format!("\n\nStep by step reduction:\n\n{}", steps_txt));
                        }
                        txt
                    },
                    Err(e) => format!("Failed to parse input: {}", e),
                },
                Err(e) => {
                    format!("Failed to tokenize input: {}", e)
                },
            }
        } else {
            "Evaluate some term to see results here...\n\nAll code loaded on playground is lost when it's closed.".to_string()
        };
        data.text_box(
            "Run details",
            &txt,
            Rect::new(0, EDITOR_W, W - EDITOR_W, H),
            false,
        );
    }

    fn eval(&mut self) {
        self.data = Some(tokenize(self.editor.to_string().chars()).map(|tokens| {
            parse(tokens).map(|term| DebugData {
                steps:       std::iter::once(term.clone())
                    .chain(interpret_itermediates(term.clone(), false, self.provider))
                    .take(MAX_STEPS)
                    .collect(),
                interpreted: interpret(term, false, self.provider),
            })
        }));
    }
}

impl<Editor: TextEditor> GameState for PlaygroundState<Editor> {
    fn name(&self) -> &'static str { "Playground" }

    fn tick(&mut self, mut data: TickData) -> GameStateEvent {
        self.editor.draw(&mut data);

        data.instructions(&["Press ESC to go back", "Press CTRL+ENTER to evaluate"]);

        if data.button("Evaluate", Pos::new(H - 3, 0), black())
            || (data.ctrl && data.pressed_key == Some(Key::Return))
        {
            self.eval();
        }

        self.print_run_details(&mut data);

        if data.pressed_key == Some(Key::Escape) {
            GameStateEvent::Pop(1)
        } else {
            GameStateEvent::None
        }
    }

    fn on_event(&mut self, event: bl::BEvent, input: &bl::Input) {
        self.editor.on_event(&event, input);
    }
}
