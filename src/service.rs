use std::future::Future;
use std::io::ErrorKind;
use std::sync::Arc;
use crate::{
    models::{
        AppState, DisableOTPSchema, GenerateOTPSchema, User, UserLoginSchema, UserRegisterSchema,
        VerifyOTPSchema,
    },
    otp_handlers::{
        disable_otp_handler_inner, generate_otp_handler_inner, validate_otp_handler_inner,
        verify_otp_handler_inner,
    },
    response::{GenericResponse, UserData, UserResponse},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde::de::StdError;
use serde::de::Unexpected::Str;
use serde_json::json;
use tokio::sync::Mutex;
use uuid::Uuid;

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "How to  Implement Two-Factor Authentication (2FA) in Rust";

    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}

#[post("/auth/register")]
async fn register_user_handler(
    body: web::Json<UserRegisterSchema>,
    data: web::Data<AppState>,
)
    -> impl Responder
{
    let user_repo = data.user_repo.lock().await;
    let email = &body.email.clone();

    if user_repo.register_user_by_email(body.into_inner()).await.is_ok() {
        return HttpResponse::Ok()
            .json(json!({"status": "success", "message": "user registered"}))
    } else {
        let msg = format!("user with email {} already exists", email);
        return HttpResponse::BadRequest()
            .json(json!({"status": "error", "message": msg}))
    }

}

#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<UserLoginSchema>,
    data: web::Data<AppState>,
) -> impl Responder {

    let user_repo = data.user_repo.lock().await;

    let user = user_repo.find_user_by_custom_field("email", &body.email.to_lowercase()).await;

    if user.is_none() {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let user = user.unwrap();

    let json_response = UserResponse {
        status: "success".to_string(),
        user: UserData::from(user),
    };

    HttpResponse::Ok().json(json_response)
}

#[post("/auth/otp/generate")]
async fn generate_otp_handler(
    body: web::Json<GenerateOTPSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    generate_otp_handler_inner(body, data).await
}

#[post("/auth/otp/verify")]
async fn verify_otp_handler(
    body: web::Json<VerifyOTPSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    verify_otp_handler_inner(body, data).await
}

#[post("/auth/otp/validate")]
async fn validate_otp_handler(
    body: web::Json<VerifyOTPSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    validate_otp_handler_inner(body, data).await
}

#[post("/auth/otp/disable")]
async fn disable_otp_handler(
    body: web::Json<DisableOTPSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    disable_otp_handler_inner(&body.user_id, data).await
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(login_user_handler)
        .service(generate_otp_handler)
        .service(verify_otp_handler)
        .service(validate_otp_handler)
        .service(disable_otp_handler);

    conf.service(scope);
}

// async fn register_user_handler_inner<Func, Fut>(
//     body: Arc<Mutex<UserRegisterSchema>>,
//     db_func: Func,
// ) -> impl Responder
//     where Func: FnOnce(Arc<Mutex<UserRegisterSchema>>) -> Fut,
//           Fut: Future<Output = Result<GenericResponse, Box<dyn StdError + Send + Sync + 'static>>>
// {
//     let res = db_func(body).await
//         .map(|resp| HttpResponse::Ok().json(
//             json!({"status": "success", "message": resp.message}))
//         );
//         //.map_err(|e| e)
//     if res.is_err() {
//         let err = res.err().unwrap();
//         // TODO map err
//         return HttpResponse::InternalServerError().json(
//             json!({"status": "error", "message": "error"})
//         )
//     } else {
//         res.unwrap()
//     }
// }
