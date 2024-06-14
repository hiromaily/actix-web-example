use crate::repositories::{todos, users};
use crate::state;
use crate::toml;
use crate::usecases::{admin, app};
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

    // TODO: return usecase trait object

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

    fn create_admin_usecase(&self) -> Arc<dyn admin::AdminUsecase> {
        Arc::new(admin::AdminAction::new(
            self.new_todos_repository(),
            self.new_users_repository(),
        ))
    }

    fn create_app_usecase(&self) -> Arc<dyn app::AppUsecase> {
        Arc::new(app::AppAction::new(
            self.new_todos_repository(),
            self.new_users_repository(),
        ))
    }

    pub fn create_global_state(&self) -> state::GlobalState {
        state::GlobalState {
            app_name: self.conf.app_name.clone(),
        }
    }

    pub fn create_admin_state(&self) -> state::AdminState {
        state::AdminState {
            admin_usecase: self.create_admin_usecase(),
        }
    }

    pub fn create_app_state(&self) -> state::AppState {
        state::AppState {
            app_usecase: self.create_app_usecase(),
        }
    }
}
