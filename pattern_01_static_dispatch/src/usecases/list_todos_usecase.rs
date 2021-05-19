use crate::domain::todo::{Todo, TodoRepository};

pub struct ListTodosUsecase<Repository: TodoRepository> {
    repository: Repository,
}

impl<Repository: TodoRepository> ListTodosUsecase<Repository> {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub fn run(&self) -> Vec<Todo> {
        self.repository.list()
    }
}
