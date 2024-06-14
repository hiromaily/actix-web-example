use crate::usecases::{admin, app};
use std::sync::Arc;

pub struct GlobalState {
    pub app_name: String,
}

pub struct AdminState {
    pub admin_usecase: Arc<dyn admin::AdminUsecase>,
}

pub struct AppState {
    pub app_usecase: Arc<dyn app::AppUsecase>,
}
