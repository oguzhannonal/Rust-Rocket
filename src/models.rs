use rocket::form::FromForm;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Child {
    pub id: i32,
    pub column1: String,
    pub column2: String,
    pub column3: String,
    pub column4: String,
    pub column5: String,
    pub column6: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableRow {
    pub id: i32,
    pub column1: String,
    pub column2: String,
    pub column3: String,
    pub column4: String,
    pub column5: String,
    pub column6: String,
    pub children: Vec<Child>,
}

#[derive(Debug, FromForm)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
} 