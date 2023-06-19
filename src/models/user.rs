use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserModel {
    pub id: String,
    pub username: String
}
