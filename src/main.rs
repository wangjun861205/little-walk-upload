mod handlers;
use actix_web::{
    middleware::Logger,
    web::{get, post, scope, Data},
    App, HttpServer,
};
use env_logger::{init_from_env, Env};
use mongodb::Client;
use nb_from_env::{FromEnv, FromEnvDerive};
use upload_service::{
    core::service::Service, repositories::mongo::Mongo, stores::local_fs::LocalFSStore,
};

#[derive(Debug, FromEnvDerive)]
struct Config {
    server_address: String,
    mongo_uri: String,
    mongo_database: String,
    store_path: String,
    #[env_default("info")]
    log_level: String,
    #[env_default("%t %s %r %D")]
    log_format: String,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = Config::from_env();
    let db = Client::with_uri_str(&config.mongo_uri)
        .await
        .expect("failed to connect client")
        .database(&config.mongo_database);
    init_from_env(Env::default().default_filter_or(&config.log_level));
    let service = Data::new(Service::<Mongo, LocalFSStore>::new(
        Mongo::new(db),
        LocalFSStore::new(&config.store_path),
    ));

    HttpServer::new(move || {
        let logger = Logger::new(&config.log_format);
        App::new().wrap(logger).app_data(service.clone()).service(
            scope("/apis").service(
                scope("/uploads")
                    .route("/{id}", get().to(handlers::get::<Mongo, LocalFSStore>))
                    .route("", post().to(handlers::upload::<Mongo, LocalFSStore>)),
            ),
        )
    })
    .bind(&config.server_address)?
    .run()
    .await
}
