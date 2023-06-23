use serde::{Deserialize, Serialize};

use crate::models::user::UserModel;

#[derive(Deserialize, Serialize)]
pub struct LocalDbModel {
    users: Vec<UserModel>
}

impl LocalDbModel {
    pub fn save(&mut self, user: &UserModel) {
        self.users.push(user.clone());

        let _ = std::fs::write(Self::get_file_path(), serde_json::to_string(&self.users)
            .expect("error serializing UserModel"));
    }

    pub fn load(&self) -> Vec<UserModel> {
        self.users.clone()
    }

    fn get_file_path() -> String {
        String::from("resources/local_db.json")
    }
}

/// NOTE | Shaku will use Default::default() on startup, so we are effectively loading
///     data as a side effect of construction!
impl Default for LocalDbModel {
    fn default() -> Self {
        let file_contents = std::fs::read_to_string(Self::get_file_path())
            .expect(format!("error loading file {}", Self::get_file_path()).as_str());
        let users: Vec<UserModel> = serde_json::from_str(&*file_contents)
            .expect("error serializing file");

        Self { users }
    }
}
