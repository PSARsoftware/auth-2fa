use std::error::Error;
use std::io::ErrorKind;
use chrono::Utc;
use sqlx::{MySql, Pool, Postgres, Sqlite};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::postgres::PgPoolOptions;
use sqlx::sqlite::SqlitePoolOptions;
use uuid::Uuid;
use crate::models::{User, UserRegisterSchema};
use crate::response::GenericResponse;

#[cfg(all(db = "postgres"))]
mod postgres;
#[cfg(all(db = "sqlite"))]
mod sqlite;
#[cfg(all(db = "mysql"))]
mod mysql;

pub trait SqlRepo {

    //async fn init(max_connections: u32, uri: &str) -> Result<Box<Self>, sqlx::Error>;

    async fn find_user_by_custom_field(&self, field_name: &str, field: &str) -> Option<User>;

    async fn register_user_by_email(&self, user: UserRegisterSchema) -> Result<GenericResponse, Box<dyn Error>>;
}

pub struct SqlRepoImpl<Pool> {
    // #[cfg(all(db = "postgres"))]
    // pool: Pool<Postgres>,
    // #[cfg(all(db = "sqlite"))]
    // pool: Pool<Sqlite>,
    //#[cfg(all(db = "mysql"))]
    //pool: Pool<MySql>,
    pool: Pool,
}

impl SqlRepoImpl<Pool<Postgres>> {
    #[cfg(all(db = "postgres"))]
    async fn init(max_connections: u32, uri: &str) -> Result<Box<Self>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(uri).await?;

        Ok( Box::new(Self { pool } ))
    }

    // #[cfg(all(db = "sqlite"))]
    // async fn init(max_connections: u32, uri: &str) -> Result<Box<Self>, sqlx::Error> {
    //     let pool = SqlitePoolOptions::new()
    //         .max_connections(max_connections)
    //         .connect(uri).await?;
    //
    //     Ok( Box::new(Self { pool } ))
    // }
    //
    // #[cfg(all(db = "mysql"))]
    // async fn init(max_connections: u32, uri: &str) -> Result<Box<Self>, sqlx::Error> {
    //     let pool = MySqlPoolOptions::new()
    //         .max_connections(max_connections)
    //         .connect(uri).await?;
    //
    //     Ok( Box::new(Self { pool } ))
    // }
}

impl<Pool> SqlRepo for SqlRepoImpl<Pool> {
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

            let registered_user = sqlx::query_as!(
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

//async fn init(max_connections: u32, uri: &str) -> Result<Box<Self>, sqlx::Error> {
//         return if cfg!(postgres) {
//             let pool = PgPoolOptions::new()
//                 .max_connections(max_connections)
//                 .connect(uri).await?;
//
//             Ok( Box::new(Self { pool } ))
//         } else if cfg!(sqlite) {
//             let pool = SqlitePoolOptions::new()
//                 .max_connections(max_connections)
//                 .connect(uri).await?;
//
//             Ok( Box::new(Self { pool } ))
//         } else if cfg!(mysql) {
//             let pool = MySqlPoolOptions::new()
//                 .max_connections(max_connections)
//                 .connect(uri).await?;
//
//             Ok( Box::new(Self { pool } ))
//         } else {
//             panic!("");
//         };
//     }