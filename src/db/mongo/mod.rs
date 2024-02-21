use std::env;
use std::error::Error;
use std::io::ErrorKind;
use chrono::Utc;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use mongodb::bson::doc;
use uuid::Uuid;
use crate::db::Repo;
use crate::models::{AuthUser, UserRegisterSchema};
use crate::response::GenericResponse;

// TODO put this in config
pub(crate) const MONGO_URI: &str = "mongodb://127.0.0.1:27017";

pub async fn establish_connection(uri: &str) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("2FA".to_string());
    Client::with_options(client_options)
}

// TODO change it to variable passed by client of lib
const USER_DB: &str = "user_db";
const USER_COLLECTION: &str = "user-collection";

pub struct MongoRepo {
    client: Client,
}

//#[cfg(all(mongo))]
impl Repo for MongoRepo {
    async fn init()
        -> Result<Box<Self>, Box<dyn Error + Send + Sync>>
    {
        let mongo_uri = env::var("MONGO_URL").expect("MONGO_URL must be set");
        let client = establish_connection(&mongo_uri).await?;
        Ok(Box::new(Self { client }))
    }

    async fn find_user_by_email(
        &mut self,
        email: &str,
    )
        -> Option<AuthUser>
    {
        let user_collection: Collection<AuthUser> =
            self.client.database(USER_DB).collection(USER_COLLECTION);

        let query = doc! {
            "email" : email
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

    async fn find_user_by_id(
        &mut self,
        id: &str,
    )
        -> Option<AuthUser>
    {
        let user_collection: Collection<AuthUser> =
            self.client.database(USER_DB).collection(USER_COLLECTION);

        let query = doc! {
            "id" : id
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

    async fn register_user_by_email(
        &mut self,
        req_body: UserRegisterSchema,
    )
        -> Result<GenericResponse, Box<dyn Error>>
    {

        return if self.find_user_by_email(&req_body.email).await.is_none() {
            let uuid_id = Uuid::new_v4();
            let datetime = Utc::now();

            let user = AuthUser {
                id: uuid_id.to_string(),
                email: req_body.email.to_owned().to_lowercase(),
                name: req_body.name.to_owned(),
                password: Some(req_body.password.to_owned()),
                otp_enabled: Some(false),
                otp_verified: Some(false),
                otp_base32: None,
                otp_auth_url: None,
                created_at: Some(datetime),
                updated_at: Some(datetime),
            };

            let user_collection: Collection<AuthUser> =
                self.client.database(USER_DB).collection(USER_COLLECTION);
            user_collection.insert_one(&user, None).await?;

            Ok(GenericResponse::ok(&format!("registered new user with email: {}", &user.email)))
        } else {
            // TODO handle error correctly
            Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)))
        }
    }
}
