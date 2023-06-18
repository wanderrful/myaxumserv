use crate::models::user::UserModel;
use std::collections::HashMap;

pub struct UserAccessor {
    data: Vec<UserModel>
}

impl UserAccessor {

    pub fn new() -> Self {
        UserAccessor { data: Vec::new() }
    }

    pub fn list_users(&self) -> Vec<UserModel> {
        self.data.clone()
    }

    pub fn save(&self, user: &UserModel) -> UserModel {
        // TODO | Save the resource to a data store

        UserModel {
            id: user.id,
            username: user.username.clone()
        }
    }

}
