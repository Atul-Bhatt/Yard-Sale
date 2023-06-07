use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Account {
    id: String,
    email: String,
    password: String,
}