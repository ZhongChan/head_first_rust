use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    pub id: Option<AccountId>,
    pub nick_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AccountId(pub i32);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NewAccount {
    pub nick_name: String,
    pub email: String,
    pub password: String,
}
