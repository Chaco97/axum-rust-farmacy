use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Cliente {
    pub id: i32,
    pub nombre: String,
    pub cedula: Option<String>,
    pub telefono: Option<String>,
}
