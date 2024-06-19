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

    // registry and get each states
    let reg = registry::Registry::new(config).await.unwrap(); // may panic

    //let global_data = web::Data::new(reg.create_global_state());
    let auth_data = web::Data::new(reg.create_auth_state());
    let admin_data = web::Data::new(reg.create_admin_state());
    let app_data = web::Data::new(reg.create_app_state());

    // In this timing, error would occur if TodoRepository has clone trait as supertrait
    // let client_db: web::Data<Arc<dyn TodoRepository>> =
    //     web::Data::new(Arc::new(TodoRepositoryForMemory::new()));

    // connect to Server
    let host = reg.conf.server.host;
    let port = reg.conf.server.port;

    info!("run server {}:{}", host, port);

    // [WIP] experimental code
    //let my_app = handlers::basis::MyApp::new(String::from("foobar"));

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
            //.app_data(global_data.clone()) // global state
            .app_data(auth_data.clone()) // global state
            .service(
                web::scope("api/v1")
                    //.route("/example", web::get().to(move || my_app.greet()))
                    .route("/health", web::get().to(handlers::basis::health))
                    .service(
                        web::scope("/admin")
                            .app_data(admin_data.clone()) // admin state // maybe divide it into each configuration level
                            .configure(routes::api_admin_login_config)
                            .configure(routes::api_admin_users_config)
                            .configure(routes::api_admin_users_id_config),
                    )
                    .service(
                        web::scope("/app")
                            .app_data(app_data.clone()) // app state // maybe divide it into each configuration level
                            .configure(routes::api_app_login_config)
                            .configure(routes::api_app_users_todo_config)
                            .configure(routes::api_app_users_todo_id_config),
                    ),
            )
    })
    .keep_alive(Duration::from_secs(30))
    .bind((host, port))?
    .run()
    .await
}
