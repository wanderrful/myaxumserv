use std::sync::Arc;

use shaku::{Component, Interface};

use crate::accessors::user::UserAccessor;
use crate::models::user::UserModel;
use crate::resources::user::{CreateUserRequest, CreateUserResponse, ListUsersResponse, ListUsersItem};
use crate::utils::uuid::generate_uuid;

pub trait UserManager : Interface {
    fn create_user(&self, payload: CreateUserRequest) -> CreateUserResponse;
    fn list_users(&self) -> ListUsersResponse;
}

/// The manager is not responsible for validation, and is not coupled to the web framework.
#[derive(Component)]
#[shaku(interface = UserManager)]
pub(crate) struct UserManagerImpl {
    #[shaku(inject)]
    user_accessor: Arc<dyn UserAccessor>
}

impl UserManager for UserManagerImpl {
    fn create_user(&self, payload: CreateUserRequest) -> CreateUserResponse {
        // TODO | Dependency injection? Reference via context?
        // let mut user_accessor = UserAccessor::new();

        // Map the request to the UserModel
        let user_model = UserModel {
            id: generate_uuid(),
            username: payload.username.clone()
        };

        // Save the resource to the data store
        let user = self.user_accessor.save(&user_model);

        // Map the new resource to the response DTO
        CreateUserResponse {
            id: user.id,
            username: user.username,
        }
    }

    fn list_users(&self) -> ListUsersResponse {
        // TODO | Dependency innjection? Reference via context?
        let users = self.user_accessor.list_users();

        // Map resource data to response DTO
        ListUsersResponse {
            users: users.iter()
                .map(|it| ListUsersItem {
                    id: generate_uuid(),
                    username: it.username.clone()
                })
                .collect()
        }
    }
}