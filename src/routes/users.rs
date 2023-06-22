use std::sync::Arc;

use axum::{Router, Json};
use axum::extract::Query;
use axum::routing::post;
use axum::http::StatusCode;
use shaku_axum::Inject;

use crate::middlewares::logger::LoggerMiddleware;
use crate::resources::user::{UserResource, CreateUserRequest, CreateUserResponse, ListUsersRequest, ListUsersResponse};
use crate::modules::api::ApiModule;
use crate::state::api::ApiState;

pub struct UserRouter;

impl UserRouter {
    pub fn new() -> Router {
        // NOTE | The module and state need to be initialized in the same Router as the handlers!
        let module = Arc::new(ApiModule::builder().build());
        let state = ApiState { module };

        Router::new()
            .route("/users",
                   post(create_user)
                       .get(list_users))
            .layer(LoggerMiddleware { target: "UserResourceMiddleware" })
            .with_state(state)
            // NOTE | The state must be added at the end!
    }
}

/// NOTE | The Inject<> must be the first argument! It's a quirk of shaku_axum.
async fn create_user(user_resource: Inject<ApiModule, dyn UserResource>, Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<CreateUserResponse>) {
    user_resource.create_user(payload)
}

async fn list_users(user_resource: Inject<ApiModule, dyn UserResource>, Query(payload): Query<ListUsersRequest>) -> (StatusCode, Json<ListUsersResponse>) {
    user_resource.list_users(payload)
}