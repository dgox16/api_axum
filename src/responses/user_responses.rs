use chrono::prelude::*;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub usuario: String,
    pub email: String,
    pub rol: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UserData {
    pub usuario: FilteredUser,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}
