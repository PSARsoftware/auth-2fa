use std::error::Error;
use std::io::ErrorKind;
use std::sync::Arc;
use chrono::Utc;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::models::{User, UserLoginSchema, UserRegisterSchema};
use crate::response::GenericResponse;

// TODO change it to variable passed by client of lib
const USER_DB: &str = "user_db";
const USER_COLLECTION: &str = "user-collection";

pub struct UserRepo {
    client: Client,
}

impl UserRepo {
    pub(crate) async fn init(client: Client) -> Self {
        Self { client }
    }

    pub(crate) async fn find_user_by_custom_field(
        &self,
        field_name: &str,
        field: &str,
    )
        -> Option<User>
    {
        let user_collection: Collection<User> =
            self.client.database(USER_DB).collection(USER_COLLECTION);

        let query = doc! {
            field_name : field
        };

        return if let Ok(mut cursor) = user_collection.find(query, None)
            .await
            .map_err(|_| Box::new(std::io::Error::from(ErrorKind::InvalidData)))
        {
            if let Err(_) = cursor.advance().await {
                return None;
            }
            let user = cursor.deserialize_current().unwrap();
            Some(user)
        } else {
            None
        }
    }

    pub(crate) async fn register_user_by_email(
        &self,
        req_body: UserRegisterSchema,
    )
        -> Result<GenericResponse, Box<dyn Error>>
    {
        
        return if self.find_user_by_custom_field("email", &req_body.email).await.is_none() {
            let uuid_id = Uuid::new_v4();
            let datetime = Utc::now();

            let user = User {
                id: Some(uuid_id.to_string()),
                email: req_body.email.to_owned().to_lowercase(),
                name: req_body.name.to_owned(),
                password: req_body.password.to_owned(),
                otp_enabled: Some(false),
                otp_verified: Some(false),
                otp_base32: None,
                otp_auth_url: None,
                createdAt: Some(datetime),
                updatedAt: Some(datetime),
            };

            let user_collection: Collection<User> =
                self.client.database(USER_DB).collection(USER_COLLECTION);
            user_collection.insert_one(&user, None).await?;

            Ok(GenericResponse::ok(&format!("registered new user with email: {}", &user.email)))
        } else {
            // TODO handle error correctly
            Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)))
        }
    }
}