use actix_web_module_upload::{
    core::service::Service,
    impls::{repositories::postgres::PostgresRepository, stores::local_fs::LocalFSStore},
    start,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("failed to  load .env file");
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not found");
    let pool = sqlx::PgPool::connect(&db_url).await.unwrap();
    let store_path = dotenv::var("STORE_PATH").expect("STORE_PATH not found");

    start::<_, _, i32, String>(Service::new(
        PostgresRepository::new(pool),
        LocalFSStore::new(store_path),
    ))
    .await
}
