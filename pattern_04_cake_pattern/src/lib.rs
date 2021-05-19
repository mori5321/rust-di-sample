mod adapters;
mod domain;
mod usecases;

#[cfg(test)]
mod tests {
    use super::adapters::todo_repository::TodoRepositoryOnMemory;
    use super::domain::todo::UseTodoRepository;
    use super::usecases::list_todos_usecase::ListTodosUsecase;

    struct ListTodosUsecaseImpl {}
    impl ListTodosUsecaseImpl {
        fn new() -> Self {
            Self {}
        }
    }

    impl ListTodosUsecase for ListTodosUsecaseImpl {}
    impl UseTodoRepository for ListTodosUsecaseImpl {
        type Repository = TodoRepositoryOnMemory;
        fn todo_repository(&self) -> Self::Repository {
            TodoRepositoryOnMemory::new()
        }
    }

    #[test]
    fn exec() {
        let usecase = ListTodosUsecaseImpl::new();
        let todos = usecase.run();

        assert_eq!(todos.len(), 3);
        assert_eq!(todos.get(0).unwrap().title, "Buy Oranges".to_string());
        assert_eq!(todos.get(1).unwrap().title, "Wash Dishes".to_string());
        assert_eq!(todos.get(2).unwrap().title, "Write Diary".to_string());
    }
}
