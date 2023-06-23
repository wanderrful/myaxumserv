use std::sync::Arc;

use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};

use crate::managers::user::UserManager;

/// The resource is the entry point, from which we exit the web framework and enter
///     the application's internal logic.
pub trait UserResource : Interface {
    fn create_user(&self, payload: CreateUserRequest) -> (StatusCode, Json<CreateUserResponse>);
    fn list_users(&self, payload: ListUsersRequest) -> (StatusCode, Json<ListUsersResponse>);
}

#[derive(Component)]
#[shaku(interface = UserResource)]
pub(crate) struct UserResourceImpl {
    #[shaku(inject)]
    user_manager: Arc<dyn UserManager>
}

impl UserResource for UserResourceImpl {

    fn create_user(&self, payload: CreateUserRequest) -> (StatusCode, Json<CreateUserResponse>) {
        // TODO | Validate the incoming request

        let response = self.user_manager.create_user(payload);

        // TODO | Error case? Maybe we should use axum's `Either` concept here
        (StatusCode::CREATED, Json(response))
    }

    fn list_users(&self, _payload: ListUsersRequest) -> (StatusCode, Json<ListUsersResponse>) {
        // TODO | Validate the incoming request

        let response = self.user_manager.list_users();

        // TODO | Error case? Maybe we should use axum's `Either` concept here
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
    pub users: Vec<ListUsersItem>
}

/// Helper struct for the ListUsers operation response DTO
#[derive(Serialize)]
pub struct ListUsersItem {
    pub id: String,
    pub username: String
}
