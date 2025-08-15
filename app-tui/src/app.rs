use app_common::features::todo::todo_state::TodoState;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::Stylize,
    widgets::Paragraph,
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
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ]);
        let [todos_area, footer_area, input_area] = vertical.areas(frame.area());

        let buffer_height = todos_area.height;
        let list_height = self.state.todos.len() as u16;
        if buffer_height > list_height {
            let empty_placeholder_height = buffer_height - list_height;
            let empty_placeholder_top = todos_area.top() + buffer_height - empty_placeholder_height;

            for top in empty_placeholder_top..empty_placeholder_top + empty_placeholder_height {
                let mut placeholder_line_area = Clone::clone(&todos_area);
                placeholder_line_area.y = top;

                frame.render_widget(Paragraph::new("~"), placeholder_line_area);
            }
        }

        let todos_text = self.state.todos.join("\n");
        frame.render_widget(Paragraph::new(todos_text), todos_area);

        let footer_text: &'static str = "[Todos] Press `Esc`or `Ctrl-C` to stop running.";
        frame.render_widget(Paragraph::new(footer_text).reversed().bold(), footer_area);

        frame.render_widget(
            Paragraph::new(format!("New todo: {}", self.state.todo_content_add)),
            input_area,
        );
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
