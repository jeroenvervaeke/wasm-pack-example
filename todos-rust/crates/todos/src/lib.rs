pub use redux_rs;

use std::collections::VecDeque;

pub struct Todo {
    id: u32,
    completed: bool,
    text: String
}

impl Todo {
    pub fn new(id: u32, text: String) -> Self {
        Self { id, completed: false, text }
    }
}

pub struct TodoState {
    next_id: u32,
    todos: VecDeque<Todo>
}

impl TodoState {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            todos: Default::default()
        }
    }

    pub fn add_todo(&mut self, text: String) -> u32 {
        let id = self.next_id;

        self.next_id += 1;
        self.todos.push_back(Todo::new(id, text));

        id
    }
}

impl Default for TodoState {
    fn default() -> Self {
        Self::new()
    }
}

pub enum TodoAction {

}

pub fn todo_reducer(state: TodoState, action: TodoAction) -> TodoState {
    todo!()
}