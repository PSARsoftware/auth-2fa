use serde_json::json;

use crate::models::User;
use crate::response::GenericResponse;
use crate::service::user_to_response;
use std::sync::{Arc, Mutex};

pub struct DisableOtpResult {
    pub json_response: serde_json::Value,
    pub is_error: bool,
}

pub async fn disable_otp_handler_impl(user_id: String, users: Arc<Mutex<Vec<User>>>) -> DisableOtpResult {
    let mut vec = users.lock().unwrap();

    if let Some(user) = vec.iter_mut().find(|user| user.id == Some(user_id.clone())) {
        user.otp_enabled = Some(false);
        user.otp_verified = Some(false);
        user.otp_auth_url = None;
        user.otp_base32 = None;
        let json_response = json!({"user": user_to_response(user), "otp_disabled": true});
        DisableOtpResult {
            json_response,
            is_error: false,
        }
    } else {
        let json_error = GenericResponse {
            status: "fail".to_string(),
            message: format!("No user with Id: {} found", user_id),
        };
        DisableOtpResult {
            json_response: serde_json::to_value(json_error).unwrap(),
            is_error: true,
        }
    }
}
