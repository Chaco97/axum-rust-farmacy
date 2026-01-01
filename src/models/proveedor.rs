use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Proveedor {
    pub id: i32,
    pub nombre: String,
    pub telefono: Option<String>,
    pub direccion: Option<String>,
    pub email: Option<String>
}
