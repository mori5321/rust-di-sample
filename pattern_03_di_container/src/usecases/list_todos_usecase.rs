use crate::domain::todo::{Todo, TodoRepository};
use std::sync::Arc;

use waiter_di::*;

#[module]
pub struct ListTodosUsecase {
    repository: Arc<dyn TodoRepository>,
}

impl ListTodosUsecase {
    pub fn new() -> Self {
        let mut container = Container::<Arc<dyn TodoRepository>>::new();
        Provider::<ListTodosUsecase>::create(&mut container)
    }

    pub fn run(&self) -> Vec<Todo> {
        self.repository.list()
    }
}
