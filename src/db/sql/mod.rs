/*
    Maybe someone knows how to make repo with generic field initializing
    depending on rustc args instead of injecting pool
    Pool<Postgres | Mysql | Sqlite> during initialization?
    Or smth similar eliminating code duplication
 */

//#[cfg(all(postgres))]
pub mod postgres;

//#[cfg(all(mysql))]
pub mod mysql;

//#[cfg(all(sqlite))]
pub mod sqlite;

use std::error::Error;
use crate::models::{User, UserRegisterSchema};
use crate::response::GenericResponse;

#[cfg(test)]
mod tests {
    use std::env;
    use dotenv::dotenv;

    // #[tokio::test]
    // async fn test_compilation_and_db_request() -> std::io::Result<()>
    // {
    //     dotenv().ok();
    //
    //     if env::var_os("RUST_LOG").is_none() {
    //         env::set_var("RUST_LOG", "actix_web=info");
    //     }
    //     env_logger::init();
    //
    //     let DATABASE_URL = env::var("DATABASE_URL").unwrap();
    //
    //     let repo = SqlRepoImpl::init(5, &DATABASE_URL).await;
    //
    //     let user = repo.find_user_by_custom_field("email", "vasia_pupkin@mail.ru");
    //
    //     println!("user {:?}", user);
    //
    //     Ok(())
    // }
}
