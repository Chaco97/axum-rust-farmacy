use axum::{extract::State, Json};
use crate::{state::app_state::AppState, models::proveedor::Proveedor};

pub async fn get_proveedores(
    State(state): State<AppState>,
) -> Json<Vec<Proveedor>> {

    let proveedores = sqlx::query_as::<_, Proveedor>(
        "SELECT id, nombre, telefono, direccion, email FROM proveedor"
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(proveedores)
}

