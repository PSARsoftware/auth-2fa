use std::error::Error;
use std::io::ErrorKind;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use crate::db::Repo;
use crate::models::{User, UserRegisterSchema};
use crate::response::GenericResponse;


/// this repo compiles : cargo rustc -- --cfg postgres
pub struct PostgresRepo {
    pool: Pool<Postgres>,
}

//#[cfg(all(postgres))]
impl Repo<Postgres> for PostgresRepo {

    async fn init(max_connections: u32, uri: &str)
        -> Result<Box<Self>, Box<dyn Error + Send + Sync>>
    {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(uri).await?;

        Ok( Box::new(Self { pool } ))
    }

    async fn find_user_by_custom_field(&self, field_name: &str, field: &str) -> Option<User> {
        let existing_user: Result<Option<User>, _> = sqlx::query_as!(
                User,
                r#"SELECT id,email,name,password,otp_enabled,otp_verified,otp_base32,otp_auth_url,
                created_at as "createdAt", updated_at as "updatedAt" FROM auth_users WHERE $1 = $2"#,
                field_name,
                field,
            )
            .fetch_optional(&self.pool)
            .await;

        return if existing_user.is_ok() {
            existing_user.unwrap()
        } else {
            None
        }
    }

    async fn register_user_by_email(&self, user: UserRegisterSchema) -> Result<GenericResponse, Box<dyn Error>> {
        return if self.find_user_by_custom_field("email", &user.email).await.is_none() {
            let uuid_id = Uuid::new_v4();
            let datetime = Utc::now();

            let user = User {
                id: Some(uuid_id),
                email: user.email.to_owned().to_lowercase(),
                name: user.name.to_owned(),
                password: user.password.to_owned(),
                otp_enabled: Some(false),
                otp_verified: Some(false),
                otp_base32: None,
                otp_auth_url: None,
                createdAt: Some(datetime),
                updatedAt: Some(datetime),
            };

            let _ = sqlx::query_as!(
                User,
                r#"INSERT INTO auth_users (name, email, password) VALUES ($1, $2, $3)"#,
                &user.name,
                &user.email,
                &user.password,
            )
                .fetch_one(&self.pool)
                .await?;

            Ok(GenericResponse::ok(&format!("registered new user with email: {}", &user.email)))
        } else {
            // TODO handle error correctly
            Err(Box::new(std::io::Error::from(ErrorKind::InvalidData)))
        }
    }
}