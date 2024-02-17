use std::error::Error;
use std::io::ErrorKind;
use chrono::Utc;
use mongodb::{Client, Collection};
use mongodb::options::ClientOptions;
use mongodb::bson::doc;
use uuid::Uuid;
use crate::db::Repo;
use crate::models::{User, UserLoginSchema, UserRegisterSchema};
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
impl Repo<()> for MongoRepo {
    async fn init(_max_connections: u32, mongo_uri: &str)
        -> Result<Box<Self>, Box<dyn Error + Send + Sync>>
    {
        let client = establish_connection(mongo_uri).await?;
        Ok(Box::new(Self { client }))
    }

    async fn find_user_by_custom_field(
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

    async fn register_user_by_email(
        &self,
        req_body: UserRegisterSchema,
    )
        -> Result<GenericResponse, Box<dyn Error>>
    {

        return if self.find_user_by_custom_field("email", &req_body.email).await.is_none() {
            let uuid_id = Uuid::new_v4();
            let datetime = Utc::now();

            let user = User {
                id: Some(uuid_id),
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
