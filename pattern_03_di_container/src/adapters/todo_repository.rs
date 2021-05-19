use crate::domain::todo::{Todo, TodoRepository};
use waiter_di::*;

#[component]
#[derive(Clone)]
pub struct TodoRepositoryOnMemory {}

impl TodoRepositoryOnMemory {
    pub fn new() -> Self {
        Self {}
    }
}

#[provides]
impl TodoRepository for TodoRepositoryOnMemory {
    fn list(&self) -> Vec<Todo> {
        vec![
            Todo::new("Buy Oranges".to_string(), false),
            Todo::new("Wash Dishes".to_string(), false),
            Todo::new("Write Diary".to_string(), false),
        ]
    }
}
