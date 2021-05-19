#[derive(Debug)]
pub struct Todo {
    pub title: String,
    pub is_done: bool,
}

impl Todo {
    pub fn new(title: String, is_done: bool) -> Self {
        Self { title, is_done }
    }
}

pub trait TodoRepository {
    fn list(&self) -> Vec<Todo>;
}

pub trait UseTodoRepository {
    type Repository: TodoRepository;
    fn todo_repository(&self) -> Self::Repository;
}
