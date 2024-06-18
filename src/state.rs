use crate::usecases::{admin, app, auth};
use std::sync::Arc;

pub struct GlobalState {
    pub app_name: String,
}

pub struct AdminState {
    pub auth_usecase: Arc<dyn auth::AuthUsecase>,
    pub admin_usecase: Arc<dyn admin::AdminUsecase>,
}

pub struct AppState {
    pub auth_usecase: Arc<dyn auth::AuthUsecase>,
    pub app_usecase: Arc<dyn app::AppUsecase>,
}
