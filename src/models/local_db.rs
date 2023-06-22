use serde::{Deserialize, Serialize};

use crate::models::user::UserModel;

#[derive(Default, Deserialize, Serialize)]
pub struct LocalDbModel {
    users: Vec<UserModel>
}

impl LocalDbModel {
    pub fn save(&mut self, user: &UserModel) {
        self.users.push(user.clone());

        let _ = std::fs::write(Self::get_file_path(), serde_json::to_string(&self.users)
            .expect("error serializing UserModel"));
    }

    pub fn load(&mut self) -> Vec<UserModel> {
        if self.users.len() == 0 {
            let file_contents = std::fs::read_to_string(Self::get_file_path())
                .expect(format!("error loading file {}", Self::get_file_path()).as_str());
            let serialized: Vec<UserModel> = serde_json::from_str(&*file_contents)
                .expect("error serializing file");

            self.users = serialized;
        }

        self.users.clone()
    }

    fn get_file_path() -> String {
        String::from("resources/local_db.json")
    }
}
