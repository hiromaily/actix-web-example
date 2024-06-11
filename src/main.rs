use actix_web::{middleware::Logger, web, App, HttpServer};
use core::time::Duration;
//use env_logger::Builder;
use log::info;
//use log::LevelFilter;

mod args;
mod registry;
mod repositories;
mod routes;
mod state;
mod toml;

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

    // registry
    // - better to move config
    let reg = registry::Registry::new(config);
    let server_data = reg.create_server_data();

    // connect to Server
    let host = reg.conf.server.host;
    let port = reg.conf.server.port;

    //println!("run server {}:{}", host, port);
    info!("run server {}:{}", host, port);

    // intentionally try various pattern to set routes
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(server_data.clone()))
            .service(routes::basis::get_hello)
            .service(routes::basis::get_hello_user)
            .service(routes::basis::post_echo)
            .service(routes::basis::post_echo_json)
            .service(web::scope("/api/v1").configure(routes::user::config))
            .service(web::scope("/api/v1").route("/info", web::get().to(routes::info::top))) // return 404
            .service(web::scope("/app").route("/index.html", web::get().to(routes::app::index)))
            .route("/health", web::get().to(routes::basis::health))
    })
    .keep_alive(Duration::from_secs(30))
    .bind((host, port))?
    .run()
    .await
}
