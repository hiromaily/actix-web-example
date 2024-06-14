use actix_web::{web, HttpResponse, Responder};

/*
 App
*/

// [post] /login
pub async fn app_login(data: web::Data<crate::state::AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[app_login] Hello {app_name}!"))
}

// [get] /users/{user_id}/todos
pub async fn get_user_todo_list(
    data: web::Data<crate::state::AppState>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[get_user_todo_list] Hello {app_name}:{user_id}!"))
}

// [post] /users/{user_id}/todos
pub async fn add_user_todo(
    data: web::Data<crate::state::AppState>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[add_user_todo] Hello {app_name}:{user_id}!"))
}

// [get] "/users/{user_id}/todos/{todo_id}"
pub async fn get_user_todo(
    data: web::Data<crate::state::AppState>,
    path: web::Path<(u32, u32)>,
) -> impl Responder {
    let (user_id, todo_id) = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!(
        "[get_user_todo] Hello {app_name}:{user_id}:{todo_id}!"
    ))
}

// [post] "/users/{user_id}/todos/{todo_id}"
pub async fn update_user_todo(
    data: web::Data<crate::state::AppState>,
    path: web::Path<(u32, u32)>,
) -> impl Responder {
    let (user_id, todo_id) = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!(
        "[update_user_todo] Hello {app_name}:{user_id}:{todo_id}!"
    ))
}

// [delete] // [post] "/users/{user_id}/todos/{todo_id}"
pub async fn delete_user_todo(
    data: web::Data<crate::state::AppState>,
    path: web::Path<(u32, u32)>,
) -> impl Responder {
    let (user_id, todo_id) = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!(
        "[delete_user_todo] Hello {app_name}:{user_id}:{todo_id}!"
    ))
}

// pub async fn index(data: web::Data<crate::state::AppState>) -> impl Responder {
//     let app_name = &data.app_name;
//     format!("Hello {app_name}!")
// }
