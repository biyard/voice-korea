use by_axum::axum::{middleware, Router};

use crate::middleware::auth::admin_authorization_middleware;

mod survey;

pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
    Router::new()
        .nest("/survey", survey::router(db))
        .layer(middleware::from_fn(admin_authorization_middleware))
}
