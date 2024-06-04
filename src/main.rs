use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
//use env_logger::Builder;
use log::info;
//use log::LevelFilter;

mod args;
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
    // initialize log
    env_logger::init();
    // Builder::from_default_env()
    //     .filter_level(LevelFilter::Info)
    //     .init();

    // command line arguments
    let arg = args::get_args();
    dbg!(&arg);

    // load toml
    let file_path = arg.conf;
    let config = toml::load_config(file_path.as_str());
    let config = match config {
        Ok(conf) => conf,
        Err(error) => {
            panic!("fail to load toml file [{}]: {:?}", file_path, error)
        }
    };
    dbg!(&config);

    // connect server
    let host = config.server.host;
    let port = config.server.port;

    //println!("run server {}:{}", host, port);
    info!("run server {}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((host, port))?
    .run()
    .await
}
