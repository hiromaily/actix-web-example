use actix_web::{get, post, web, HttpResponse, Responder, Result};
use log::info;
use serde::Deserialize;

pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/hello")]
pub async fn get_hello(data: web::Data<crate::state::AppState>) -> impl Responder {
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("Hello {app_name}!"))
}

// [Path: https://actix.rs/docs/extractors#path]
// it doesn't matter order of arguments
#[get("/hello/{user_id}/{user_name}")]
pub async fn get_hello_user(
    data: web::Data<crate::state::AppState>,
    path: web::Path<(u32, String)>,
) -> impl Responder {
    let (user_id, user_name) = path.into_inner();
    let app_name = &data.app_name;
    HttpResponse::Ok().body(format!("[{app_name}] Hello {user_id}:{user_name}!"))
}

#[post("/echo")]
pub async fn post_echo(
    data: web::Data<crate::state::AppState>,
    req_body: String,
) -> impl Responder {
    let app_name = &data.app_name;
    info!("app_name: {}", app_name);

    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[post("/echojson")]
pub async fn post_echo_json(
    data: web::Data<crate::state::AppState>,
    user: web::Json<User>,
) -> Result<String> {
    let app_name = &data.app_name;
    info!("app_name: {}", app_name);

    Ok(format!("Welcome {}({})!", user.name, user.age))
}
