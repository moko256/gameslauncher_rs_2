use app_common::features::todo::todo_state::TodoState;
use eframe::egui::{self, Vec2};

#[derive(Default)]
pub struct MyApp {
    state: TodoState,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Todos");

            ui.horizontal(|ui| {
                let name_label = ui.label("New todo");

                let mut input = self.state.todo_content_add.clone();

                if ui
                    .add_sized(
                        ui.available_size() * Vec2::new(0.8, 1.0),
                        egui::TextEdit::singleline(&mut input),
                    )
                    .labelled_by(name_label.id)
                    .changed()
                {
                    self.state.on_input_todo_content(input);
                }

                if ui.button("Add").clicked() {
                    self.state.on_click_add();
                }
            });

            ui.label(self.state.todos.join("\n"));
        });
    }
}
