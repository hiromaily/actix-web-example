use actix_web::{web, HttpResponse, Responder};

// [post] /login
pub async fn admin_login(data: web::Data<crate::state::AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[admin_login] Hello {app_name}!"))
}

// [get] /users
pub async fn get_user_list(data: web::Data<crate::state::AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[get_user_list] Hello {app_name}!"))
}

// [post] /users
pub async fn add_user(data: web::Data<crate::state::AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[add_user] Hello {app_name}!"))
}

// [get] "/users/{user_id}"
pub async fn get_user(
    data: web::Data<crate::state::AppState>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[get_user] Hello {app_name}:{user_id}!"))
}

// [post] "/users/{user_id}"
pub async fn update_user(
    data: web::Data<crate::state::AppState>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[update_user] Hello {app_name}:{user_id}!"))
}

// [delete] "/users/{user_id}"
pub async fn delete_user(
    data: web::Data<crate::state::AppState>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[delete_user] Hello {app_name}:{user_id}!"))
}
