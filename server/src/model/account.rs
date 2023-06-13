use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Account {
    pub email: String,
    pub password: String,
}