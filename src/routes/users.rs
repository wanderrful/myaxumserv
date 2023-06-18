use axum::{Router, Json};
use axum::http::StatusCode;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};

use crate::models::user::UserModel;
use crate::managers::user::UserManager;

pub struct UserRouter;

impl UserRouter {

    pub fn new() -> Router {
        Router::new()
            .route("/users", post(UserManager::create_user).get(UserManager::list_users))
    }

}
