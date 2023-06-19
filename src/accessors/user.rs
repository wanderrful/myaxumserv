use std::collections::HashMap;

use crate::models::user::UserModel;
use crate::clients::local_db::LocalDb;

pub struct UserAccessor {
    db_connection: LocalDb
}

impl UserAccessor {

    pub fn new() -> Self {
        UserAccessor { db_connection: LocalDb::new() }
    }

    pub fn list_users(&mut self) -> Vec<UserModel> {
        self.db_connection.load()
    }

    pub fn save(&mut self, user: &UserModel) -> UserModel {
        self.db_connection.save(user);

        UserModel {
            id: user.id.clone(),
            username: user.username.clone()
        }
    }

}
