use chrono::{DateTime,Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug,sqlx::FromRow, Serialize)]
pub struct User{
    pub id: i64,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    #[serde(skip)]
    pub created_at: DateTime<Utc>, // instead of NaiveDateTime
}

#[derive(Debug,Deserialize)]
pub struct RegisterUser{
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug,Deserialize)]
pub struct LoginUser{
    pub email: String,
    pub password: String
}