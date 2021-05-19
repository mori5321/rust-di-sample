use crate::domain::todo::{Todo, TodoRepository};
use std::sync::Arc;

pub struct ListTodosUsecase {
    repository: Arc<dyn TodoRepository>,
}

impl ListTodosUsecase {
    pub fn new(repository: Arc<dyn TodoRepository>) -> Self {
        Self { repository }
    }

    pub fn run(&self) -> Vec<Todo> {
        self.repository.list()
    }
}
