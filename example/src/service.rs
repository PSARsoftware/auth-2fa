use crate::{
    models::{
        AppState, DisableOTPSchema, GenerateOTPSchema, User, UserLoginSchema, UserRegisterSchema,
        VerifyOTPSchema,
    },
    otp_handlers::{
        disable_otp_handler_impl, generate_otp_handler_impl, validate_otp_handler_impl,
        verify_otp_handler_impl,
    },
    response::{GenericResponse, UserData, UserResponse},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use serde_json::json;
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
) -> impl Responder {
    let mut vec = data.db.lock().unwrap();

    for user in vec.iter() {
        if user.email == body.email.to_lowercase() {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("User with email: {} already exists", user.email),
            };
            return HttpResponse::Conflict().json(error_response);
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    let user = User {
        id: Some(uuid_id.to_string()),
        email: body.email.to_owned().to_lowercase(),
        name: body.name.to_owned(),
        password: body.password.to_owned(),
        otp_enabled: Some(false),
        otp_verified: Some(false),
        otp_base32: None,
        otp_auth_url: None,
        createdAt: Some(datetime),
        updatedAt: Some(datetime),
    };

    vec.push(user);

    HttpResponse::Ok()
        .json(json!({"status": "success", "message": "Registered successfully, please login"}))
}

#[post("/auth/login")]
async fn login_user_handler(
    body: web::Json<UserLoginSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let vec = data.db.lock().unwrap();

    let user = vec
        .iter()
        .find(|user| user.email == body.email.to_lowercase());

    if user.is_none() {
        return HttpResponse::BadRequest()
            .json(json!({"status": "fail", "message": "Invalid email or password"}));
    }

    let user = user.unwrap().clone();

    let json_response = UserResponse {
        status: "success".to_string(),
        user: user_to_response(&user),
    };

    HttpResponse::Ok().json(json_response)
}

#[post("/auth/otp/generate")]
async fn generate_otp_handler(
    body: web::Json<GenerateOTPSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    generate_otp_handler_impl(body, data).await
}

#[post("/auth/otp/verify")]
async fn verify_otp_handler(
    body: web::Json<VerifyOTPSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    verify_otp_handler_impl(body, data).await
}

#[post("/auth/otp/validate")]
async fn validate_otp_handler(
    body: web::Json<VerifyOTPSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    validate_otp_handler_impl(body, data).await
}

#[post("/auth/otp/disable")]
async fn disable_otp_handler(
    body: web::Json<DisableOTPSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    disable_otp_handler_impl(body.user_id.clone(), data).await
}

pub fn user_to_response(user: &User) -> UserData {
    UserData {
        id: user.id.to_owned().unwrap(),
        name: user.name.to_owned(),
        email: user.email.to_owned(),
        otp_auth_url: user.otp_auth_url.to_owned(),
        otp_base32: user.otp_base32.to_owned(),
        otp_enabled: user.otp_enabled.unwrap(),
        otp_verified: user.otp_verified.unwrap(),
        createdAt: user.createdAt.unwrap(),
        updatedAt: user.updatedAt.unwrap(),
    }
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
