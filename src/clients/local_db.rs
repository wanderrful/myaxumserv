use std::sync::RwLock;

use shaku::{Component, Interface};

use crate::models::local_db::LocalDbModel;
use crate::models::user::UserModel;

pub trait LocalDbClient : Interface {
    fn save(&self, user: &UserModel);
    fn load(&self) -> Vec<UserModel>;
}

#[derive(Component)]
#[shaku(interface = LocalDbClient)]
pub struct LocalDbClientImpl {
    #[shaku(default)]
    data: RwLock<LocalDbModel>
}

impl LocalDbClient for LocalDbClientImpl {
    /// Write the existing data state to the local file
    fn save(&self, user: &UserModel) {
        self.data.write().expect("error unlocking LocalDbModel").save(user);
    }

    /// Read the persisted data state from the local file
    fn load(&self) -> Vec<UserModel> {
        self.data.read().expect("error unlocking LocalDbModel").load()
    }
}
