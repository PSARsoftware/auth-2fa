use std::sync::Arc;
use actix_web::{web, HttpResponse};
use serde_json::json;
use totp_rs::{Algorithm, Secret, TOTP};
use rand::Rng;
use tokio::sync::Mutex;

use crate::models::{GenerateOTPSchema, VerifyOTPSchema};
use crate::response::{GenericResponse, UserData};
use crate::db::{GenericRepo, Repo};

pub async fn disable_otp_handler_inner(
    user_id: &str,
    data: web::Data<GenericRepo>)
    -> HttpResponse
{
    async fn inner_fn<DB, R: Repo<DB>>(repo: &Arc<Mutex<R>>, user_id : &str)
        -> HttpResponse 
    {
        let repo = repo.lock().await;
        if let Some(mut user) =
            repo.find_user_by_custom_field("user_id", user_id).await
        {
            user.otp_enabled = Some(false);
            user.otp_verified = Some(false);
            user.otp_auth_url = None;
            user.otp_base32 = None;

            HttpResponse::Ok().json(json!({
                    "user": UserData::from(user), 
                    "otp_disabled": true })
            )
        } else {
            let json_error = GenericResponse {
                status: "fail".to_string(),
                message: format!("No user with Id: {} found", user_id),
            };

            HttpResponse::NotFound().json(json_error)
        }
    }
    
    match &*data.into_inner() {
        GenericRepo::Mongo { repo } => { inner_fn(repo, user_id).await }
        GenericRepo::Postgres { repo } => { inner_fn(repo, user_id).await }
        GenericRepo::Mysql { repo } => {  inner_fn(repo, user_id).await }
        GenericRepo::Sqlite { repo } => { inner_fn(repo, user_id).await }
    }
}

pub async fn validate_otp_handler_inner(
    body: web::Json<VerifyOTPSchema>,
    data: web::Data<GenericRepo>,
) -> HttpResponse
{
    async fn inner_fn<DB, R: Repo<DB>>(repo: &Arc<Mutex<R>>, body: web::Json<VerifyOTPSchema>)
        -> HttpResponse
    {
        let repo = repo.lock().await;

        if let Some(user) =
            repo.find_user_by_custom_field("user_id", &body.user_id).await
        {
            if let Some(otp_enabled) = user.otp_enabled {
                if !otp_enabled {
                    let json_error = GenericResponse {
                        status: "fail".to_string(),
                        message: "2FA not enabled".to_string(),
                    };

                    return HttpResponse::Forbidden().json(json_error);
                }
            }

            if let Some(otp_base32) = &user.otp_base32 {
                let totp = TOTP::new(
                    Algorithm::SHA1,
                    6,
                    1,
                    30,
                    Secret::Encoded(otp_base32.clone()).to_bytes().unwrap(),
                )
                    .unwrap();

                if !totp.check_current(&body.token).unwrap() {
                    return HttpResponse::Forbidden().json(
                        json!({"status": "fail", "message": "Token is invalid or user doesn't exist"}),
                    );
                }
            } else {
                return HttpResponse::InternalServerError()
                    .json(json!({"status": "fail", "message": "OTP base32 is missing"}));
            }

            HttpResponse::Ok().json(json!({"otp_valid": true}))
        } else {
            let json_error = GenericResponse {
                status: "fail".to_string(),
                message: format!("No user with Id: {} found", body.user_id),
            };

            HttpResponse::NotFound().json(json_error)
        }
    }
    
    match &*data.into_inner() {
        GenericRepo::Mongo { repo } => { inner_fn(repo, body).await }
        GenericRepo::Postgres { repo } => { inner_fn(repo, body).await }
        GenericRepo::Mysql { repo } => {  inner_fn(repo, body).await }
        GenericRepo::Sqlite { repo } => { inner_fn(repo, body).await }
    }
}

pub async fn verify_otp_handler_inner(
    body: web::Json<VerifyOTPSchema>,
    data: web::Data<GenericRepo>)
    -> HttpResponse
{
    async fn inner_fn<DB, R: Repo<DB>>(repo: &Arc<Mutex<R>>, body: web::Json<VerifyOTPSchema>)
        -> HttpResponse
    {
        let user_repo = repo.lock().await;

        if let Some(mut user) =
            user_repo.find_user_by_custom_field("user_id", &body.user_id).await 
        {
            let otp_base32 = user.otp_base32.to_owned().unwrap();

            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                Secret::Encoded(otp_base32.clone()).to_bytes().unwrap(),
            )
                .unwrap();

            let is_valid = totp.check_current(&body.token).unwrap();

            if !is_valid {
                let json_error = GenericResponse {
                    status: "fail".to_string(),
                    message: "Token is invalid or user doesn't exist".to_string(),
                };

                return HttpResponse::Forbidden().json(json_error);
            }

            user.otp_enabled = Some(true);
            user.otp_verified = Some(true);

            HttpResponse::Ok().json(json!({"otp_verified": true, "user": UserData::from(user)}))
        } else {
            let json_error = GenericResponse {
                status: "fail".to_string(),
                message: format!("No user with Id: {} found", body.user_id),
            };

            HttpResponse::NotFound().json(json_error)
        }
    }
    
    match &*data.into_inner() {
        GenericRepo::Mongo { repo } => { inner_fn(repo, body).await }
        GenericRepo::Postgres { repo } => { inner_fn(repo, body).await }
        GenericRepo::Mysql { repo } => {  inner_fn(repo, body).await }
        GenericRepo::Sqlite { repo } => { inner_fn(repo, body).await }
    }
}

pub async fn generate_otp_handler_inner(
    body: web::Json<GenerateOTPSchema>,
    data: web::Data<GenericRepo>)
    -> HttpResponse
{
    async fn inner_fn<DB, R: Repo<DB>>(repo: &Arc<Mutex<R>>, body: web::Json<GenerateOTPSchema>)
        -> HttpResponse
    {
        let user_repo = repo.lock().await;

        if let Some(mut user) =
            user_repo.find_user_by_custom_field("user_id", &body.user_id).await
        {
            let mut rng = rand::thread_rng();
            let data_byte: [u8; 21] = rng.gen();
            let base32_string = base32::encode(base32::Alphabet::RFC4648 { padding: false }, &data_byte);

            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                Secret::Encoded(base32_string.clone()).to_bytes().unwrap(),
            )
                .unwrap();

            let otp_base32 = totp.get_secret_base32();
            let email = body.email.to_owned();
            let issuer = "PSAR";
            let otp_auth_url = format!("otpauth://totp/{issuer}:{email}?secret={otp_base32}&issuer={issuer}");

            user.otp_base32 = Some(otp_base32.clone());
            user.otp_auth_url = Some(otp_auth_url.clone());

            HttpResponse::Ok().json(json!({"base32": otp_base32, "otpauth_url": otp_auth_url}))
        } else {
            let json_error = GenericResponse {
                status: "fail".to_string(),
                message: format!("No user with Id: {} found", body.user_id),
            };
            HttpResponse::NotFound().json(json_error)
        }
    }

    match &*data.into_inner() {
        GenericRepo::Mongo { repo } => { inner_fn(repo, body).await }
        GenericRepo::Postgres { repo } => { inner_fn(repo, body).await }
        GenericRepo::Mysql { repo } => {  inner_fn(repo, body).await }
        GenericRepo::Sqlite { repo } => { inner_fn(repo, body).await }
    }
}

// async fn call_inner_fn<R: Repo, _Data, Func, Fut>(
//     func: Func, 
//     _data: _Data,
//     _repo: &Arc<Mutex<R>>,
// ) 
//     -> HttpResponse
//     where Func: FnOnce(&Arc<Mutex<R>>, _Data) -> Fut,
//           Fut: Future<Output = HttpResponse>
// {
//     func(_repo, _data).await
// }
