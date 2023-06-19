use crate::models::user::UserModel;
use crate::clients::local_db::LocalDbClient;

pub struct UserAccessor {
    db_connection: LocalDbClient
}

impl UserAccessor {

    pub fn new() -> Self {
        UserAccessor { db_connection: LocalDbClient::new() }
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
