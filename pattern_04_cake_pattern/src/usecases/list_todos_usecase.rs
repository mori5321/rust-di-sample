use crate::domain::todo::{Todo, TodoRepository, UseTodoRepository};

pub trait ListTodosUsecase: UseTodoRepository {
    fn run(&self) -> Vec<Todo> {
        self.todo_repository().list()
    }
}
