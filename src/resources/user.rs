use axum::extract::Query;
use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::managers::user::UserManager;


pub struct UserResource;

impl UserResource {

    pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<CreateUserResponse>) {
        // TODO | Dependency injection? Reference via context?
        let user_manager = UserManager { };

        // TODO | Validate the incoming request

        let response = user_manager.create_user(payload);

        (StatusCode::CREATED, Json(response))
    }

    pub async fn list_users(Query(_payload): Query<ListUsersRequest>) -> (StatusCode, Json<ListUsersResponse>) {
        // TODO | Dependency injection? Reference via context?
        let user_manager = UserManager { };

        // TODO | Validate the incoming request

        let response = user_manager.list_users();

        (StatusCode::OK, Json(response))
    }

}

/// Request DTO for the CreateUser operation
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

/// Response DTO for the CreateUser operation
#[derive(Serialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub username: String,
}

/// Request DTO (query params) for the ListUsers operation
#[derive(Deserialize)]
pub struct ListUsersRequest {
    pub limit: Option<usize>,
    pub username: Option<String>
}

/// Response DTO for the ListUsers operation
#[derive(Serialize)]
pub struct ListUsersResponse {
    pub data: Vec<ListUsersItem>
}

/// Helper struct for the ListUsers operation response DTO
#[derive(Serialize)]
pub struct ListUsersItem {
    pub id: String,
    pub username: String
}
