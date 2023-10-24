mod handlers;
use actix_multipart::Field;
use actix_web::{
    middleware::Logger,
    web::{post, resource, Data},
    App, HttpServer,
};
use env_logger::{init_from_env, Env};
use nb_from_env::{FromEnv, FromEnvDerive};
use upload_service::{
    core::service::Service, repositories::postgres::PostgresRepository,
    stores::local_fs::LocalFSStore,
};

#[derive(Debug, FromEnvDerive)]
struct Config {
    postgres_uri: String,
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
    let pool = sqlx::PgPool::connect(&config.postgres_uri)
        .await
        .expect("failed to connect to postgres");
    init_from_env(Env::default().default_filter_or(&config.log_level));
    let service = Data::new(Service::<PostgresRepository, LocalFSStore, String>::new(
        PostgresRepository::new(pool),
        LocalFSStore::new(&config.store_path),
    ));

    HttpServer::new(move || {
        let logger = Logger::new(&config.log_format);
        App::new()
            .wrap(logger)
            .app_data(service.clone())
            .service(resource("files").post(handlers::upload::<PostgresRepository, LocalFSStore>))
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}
