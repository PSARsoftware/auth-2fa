use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc,};
use tokio::sync::Mutex;
use crate::db::mongo::MONGO_URI;
use crate::db::mongo::MongoRepo;
use crate::db::mongo::establish_connection;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub id: Option<uuid::Uuid>,
    pub email: String,
    pub name: String,
    pub password: String,

    pub otp_enabled: Option<bool>,
    pub otp_verified: Option<bool>,
    pub otp_base32: Option<String>,
    pub otp_auth_url: Option<String>,

    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}

pub struct AppState {
    pub user_repo: Arc<Mutex<MongoRepo>>,
}

impl AppState {
    pub async fn init() -> AppState {
        let mongo_client = establish_connection(MONGO_URI).await
            .expect("Establishing mongo db connection failed");
        println!("connection to mongo db has been established");

        AppState {
            user_repo: Arc::new(Mutex::new(MongoRepo::init(mongo_client).await)),
        }
    }
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
