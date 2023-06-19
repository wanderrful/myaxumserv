use axum::Json;
use serde::{Deserialize, Serialize};

use crate::models::user::UserModel;

#[derive(Deserialize, Serialize)]
struct LocalDbModel {
    users: Vec<UserModel>
}

pub struct LocalDb {
    data: LocalDbModel
}

impl LocalDb {

    pub fn new() -> Self {
        let mut instance = LocalDb { data: LocalDbModel { users: Vec::new() } };

        instance.load();

        instance
    }

    /// Write the existing data state to the local file
    pub fn save(&mut self, user: &UserModel) {
        self.data.users.push(user.clone());

        let _ = std::fs::write(Self::get_file_path(), serde_json::to_string(&self.data)
            .expect("error serializing UserModel"));
    }

    /// Read the persisted data state from the local file
    pub fn load(&mut self) -> Vec<UserModel> {
        if self.data.users.len() == 0 {
            let file_contents = std::fs::read_to_string(Self::get_file_path())
                .expect(format!("error loading file {}", Self::get_file_path()).as_str());
            let serialized: LocalDbModel = serde_json::from_str(&*file_contents)
                .expect("error serializing file");

            self.data = serialized;
        }

        self.data.users.clone()
    }

    fn get_file_path() -> String {
        String::from("resources/local_db.json")
    }

}