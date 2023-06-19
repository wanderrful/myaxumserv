use crate::accessors::user::UserAccessor;
use crate::models::user::UserModel;
use crate::resources::user::{CreateUserRequest, CreateUserResponse, ListUsersResponse, ListUsersItem};
use crate::utils::uuid::generate_uuid;

pub struct UserManager;

impl UserManager {

    pub fn create_user(&self, payload: CreateUserRequest) -> CreateUserResponse {
        // TODO | Dependency injection? Reference via context?
        let mut user_accessor = UserAccessor::new();

        // TODO | Validate the incoming data

        // Map the request to the UserModel
        let user_model = UserModel {
            id: generate_uuid(),
            username: payload.username.clone()
        };

        // Save the resource to the data store
        let user = user_accessor.save(&user_model);

        // Map the new resource to the response DTO
        CreateUserResponse {
            id: user.id,
            username: user.username,
        }
    }

    pub fn list_users(&self) -> ListUsersResponse {
        // TODO | Dependency innjection? Reference via context?
        let mut user_accessor = UserAccessor::new();

        // TODO | Validate the incoming data

        let users = user_accessor.list_users();

        // Map resource data to response DTO
        ListUsersResponse {
            data: users.iter()
                .map(|it| ListUsersItem {
                    id: generate_uuid(),
                    username: it.username.clone()
                })
                .collect()
        }
    }

}