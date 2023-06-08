use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Default, FromRow)]
pub struct Account {
    pub email: String,
    pub password: String,
}