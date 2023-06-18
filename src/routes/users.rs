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
            // `POST /users` goes to `create_user`
            .route("/users", post(create_user))
    }
}

async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<CreateUserResponse>) {
    // TODO | Dependency injection? Reference via context?
    let user_manager = UserManager { };

    let user = user_manager.create(&payload);

    // Map the new resource to the response DTO
    let response = CreateUserResponse {
        id: user.id,
        username: user.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(response))
}

/// Request DTO for the CreateUser operation
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

/// Response DTO for the CreateUser operation
#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: u64,
    pub username: String,
}
