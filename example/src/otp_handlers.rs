use actix_web::{web, HttpResponse};
use serde_json::json;
use totp_rs::{Algorithm, Secret, TOTP};
use rand::Rng;

use crate::models::{AppState, GenerateOTPSchema, VerifyOTPSchema};
use crate::response::GenericResponse;
use crate::service::user_to_response;

pub async fn disable_otp_handler_impl(user_id: String, data: web::Data<AppState>) -> HttpResponse {
    let mut vec = data.db.lock().unwrap();

    if let Some(user) = vec.iter_mut().find(|user| user.id == Some(user_id.clone())) {
        user.otp_enabled = Some(false);
        user.otp_verified = Some(false);
        user.otp_auth_url = None;
        user.otp_base32 = None;
        HttpResponse::Ok().json(json!({"user": user_to_response(user), "otp_disabled": true}))
    } else {
        let json_error = GenericResponse {
            status: "fail".to_string(),
            message: format!("No user with Id: {} found", user_id),
        };
        HttpResponse::NotFound().json(json_error)
    }
}

pub async fn validate_otp_handler_impl(
    body: web::Json<VerifyOTPSchema>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let vec = data.db.lock().unwrap();

    if let Some(user) = vec
        .iter()
        .find(|user| user.id == Some(body.user_id.to_owned()))
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

pub async fn verify_otp_handler_impl(body: web::Json<VerifyOTPSchema>, data: web::Data<AppState>) -> HttpResponse {
    let mut vec = data.db.lock().unwrap();

    if let Some(user) = vec.iter_mut().find(|user| user.id == Some(body.user_id.to_owned())) {
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

        HttpResponse::Ok().json(json!({"otp_verified": true, "user": user_to_response(user)}))
    } else {
        let json_error = GenericResponse {
            status: "fail".to_string(),
            message: format!("No user with Id: {} found", body.user_id),
        };

        HttpResponse::NotFound().json(json_error)
    }
}

pub async fn generate_otp_handler_impl(body: web::Json<GenerateOTPSchema>, data: web::Data<AppState>) -> HttpResponse {
    let mut vec = data.db.lock().unwrap();

    if let Some(user) = vec.iter_mut().find(|user| user.id == Some(body.user_id.to_owned())) {
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
