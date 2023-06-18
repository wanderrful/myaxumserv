use axum::Router;
use crate::routes::users::UserRouter;

pub struct ApiRouter;

impl ApiRouter {

    pub fn new() -> Router {
        let v1_routes = Router::new()
            .merge(UserRouter::new());

        Router::new()
            .nest("/v1", v1_routes)
    }

}
