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

pub trait SqlRepo {

    async fn find_user_by_custom_field(&self, field_name: &str, field: &str) -> Option<User>;

    async fn register_user_by_email(&self, user: UserRegisterSchema) -> Result<GenericResponse, Box<dyn Error>>;
}

/// this repo compiles depending on args passed into rustc
/// cargo rustc -- --cfg postgres
pub struct SqlRepoImpl {
    #[cfg(all(postgres))]
    pool: Pool<Postgres>,
    #[cfg(all(sqlite))]
    pool: Pool<Sqlite>,
    #[cfg(all(mysql))]
    pool: Pool<MySql>,

}

impl SqlRepoImpl {
    #[cfg(all(postgres))]
    pub async fn init(max_connections: u32, uri: &str) -> Result<Box<Self>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(uri).await?;

        Ok( Box::new(Self { pool } ))
    }

    #[cfg(all(sqlite))]
    pub async fn init(max_connections: u32, uri: &str) -> Result<Box<Self>, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect(uri).await?;

        Ok( Box::new(Self { pool } ))
    }

    #[cfg(all(mysql))]
    pub async fn init(max_connections: u32, uri: &str) -> Result<Box<Self>, sqlx::Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .connect(uri).await?;

        Ok( Box::new(Self { pool } ))
    }
}

impl SqlRepo for SqlRepoImpl {

    #[cfg(any(postgres, mysql, sqlite))]
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

    #[cfg(any(postgres, mysql, sqlite))]
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

#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;
    use crate::db::sql::SqlRepoImpl;

    #[tokio::test]
    async fn test_compilation_and_db_request() -> std::io::Result<()>
    {
        dotenv().ok();

        if env::var_os("RUST_LOG").is_none() {
            env::set_var("RUST_LOG", "actix_web=info");
        }
        env_logger::init();

        let DATABASE_URL = env::var("DATABASE_URL").unwrap();

        let repo = SqlRepoImpl::init(5, &DATABASE_URL).await;

        let user = repo.find_user_by_custom_field("email", "vasia_pupkin@mail.ru");

        println!("user {:?}", user);

        Ok(())
    }
}
