use crate::repositories::{todos, users};
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
    // fn new_todos_repository(&self) -> Box<dyn todo_repository::TodoRepository> {
    //     if self.conf.db.enabled {
    //         return Box::new(todo_repository::TodoRepositoryForDB::new());
    //     } else {
    //         return Box::new(todo_repository::TodoRepositoryForMemory::new());
    //     }
    // }
    fn new_todos_repository(&self) -> Arc<dyn todos::TodoRepository> {
        if self.conf.db.enabled {
            return Arc::new(todos::TodoRepositoryForDB::new());
        }
        Arc::new(todos::TodoRepositoryForMemory::new())
    }

    fn new_users_repository(&self) -> Arc<dyn users::UserRepository> {
        if self.conf.db.enabled {
            return Arc::new(users::UserRepositoryForDB::new());
        }
        Arc::new(users::UserRepositoryForMemory::new())
    }

    pub fn create_server_data(&self) -> state::AppState {
        state::AppState {
            app_name: self.conf.app_name.clone(),
            todos_repo: self.new_todos_repository(),
            users_repo: self.new_users_repository(),
        }
    }
}
