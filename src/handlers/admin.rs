use crate::entities::users;
use crate::state;
use actix_web::{web, HttpResponse, Responder};
use log::info;
use serde_json::json;
use validator::Validate;

/*
 Admin
*/

// [post] /login
pub async fn admin_login(
    data: web::Data<state::GlobalState>,
    admin_data: web::Data<state::AdminState>,
    body: web::Json<users::LoginBody>,
) -> impl Responder {
    info!("admin_login received: app_name:{}", data.app_name);

    // validation
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(json!({ "error": format!("{:?}", e) }));
    }

    // Extract the email and password
    let email = &body.email;
    let password = &body.password;

    // authentication usecase
    match admin_data.admin_usecase.admin_login(email, password).await {
        Ok(_) => {
            HttpResponse::Ok().json(json!({ "status": "success", "message": "Login successful" }))
        }
        Err(e) => HttpResponse::Unauthorized()
            .json(json!({ "status": "error", "message": e.to_string() })),
    }
}

// [get] /users
pub async fn get_user_list(
    _data: web::Data<state::GlobalState>,
    admin_data: web::Data<state::AdminState>,
) -> impl Responder {
    // let app_name = &data.app_name;
    // HttpResponse::Ok().body(format!("[get_user_list] Hello {app_name}!"))

    // usecase
    let user_list = admin_data.admin_usecase.get_user_list();
    // response
    HttpResponse::Ok().json(user_list)
}

// [post] /users
pub async fn add_user(
    _data: web::Data<state::GlobalState>,
    admin_data: web::Data<state::AdminState>,
    body: web::Json<users::UserBody>,
) -> impl Responder {
    // let app_name = &data.app_name;
    // HttpResponse::Ok().body(format!("[add_user] Hello {app_name}!"))

    // validation
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(json!({ "error": format!("{:?}", e) }));
    }
    let user_body: users::UserBody = body.into_inner();

    // usecase
    match admin_data.admin_usecase.add_user(user_body) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
    }
}

// [get] "/users/{user_id}"
pub async fn get_user(
    _data: web::Data<state::GlobalState>,
    admin_data: web::Data<state::AdminState>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    // let app_name = &data.app_name;
    // HttpResponse::Ok().body(format!("[get_user] Hello {app_name}:{user_id}!"))

    // usecase
    let res = admin_data.admin_usecase.get_user(user_id);
    // response
    if let Some(user) = res {
        HttpResponse::Ok().json(user)
    } else {
        // return 404
        HttpResponse::NotFound().json(json!({
            "error": "User not found",
            "message": format!("User with ID {} not found", user_id)
        }))
    }
    // match res {
    //     Some(user) => {
    //         HttpResponse::Ok().json(user)
    //     },
    //     None => {
    //         HttpResponse::NotFound().json(json!({
    //             "error": "User not found",
    //             "message": format!("User with ID {} not found", user_id)
    //         }))
    //     }
    // }
}

// [post] "/users/{user_id}"
pub async fn update_user(
    _data: web::Data<state::GlobalState>,
    admin_data: web::Data<state::AdminState>,
    path: web::Path<i32>,
    body: web::Json<users::UserUpdateBody>,
) -> impl Responder {
    let user_id = path.into_inner();
    // let app_name = &data.app_name;
    // HttpResponse::Ok().body(format!("[update_user] Hello {app_name}:{user_id}!"))

    // validate
    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(json!({ "error": format!("{:?}", e) }));
    }
    let user_body: users::UserUpdateBody = body.into_inner();

    // usecase
    match admin_data.admin_usecase.update_user(user_id, user_body) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::BadRequest().json(json!({ "error": e.to_string() })),
    }
}

// [delete] "/users/{user_id}"
pub async fn delete_user(
    _data: web::Data<state::GlobalState>,
    admin_data: web::Data<state::AdminState>,
    path: web::Path<i32>,
) -> impl Responder {
    let user_id = path.into_inner();
    // let app_name = &data.app_name;
    // HttpResponse::Ok().body(format!("[delete_user] Hello {app_name}:{user_id}!"))
    match admin_data.admin_usecase.delete_user(user_id) {
        Ok(_) => {
            HttpResponse::Ok().json(json!({ "status": "success", "message": "Delete successful" }))
        }
        Err(e) => HttpResponse::Unauthorized()
            .json(json!({ "status": "error", "message": e.to_string() })),
    }
}
