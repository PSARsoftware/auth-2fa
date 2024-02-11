use actix_web::{web, HttpResponse};
use serde_json::json;

use crate::models::AppState;
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
