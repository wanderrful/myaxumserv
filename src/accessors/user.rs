use std::sync::Arc;

use shaku::{Component, Interface};

use crate::models::user::UserModel;
use crate::clients::local_db::LocalDbClient;

pub trait UserAccessor : Interface {
    fn save(&self, user: &UserModel) -> UserModel;
    fn list_users(&self) -> Vec<UserModel>;
}

#[derive(Component)]
#[shaku(interface = UserAccessor)]
pub(crate) struct UserAccessorImpl {
    #[shaku(inject)]
    db_connection: Arc<dyn LocalDbClient>
}

impl UserAccessor for UserAccessorImpl {
    fn save(&self, user: &UserModel) -> UserModel {
        self.db_connection.save(user);

        UserModel {
            id: user.id.clone(),
            username: user.username.clone()
        }
    }

    fn list_users(&self) -> Vec<UserModel> {
        self.db_connection.load()
    }
}