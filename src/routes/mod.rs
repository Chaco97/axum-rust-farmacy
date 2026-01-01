use axum::Router;
use crate::{config::database::connect_db,  state::app_state::AppState};

mod cliente_routes;
mod proveedor_routers;

pub async fn create_router() -> Router {
    let db = connect_db().await;
    let state = AppState { db };

    Router::new()
        .merge(cliente_routes::cliente_routes(state.clone()))
        .merge(proveedor_routers::proveedor_routes(state.clone()))
}
