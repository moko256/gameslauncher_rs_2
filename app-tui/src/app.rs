use app_common::features::launcher::launcher_state::LauncherState;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Stylize},
    widgets::{List, ListState, Paragraph},
};

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    redraw: bool,
    state: LauncherState,
    list_state: ListState,
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
                self.redraw = false;

                self.before_render();
                terminal.draw(|frame| self.render(frame))?;
                self.after_render();
            }
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn redraw(&mut self) {
        self.redraw = true;
    }

    fn before_render(&mut self) {
        if self.list_state.selected() != self.state.selected_index {
            self.list_state.select(self.state.selected_index);
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ]);
        let [list_area, footer_area, input_area] = vertical.areas(frame.area());

        let items: Vec<&str> = self
            .state
            .data_filtered
            .iter()
            .map(|s| s.as_str())
            .collect();

        let list = List::new(items)
            .style(Color::White)
            .highlight_style(Modifier::REVERSED)
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, list_area, &mut self.list_state);

        let footer_text: &'static str =
            "[gameslauncher_rs_2] Press `Esc`or `Ctrl-C` to stop running.";
        frame.render_widget(Paragraph::new(footer_text).reversed().bold(), footer_area);

        frame.render_widget(
            Paragraph::new(format!("{}", self.state.filter_query)),
            input_area,
        );
    }

    fn after_render(&mut self) {
        if !self.state.loaded {
            self.state.fetch_data();
        }
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
                let new_filter = self.state.filter_query.clone() + &String::from(char);

                self.state.update_filter(new_filter);
                self.redraw();
            }
            (_, KeyCode::Backspace) => {
                let len = self.state.filter_query.len();
                let new_filter = if len <= 1 {
                    String::new()
                } else {
                    self.state.filter_query[..(len - 1)].to_string()
                };

                self.state.update_filter(new_filter);
                self.redraw();
            }
            (_, KeyCode::Enter) => {
                self.state.launch_selected();
                self.redraw();
            }
            (_, KeyCode::Up) => {
                self.list_state.select_previous();
                self.state.update_selected(self.list_state.selected());
                self.redraw();
            }
            (_, KeyCode::Down) => {
                self.list_state.select_next();
                self.state.update_selected(self.list_state.selected());
                self.redraw();
            }
            _ => {}
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
