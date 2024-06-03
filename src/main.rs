use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod toml;

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
    let file_path = "./config/settings.toml";
    let config = toml::load_config(file_path);
    let config = match config {
        Ok(conf) => conf,
        Err(error) => {
            panic!("fail to load toml file [{}]: {:?}", file_path, error)
        }
    };
    // debug
    dbg!(&config);

    // connect server
    let host = config.server.host;
    let port = config.server.port;

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
