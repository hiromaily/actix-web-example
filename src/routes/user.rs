use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .route(web::get().to(top))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

async fn top() -> impl Responder {
    HttpResponse::Ok().body("{\"page\": \"user top\"}")
}
