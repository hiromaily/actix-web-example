use crate::dbs::conn;
use crate::repositories::{todos, users};
use crate::state;
use crate::toml;
use crate::usecases::{admin, app, auth};
use crate::{hash, jwt};
use sea_orm::DbErr;
use std::sync::Arc;

// error would occur if TodoRepository has clone trait as supertrait
// fn new_todos_repository(&self) -> Box<dyn todo_repository::TodoRepository> {
//     if self.conf.db.enabled {
//         return Box::new(todo_repository::TodoRepositoryForDB::new());
//     } else {
//         return Box::new(todo_repository::TodoRepositoryForMemory::new());
//     }
// }
async fn new_todos_repository(
    db: &toml::PostgreSQL,
) -> Result<Arc<dyn todos::TodoRepository>, DbErr> {
    if db.enabled {
        let connected = conn::get_conn(&db.user, &db.password, &db.host, &db.dbname).await?;
        return Ok(Arc::new(todos::TodoRepositoryForDB::new(connected)));
    }
    Ok(Arc::new(todos::TodoRepositoryForMemory::new()))
}

async fn new_users_repository(
    db: &toml::PostgreSQL,
) -> Result<Arc<dyn users::UserRepository>, DbErr> {
    if db.enabled {
        let connected = conn::get_conn(&db.user, &db.password, &db.host, &db.dbname).await?;
        return Ok(Arc::new(users::UserRepositoryForDB::new(connected)));
    }
    Ok(Arc::new(users::UserRepositoryForMemory::new()))
}

// Must not use for production
//const SALT: &str = "Salt should be passed in a more secure way";

fn new_hash() -> Arc<dyn hash::Hash> {
    // for now, only 1 implementation
    Arc::new(hash::HashPbkdf2::new())
}

fn new_jwt(cjwt: &toml::JWT) -> Arc<dyn jwt::JWT> {
    match cjwt.kind {
        toml::JWTKind::JWTSimple => Arc::new(jwt::SimpleJWT::new(cjwt.duration_min)),
        toml::JWTKind::JsonWebToken => Arc::new(jwt::JsonWebToken::new(cjwt.duration_min * 60)),
        toml::JWTKind::None => Arc::new(jwt::DummyJWT::new()),
    }
}

#[allow(dead_code)]
pub struct Registry {
    pub conf: toml::Config,
    pub todos_repo: Arc<dyn todos::TodoRepository>,
    pub users_repo: Arc<dyn users::UserRepository>,
    pub hash: Arc<dyn hash::Hash>,
    pub jwt: Arc<dyn jwt::JWT>,
}

#[allow(dead_code)]
impl Registry {
    pub async fn new(conf: toml::Config) -> Result<Self, DbErr> {
        //let db = conf.db.clone();
        let todos_repo = new_todos_repository(&conf.db).await?;
        let users_repo = new_users_repository(&conf.db).await?;
        let hash = new_hash();
        let jwt = new_jwt(&conf.jwt);

        Ok(Self {
            conf,
            todos_repo,
            users_repo,
            hash,
            jwt,
        })
    }

    // is_admin: true => AuthAdminAction, false => AuthAppAction
    fn create_auth_usecase(&self) -> Arc<dyn auth::AuthUsecase> {
        Arc::new(auth::AuthAction::new(
            self.users_repo.clone(),
            self.hash.clone(),
            self.jwt.clone(),
        ))
    }

    fn create_admin_usecase(&self) -> Arc<dyn admin::AdminUsecase> {
        // TODO: is there any way to avoid clone?
        Arc::new(admin::AdminAction::new(
            self.todos_repo.clone(),
            self.users_repo.clone(),
            self.hash.clone(),
        ))
    }

    fn create_app_usecase(&self) -> Arc<dyn app::AppUsecase> {
        Arc::new(app::AppAction::new(
            self.todos_repo.clone(), // TODO: is there any way to avoid clone?
            self.users_repo.clone(), // TODO: is there any way to avoid clone?
        ))
    }

    pub fn create_global_state(&self) -> state::GlobalState {
        state::GlobalState {
            app_name: self.conf.app_name.clone(),
        }
    }

    pub fn create_auth_state(&self) -> state::AuthState {
        state::AuthState {
            auth_usecase: self.create_auth_usecase(),
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
