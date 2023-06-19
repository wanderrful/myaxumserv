use axum::Router;
use axum::routing::post;

use crate::resources::user::UserResource;

pub struct UserRouter;

impl UserRouter {

    pub fn new() -> Router {
        Router::new()
            .route("/users", post(UserResource::create_user).get(UserResource::list_users))
    }

}
