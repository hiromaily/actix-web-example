use crate::repositories::{todos, users};
use crate::state;
use crate::toml;
use crate::usecases::{admin, app};
use std::sync::Arc;

// error would occur if TodoRepository has clone trait as supertrait
// fn new_todos_repository(&self) -> Box<dyn todo_repository::TodoRepository> {
//     if self.conf.db.enabled {
//         return Box::new(todo_repository::TodoRepositoryForDB::new());
//     } else {
//         return Box::new(todo_repository::TodoRepositoryForMemory::new());
//     }
// }
fn new_todos_repository(db: &toml::PostgreSQL) -> Arc<dyn todos::TodoRepository> {
    if db.enabled {
        return Arc::new(todos::TodoRepositoryForDB::new());
    }
    Arc::new(todos::TodoRepositoryForMemory::new())
}

fn new_users_repository(db: &toml::PostgreSQL) -> Arc<dyn users::UserRepository> {
    if db.enabled {
        return Arc::new(users::UserRepositoryForDB::new());
    }
    Arc::new(users::UserRepositoryForMemory::new())
}

#[allow(dead_code)]
pub struct Registry {
    pub conf: toml::Config,
    pub todos_repo: Arc<dyn todos::TodoRepository>,
    pub users_repo: Arc<dyn users::UserRepository>,
}

#[allow(dead_code)]
impl Registry {
    pub fn new(conf: toml::Config) -> Self {
        let db = conf.db.clone();
        Self {
            conf,
            todos_repo: new_todos_repository(&db),
            users_repo: new_users_repository(&db),
        }
    }

    fn create_admin_usecase(&self) -> Arc<dyn admin::AdminUsecase> {
        Arc::new(admin::AdminAction::new(
            self.todos_repo.clone(),
            self.users_repo.clone(),
        ))
    }

    fn create_app_usecase(&self) -> Arc<dyn app::AppUsecase> {
        Arc::new(app::AppAction::new(
            self.todos_repo.clone(),
            self.users_repo.clone(),
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
