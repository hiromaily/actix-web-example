use crate::repositories::todos;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct AppState {
    pub app_name: String,
    pub repository: Arc<dyn todos::TodoRepository>,
}
