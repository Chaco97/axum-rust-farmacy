use axum::{Router, routing::get};
use crate::handlers::proveedor_handler::get_proveedores;
use crate::state::app_state::AppState;

pub fn proveedor_routes(state: AppState) -> Router {
    Router::new()
        .route("/proveedores", get(get_proveedores))
        .with_state(state)
}
