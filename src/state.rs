use crate::repositories::todo_repository;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AppState {
    pub app_name: String,
    //pub repository: Box<dyn todo_repository::TodoRepository>,
}
