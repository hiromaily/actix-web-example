use crate::repositories::todo_repository;
use crate::state;
use crate::toml;
use std::sync::Arc;

#[allow(dead_code)]
pub struct Registry {
    pub conf: toml::Config,
    //pub app_repo: Box<dyn todo_repository::TodoRepository>,
}

#[allow(dead_code)]
impl Registry {
    pub fn new(conf: toml::Config) -> Self {
        Self { conf }
    }

    // error would occur if TodoRepository has clone trait as supertrait
    // fn new_repository(&self) -> Box<dyn todo_repository::TodoRepository> {
    //     if self.conf.db.enabled {
    //         return Box::new(todo_repository::TodoRepositoryForDB::new());
    //     } else {
    //         return Box::new(todo_repository::TodoRepositoryForMemory::new());
    //     }
    // }
    fn new_repository(&self) -> Arc<dyn todo_repository::TodoRepository> {
        if self.conf.db.enabled {
            return Arc::new(todo_repository::TodoRepositoryForDB::new());
        }
        Arc::new(todo_repository::TodoRepositoryForMemory::new())
    }

    pub fn create_server_data(&self) -> state::AppState {
        state::AppState {
            app_name: self.conf.app_name.clone(),
            repository: self.new_repository(),
        }
    }
}
