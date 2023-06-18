use crate::models::user::UserModel;
use crate::routes::users::CreateUserRequest;

pub struct UserManager;

impl UserManager {

    pub fn create(&self, request: &CreateUserRequest) -> UserModel {
        let CreateUserRequest { username} = request;

        // Create the resource to save to the data store
        let user = UserModel {
            id: 1337,
            username: username.clone(),
        };

        // TODO | Save the resource to a data store

        user
    }

}
