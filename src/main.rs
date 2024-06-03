use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::fs;
use toml;

// toml definition
#[derive(Debug, Deserialize)]
struct Config {
    #[allow(dead_code)]
    project_name: String,
    #[allow(dead_code)]
    logger: Logger,
    #[allow(dead_code)]
    pg: PostgreSQL,
}

#[derive(Debug, Deserialize)]
struct Logger {
    #[allow(dead_code)]
    service: String,
    #[allow(dead_code)]
    level: String,
}

#[derive(Debug, Deserialize)]
struct PostgreSQL {
    #[allow(dead_code)]
    host: String,
    #[allow(dead_code)]
    dbname: String,
    #[allow(dead_code)]
    user: String,
    #[allow(dead_code)]
    password: String,
}

// router
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load toml
    let toml_str =
        fs::read_to_string("./config/settings.toml").expect("Failed to read settings.toml file");
    let config: Config = toml::from_str(&toml_str).expect("Failed to deserialize settings.toml");
    println!("{:#?}", config);

    // connect server
    let host = "127.0.0.1";
    let port = 8080;

    println!("run server {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((host, port))?
    .run()
    .await
}
