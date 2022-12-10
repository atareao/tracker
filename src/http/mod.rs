use std::{sync::Arc, net::{SocketAddr, Ipv4Addr}};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use axum::{
    Extension,
    Router
};
use sqlx::SqlitePool;
use crate::{
    config::Configuration,
    routes::location,
    http::error::LocationError,
};

pub mod error;


pub async fn serve(config: Configuration, pool: SqlitePool) -> Result<(), LocationError>{
    let app = api_router().layer(
        ServiceBuilder::new()
            .layer(Extension(ApiContext {
                config: Arc::new(config.clone()),
                pool,
            }))
            // Enables logging. Use `RUST_LOG=tower_http=debug`
            .layer(TraceLayer::new_for_http())
    );

    axum::Server::bind(
        &SocketAddr::new(std::net::IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), config.get_port()))
        .serve(app.into_make_service())
        .await
        .map_err(|err| error::LocationError::ReadError)
    
}

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Configuration>,
    pub pool: SqlitePool,
}

fn api_router() -> Router{
    location::router()
}
