use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use core::time::Duration;
use log::info;
//use env_logger::Builder;
//use log::LevelFilter;

// local
use api_server::args;
use api_server::handlers;
use api_server::registry;
use api_server::routes;
use api_server::toml;

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
    let web_data = web::Data::new(server_data);

    // In this timing, error would occur if TodoRepository has clone trait as supertrait
    // let client_db: web::Data<Arc<dyn TodoRepository>> =
    //     web::Data::new(Arc::new(TodoRepositoryForMemory::new()));

    // connect to Server
    let host = reg.conf.server.host;
    let port = reg.conf.server.port;

    info!("run server {}:{}", host, port);

    // intentionally try various pattern to set routes
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web_data.clone())
            .service(
                web::scope("api/v1")
                    .route("/health", web::get().to(handlers::basis::health))
                    .service(
                        web::scope("/admin")
                            .configure(routes::api_admin_login_config)
                            .configure(routes::api_admin_users_config)
                            .configure(routes::api_admin_users_id_config),
                    )
                    .service(
                        web::scope("/app")
                            .configure(routes::api_app_login_config)
                            .configure(routes::api_app_users_todo_config)
                            .configure(routes::api_app_users_todo_id_config),
                    ),
            )
        // .service(routes::basis::get_hello)
        // .service(routes::basis::get_hello_user)
        // .service(routes::basis::post_echo)
        // .service(routes::basis::post_echo_json)
        // .service(web::scope("/api/v1").configure(routes::user::config))
        // .service(web::scope("/api/v1").route("/info", web::get().to(routes::info::top))) // return 404
        // .service(web::scope("/app").route("/index.html", web::get().to(routes::app::index)))
        // .route("/health", web::get().to(routes::basis::health))
    })
    .keep_alive(Duration::from_secs(30))
    .bind((host, port))?
    .run()
    .await
}
