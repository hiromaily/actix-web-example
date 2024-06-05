use actix_web::{HttpResponse, Responder};

pub async fn top() -> impl Responder {
    println!("info top");
    HttpResponse::Ok().body("{\"page\": \"info top\"}")
}
