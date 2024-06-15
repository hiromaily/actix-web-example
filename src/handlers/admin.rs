use crate::state;
use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;
/*
 Admin
*/

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginBody {
    #[validate(length(min = 8, max = 50))]
    pub email: String,
    #[validate(length(min = 10, max = 20))]
    pub password: String,
}

// [post] /login
pub async fn admin_login(
    data: web::Data<state::GlobalState>,
    admin_data: web::Data<state::AdminState>,
    login_body: web::Json<LoginBody>,
) -> impl Responder {
    info!("admin_login received: app_name:{}", data.app_name);

    // Validate the login body
    if let Err(e) = login_body.validate() {
        return HttpResponse::BadRequest().json(json!({ "error": format!("{:?}", e) }));
    }

    // Extract the email and password
    let email = &login_body.email;
    let password = &login_body.password;

    // authentication
    let ret = admin_data.admin_usecase.admin_login(email, password);
    if !ret {
        return HttpResponse::BadRequest().json(json!({ "error": "invalid email or password" }));
    }

    // If login is successful, respond accordingly
    //format!("[admin_login] Hello {app_name}:{login_body.email}!")
    HttpResponse::Ok().json(json!({ "status": "success", "path": "admin_login", "email": email }))
}

// [get] /users
pub async fn get_user_list(
    _data: web::Data<state::GlobalState>,
    admin_data: web::Data<state::AdminState>,
) -> impl Responder {
    // let app_name = &data.app_name;
    // HttpResponse::Ok().body(format!("[get_user_list] Hello {app_name}!"))
    let user_list = admin_data.admin_usecase.get_user_list();
    HttpResponse::Ok().json(user_list)
}

// [post] /users
pub async fn add_user(data: web::Data<state::GlobalState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[add_user] Hello {app_name}!"))
}

// [get] "/users/{user_id}"
pub async fn get_user(data: web::Data<state::GlobalState>, path: web::Path<u32>) -> impl Responder {
    let user_id = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[get_user] Hello {app_name}:{user_id}!"))
}

// [post] "/users/{user_id}"
pub async fn update_user(
    data: web::Data<state::GlobalState>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[update_user] Hello {app_name}:{user_id}!"))
}

// [delete] "/users/{user_id}"
pub async fn delete_user(
    data: web::Data<state::GlobalState>,
    path: web::Path<u32>,
) -> impl Responder {
    let user_id = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[delete_user] Hello {app_name}:{user_id}!"))
}
