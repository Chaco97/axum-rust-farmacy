pub use axum_macros::{debug_handler};

use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json, 
    
};

use crate::{models::cliente::{Cliente, NewCliente}, state::app_state::AppState};


pub async fn get_clientes(
    State(state): State<AppState>,
) -> Json<Vec<Cliente>> {

    let clientes = sqlx::query_as::<_, Cliente>(
        "SELECT id, nombre, cedula, telefono FROM cliente"
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(clientes)

}

 pub async fn get_cliente_by_id(
    Path(id): Path<i32>,
    State(state): State<AppState>,
) -> Result<Json<Cliente>, StatusCode> {

    let cliente = sqlx::query_as::<_, Cliente>(
        "SELECT id, nombre, cedula, telefono FROM cliente WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        eprintln!("DB error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match cliente {
        Some(c) => Ok(Json(c)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

#[debug_handler]
pub async fn create_cliente(
    State(state): State<AppState>,
    Json(payload): Json<NewCliente>,
) -> Result<(StatusCode, Json<Cliente>), StatusCode> {


    let cliente = sqlx::query_as!(
        Cliente,
        "INSERT INTO cliente (nombre, cedula, telefono) VALUES ($1, $2, $3) 
         RETURNING id, nombre, cedula, telefono",
        payload.nombre,
        payload.cedula,
        payload.telefono
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        eprintln!("DB error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(cliente)))
}


