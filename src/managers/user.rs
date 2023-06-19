use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::accessors::user::UserAccessor;
use crate::models::user::UserModel;
use axum::extract::Query;

pub struct UserManager;

impl UserManager {

    pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<CreateUserResponse>) {
        // TODO | Dependency injection? Reference via context?
        let mut user_accessor = UserAccessor::new();

        // TODO | Validate the incoming data

        // Save the resource to the data store
        let user = user_accessor.save(&payload.into());

        // Map the new resource to the response DTO
        let response = CreateUserResponse {
            id: user.id,
            username: user.username,
        };

        (StatusCode::CREATED, Json(response))
    }

    pub async fn list_users(Query(_payload): Query<ListUsersRequest>) -> (StatusCode, Json<ListUsersResponse>) {
        // TODO | Dependency innjection? Reference via context?
        let mut user_accessor = UserAccessor::new();

        // TODO | Validate the incoming data

        let users = user_accessor.list_users();

        let response = ListUsersResponse {
            data: users.iter()
                .map(|it| <UserModel as Into<ListUsersItem>>::into(it.clone()))
                .collect()
        };

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
