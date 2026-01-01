use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn connect_db() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL")
        .expect("DATABASE_URL no definido"))
        .await
        .expect("Error conectando a la BD")
}
