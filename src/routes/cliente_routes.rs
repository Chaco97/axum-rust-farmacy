use axum::{Router, routing::get};
use crate::handlers::cliente_handler::{get_cliente_by_id, get_clientes};
use crate::state::app_state::AppState;

pub fn cliente_routes(state: AppState) -> Router {
    Router::new()
        .route("/clientes", get(get_clientes))
        .route("/clientes/:id", get(get_cliente_by_id))
        .with_state(state)
       
}
