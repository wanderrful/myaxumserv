use serde::{Deserialize, Serialize};

use crate::managers::user::{CreateUserRequest, ListUsersItem};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserModel {
    pub id: String,
    pub username: String
}

impl Into<CreateUserRequest> for UserModel {

    fn into(self) -> CreateUserRequest {
        CreateUserRequest {
            username: self.username
        }
    }

}

impl From<CreateUserRequest> for UserModel {

    fn from(value: CreateUserRequest) -> Self {
        UserModel {
            id: uuid::Uuid::new_v4().to_string(),
            username: value.username
        }
    }

}

impl Into<ListUsersItem> for UserModel {

    fn into(self) -> ListUsersItem {
        ListUsersItem {
            id: self.id,
            username: self.username
        }
    }

}

impl From<ListUsersItem> for UserModel {

    fn from(value: ListUsersItem) -> Self {
        UserModel {
            id: value.id,
            username: value.username
        }
    }

}
