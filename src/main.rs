mod handlers;
use actix_web::{App, HttpServer};
use upload_service::{
    core::service::Service, repositories::postgres::PostgresRepository,
    stores::local_fs::LocalFSStore,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("failed to  load .env file");
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not found");
    let pool = sqlx::PgPool::connect(&db_url).await.unwrap();
    let store_path = dotenv::var("STORE_PATH").expect("STORE_PATH not found");
    // let service = Service::new(PostgresRepository::new(pool), LocalFSStore::new(store_path));

    HttpServer::new(move || App::new())
        .bind("0.0.0.0:8001")?
        .run()
        .await
}
