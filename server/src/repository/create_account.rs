use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAccountSchema {
    pub email: String,
    pub password: String,
}