use axum::Router;
use axum::routing::post;

use crate::managers::user::UserManager;

pub struct UserRouter;

impl UserRouter {

    pub fn new() -> Router {
        Router::new()
            .route("/users", post(UserManager::create_user).get(UserManager::list_users))
    }

}
