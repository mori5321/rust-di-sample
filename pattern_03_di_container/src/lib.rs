mod adapters;
mod domain;
mod usecases;

#[cfg(test)]
mod tests {
    use super::usecases::list_todos_usecase::ListTodosUsecase;

    #[test]
    fn exec() {
        let usecase = ListTodosUsecase::new();
        let todos = usecase.run();

        assert_eq!(todos.len(), 3);
        assert_eq!(todos.get(0).unwrap().title, "Buy Oranges".to_string());
        assert_eq!(todos.get(1).unwrap().title, "Wash Dishes".to_string());
        assert_eq!(todos.get(2).unwrap().title, "Write Diary".to_string());
    }
}
