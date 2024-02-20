use chrono::prelude::*;
use serde::Serialize;
use crate::models::AuthUser;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

impl GenericResponse {
    pub fn ok(message: &str) -> Self {
        Self {
            status: "success".to_string(),
            message: message.to_string(),
        }
    }

    pub fn error(message: &str) -> Self {
        Self {
            status: "error".to_string(),
            message: message.to_string(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct UserData {
    pub id: String,
    pub email: String,
    pub name: String,

    pub otp_enabled: bool,
    pub otp_verified: bool,
    pub otp_base32: Option<String>,
    pub otp_auth_url: Option<String>,

    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

impl From<AuthUser> for UserData {
    fn from(user: AuthUser) -> Self {
        UserData {
            id: user.id,
            name: user.name,
            email: user.email,
            otp_auth_url: user.otp_auth_url,
            otp_base32: user.otp_base32,
            otp_enabled: user.otp_enabled.unwrap(),
            otp_verified: user.otp_verified.unwrap(),
            createdAt: user.created_at.unwrap(),
            updatedAt: user.updated_at.unwrap(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub user: UserData,
}
