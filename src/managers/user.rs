use std::sync::Arc;

use shaku::{Component, Interface};

use crate::accessors::user::UserAccessor;
use crate::models::user::UserModel;
use crate::resources::user::{CreateUserRequest, CreateUserResponse, ListUsersResponse, ListUsersItem};
use crate::utils::uuid::generate_uuid;

/// The manager is the primary entry point of our actual application's logic,
///     without any dependency on the web framework itself.
pub trait UserManager : Interface {
    fn create_user(&self, payload: CreateUserRequest) -> CreateUserResponse;
    fn list_users(&self) -> ListUsersResponse;
}

#[derive(Component)]
#[shaku(interface = UserManager)]
pub(crate) struct UserManagerImpl {
    #[shaku(inject)]
    user_accessor: Arc<dyn UserAccessor>
}

impl UserManager for UserManagerImpl {
    fn create_user(&self, payload: CreateUserRequest) -> CreateUserResponse {
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
        let users = self.user_accessor.list_users();

        // Map resource data to response DTO
        ListUsersResponse {
            users: users.iter()
                .map(|it| ListUsersItem {
                    id: it.id.clone(),
                    username: it.username.clone()
                })
                .collect()
        }
    }
}