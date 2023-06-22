use std::sync::Arc;

use shaku::{Component, Interface};

use crate::models::user::UserModel;
use crate::clients::local_db::LocalDbClient;

pub trait UserAccessor : Interface {
    fn list_users(&self) -> Vec<UserModel>;
    fn save(&self, user: &UserModel) -> UserModel;
}

#[derive(Component)]
#[shaku(interface = UserAccessor)]
pub(crate) struct UserAccessorImpl {
    #[shaku(inject)]
    db_connection: Arc<dyn LocalDbClient>
}

impl UserAccessor for UserAccessorImpl {

    fn list_users(&self) -> Vec<UserModel> {
        self.db_connection.load()
    }

    fn save(&self, user: &UserModel) -> UserModel {
        self.db_connection.save(user);

        UserModel {
            id: user.id.clone(),
            username: user.username.clone()
        }
    }

}