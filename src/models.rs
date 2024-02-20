use chrono::prelude::*;
use diesel::{Insertable, Queryable, Selectable};
//use diesel::sql_types::Datetime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::auth_users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthUser {
    pub id: String,
    pub email: String,
    pub name: String,
    pub password: Option<String>,

    pub otp_enabled: Option<bool>,
    pub otp_verified: Option<bool>,
    pub otp_base32: Option<String>,
    pub otp_auth_url: Option<String>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::auth_users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub password: &'a str,

    pub otp_enabled: Option<bool>,
    pub otp_verified: Option<bool>,
    pub otp_base32: Option<&'a str>,
    pub otp_auth_url: Option<&'a str>,

    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UserRegisterSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UserLoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateOTPSchema {
    pub email: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct VerifyOTPSchema {
    pub user_id: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct DisableOTPSchema {
    pub user_id: String,
}
