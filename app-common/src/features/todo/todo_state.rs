#[derive(Debug, Default)]
pub struct TodoState {
    pub todos: Vec<String>,
    pub todo_content_add: String,
}

impl TodoState {
    pub fn on_input_todo_content(&mut self, content: String) {
        self.todo_content_add = content;
    }

    pub fn on_click_add(&mut self) {
        let mut new_todos = self.todos.clone();

        let mut new_todo_content = String::new();
        std::mem::swap(&mut self.todo_content_add, &mut new_todo_content);

        new_todos.push(new_todo_content);

        self.todos = new_todos;
    }
}
