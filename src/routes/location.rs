use axum::{
    Router,
    Json,
    Extension,
    extract,
    routing::{get, post},
    response::IntoResponse,
};

use crate::{
    http::{
        ApiContext,
        error::LocationError,
    },
    models::location::{
        Location,
        NewLocation
    },
};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/locations",
            post(create)
            .get(read_all)
            .put(update)
        )
        .route("/api/v1/locations/:id",
            get(read)
            .delete(delete)
        )
}

async fn create(
    ctx: Extension<ApiContext>,
    extract::Json(req): extract::Json<NewLocation>,
) -> impl IntoResponse{
    let response: [Location; 0] = [];
    Location::create( &ctx.pool, &req.tid, req.lat, req.lon,
            req.tst)
        .await
        .map_err(|error| LocationError::Sqlx(error.to_string()))
        .map(|_| Json(response))
        //.on_db_error(|e| Error::unprocessable_entity([("error", e.to_string())]))
}

async fn read(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
    Location::read(&ctx.pool, id)
        .await
        .map_err(|error| LocationError::Sqlx(error.to_string()))
        .map(|location| Json(location))
}

async fn read_all(
    ctx: Extension<ApiContext>,
) -> impl IntoResponse{
    Location::read_all(&ctx.pool)
        .await
        .map_err(|error| LocationError::Sqlx(error.to_string()))
        .map(|locations| Json(locations))
}

async fn update(
    ctx: Extension<ApiContext>,
    Json(location): Json<Location>,
) -> impl IntoResponse{
    Location::update(&ctx.pool, location)
        .await
        .map_err(|error| LocationError::Sqlx(error.to_string()))
        .map(|location| Json(location))
}

async fn delete(
    ctx: Extension<ApiContext>,
    extract::Path(id): extract::Path<i64>,
) -> impl IntoResponse{
        Location::delete(&ctx.pool, id)
            .await
            .map_err(|error| LocationError::Sqlx(error.to_string()))
            .map(|location| Json(location))
}
