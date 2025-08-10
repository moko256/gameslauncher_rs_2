use app_common::features::todo::todo_state::TodoState;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph},
};

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    redraw: bool,
    state: TodoState,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.redraw = true;

        self.running = true;
        while self.running {
            if self.redraw {
                terminal.draw(|frame| self.render(frame))?;

                self.redraw = false;
            }
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn redraw(&mut self) {
        self.redraw = true;
    }

    fn render(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(1),
        ]);
        let [input_area, todos_area, footer_area] = vertical.areas(frame.area());

        let input_title = Line::from("New todo:").bold();
        frame.render_widget(
            Paragraph::new(self.state.todo_content_add.as_str())
                .block(Block::bordered().title(input_title))
                .yellow(),
            input_area,
        );

        let todos_title = Line::from("Todos:").bold();
        let todos_text = self.state.todos.join("\n");
        frame.render_widget(
            Paragraph::new(todos_text).block(Block::bordered().title(todos_title)),
            todos_area,
        );

        let footer_text: &'static str = "Press `Esc`or `Ctrl-C` to stop running.";
        frame.render_widget(Paragraph::new(footer_text), footer_area);
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                self.on_key_event(key);
            }
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {
                self.redraw();
            }
            _ => {}
        }

        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc)
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, KeyCode::Char(char)) => {
                let new_input = self.state.todo_content_add.clone() + &String::from(char);

                self.state.on_input_todo_content(new_input);
                self.redraw();
            }
            (_, KeyCode::Backspace) => {
                let len = self.state.todo_content_add.len();
                let new_input = if len <= 1 {
                    String::new()
                } else {
                    self.state.todo_content_add[..(len - 1)].to_string()
                };

                self.state.on_input_todo_content(new_input);
                self.redraw();
            }
            (_, KeyCode::Enter) => {
                self.state.on_click_add();
                self.redraw();
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
