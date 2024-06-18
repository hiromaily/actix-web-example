use crate::entities::{todos, users};
use crate::state;
use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde_json::json;
use validator::Validate;

/*
 App
*/

// [post] /login
pub async fn app_login(
    data: web::Data<state::GlobalState>,
    app_data: web::Data<state::AppState>,
    body: web::Json<users::LoginBody>,
) -> impl Responder {
    info!("app_login received: app_name:{}", data.app_name);

    // validation
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(json!({ "error": format!("{:?}", e) }));
    }

    // Extract the email and password
    let email = &body.email;
    let password = &body.password;

    // authentication usecase
    match app_data.auth_usecase.login(email, password).await {
        Ok(Some(user)) => {
            // TODO: return access key
            match app_data
                .auth_usecase
                .generate_token(user.id, user.email.as_str())
            {
                Ok(token) => HttpResponse::Ok().json(json!({
                    "status": "success",
                    "message": "Login successful",
                    "token": token
                })),
                Err(e) => HttpResponse::InternalServerError().json(json!({
                    "status": "error",
                    "message": e.to_string()
                })),
            }
            //HttpResponse::Ok().json(json!({ "status": "success", "message": "Login successful" }))
        }
        Ok(None) => HttpResponse::Unauthorized()
            .json(json!({ "status": "error", "message": "user is not found" })),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({ "status": "error", "message": e.to_string() })),
    }
}

// [get] /users/{user_id}/todos
pub async fn get_user_todo_list(
    _data: web::Data<state::GlobalState>,
    app_data: web::Data<state::AppState>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();

    // usecase
    match app_data.app_usecase.get_user_todo_list(user_id).await {
        Ok(todo_list) => HttpResponse::Ok().json(todo_list),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({ "status": "error", "message": e.to_string() })),
    }
}

// [post] /users/{user_id}/todos
pub async fn add_user_todo(
    _data: web::Data<state::GlobalState>,
    app_data: web::Data<state::AppState>,
    path: web::Path<i32>,
    body: web::Json<todos::TodoBody>,
) -> impl Responder {
    let user_id = path.into_inner();

    // validation
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(json!({ "error": format!("{:?}", e) }));
    }
    let todo_body: todos::TodoBody = body.into_inner();

    // usecase
    match app_data.app_usecase.add_user_todo(user_id, todo_body).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
    }
}

// [get] "/users/{user_id}/todos/{todo_id}"
pub async fn get_user_todo(
    _data: web::Data<state::GlobalState>,
    app_data: web::Data<state::AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (user_id, todo_id) = path.into_inner();

    // usecase
    let res = app_data.app_usecase.get_user_todo(user_id, todo_id).await;
    // response
    match res {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Todo not found",
            "message": format!("Todo with ID {} not found", todo_id)
        })),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({ "status": "error", "message": e.to_string() })),
    }
}

// [post] "/users/{user_id}/todos/{todo_id}"
pub async fn update_user_todo(
    _data: web::Data<state::GlobalState>,
    app_data: web::Data<state::AppState>,
    path: web::Path<(i32, i32)>,
    body: web::Json<todos::TodoUpdateBody>,
) -> impl Responder {
    let (user_id, todo_id) = path.into_inner();

    // validate
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(json!({ "error": format!("{:?}", e) }));
    }
    let todo_body: todos::TodoUpdateBody = body.into_inner();

    // usecase
    match app_data
        .app_usecase
        .update_user_todo(user_id, todo_id, todo_body)
        .await
    {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().json(json!({
            "error": "Todo not found",
            "message": format!("Todo with ID {} not found", todo_id)
        })),
        Err(e) => {
            HttpResponse::BadRequest().json(json!({ "status": "error", "message": e.to_string() }))
        }
    }
}

// [delete] // [post] "/users/{user_id}/todos/{todo_id}"
pub async fn delete_user_todo(
    _data: web::Data<state::GlobalState>,
    app_data: web::Data<state::AppState>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (user_id, todo_id) = path.into_inner();
    match app_data
        .app_usecase
        .delete_user_todo(user_id, todo_id)
        .await
    {
        Ok(0) => HttpResponse::NotFound().json(json!({
            "error": "Todo not found",
            "message": format!("Todo with ID {} not found", todo_id)
        })),
        Ok(_) => {
            HttpResponse::Ok().json(json!({ "status": "success", "message": "Delete successful" }))
        }
        Err(e) => {
            HttpResponse::NotFound().json(json!({ "status": "error", "message": e.to_string() }))
        }
    }
}

// pub async fn index(data: web::Data<crate::state::AppState>) -> impl Responder {
//     let app_name = &data.app_name;
//     format!("Hello {app_name}!")
// }
