use crate::repositories::todo_repository;
use crate::state;
use crate::toml;

pub struct Registry {
    pub conf: toml::Config,
    //pub app_repo: Box<dyn todo_repository::TodoRepository>,
}

impl Registry {
    pub fn new(conf: toml::Config) -> Self {
        Self { conf }
    }

    #[allow(dead_code)]
    fn new_repository(&self) -> Box<dyn todo_repository::TodoRepository> {
        if self.conf.pg.enabled {
            return Box::new(todo_repository::TodoRepositoryForDB::new());
        } else {
            return Box::new(todo_repository::TodoRepositoryForMemory::new());
        }
    }

    pub fn create_server_data(&self) -> state::AppState {
        state::AppState {
            app_name: String::from("Actix Web"),
            repository: self.new_repository(),
        }
    }
}
