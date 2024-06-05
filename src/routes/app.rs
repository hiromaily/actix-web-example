use actix_web::{web, Responder};

pub async fn index(data: web::Data<crate::state::AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}!")
}
