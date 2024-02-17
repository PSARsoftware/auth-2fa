use std::env;
use std::error::Error;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::dev::HttpServiceFactory;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenv::dotenv;
use env_logger::Builder;
use sqlx::{MySql, Postgres, Sqlite};
//#[cfg(all(mongo))]
use crate::db::mongo::MongoRepo;
use crate::db::Repo;
//#[cfg(all(postgres))]
use crate::db::sql::postgres::PostgresRepo;
//#[cfg(all(sqlite))]
use crate::db::sql::sqlite::SqliteRepo;
//#[cfg(all(mysql))]
use crate::db::sql::mysql::MysqlRepo;

mod otp_handlers;
#[macro_use]
mod service;
mod response;
mod models;
mod db;

#[cfg(all(mongo))]
pub async fn start_2FA_server_with_mongo_repo() -> std::io::Result<()>
{
    let (SERVER_HOST, SERVER_PORT, CORS_URL) = prepare_env();

    let app_data = init_mongo_repo().await;
    let app_data = Data::new(Arc::new(app_data));
    start_server(CORS_URL.clone(), SERVER_HOST.clone(), SERVER_PORT, app_data).await?;

    println!("🚀 Server started successfully");

    Ok(())
}

#[cfg(all(postgres))]
pub async fn start_2FA_server_with_postgres_repo() -> std::io::Result<()>
{
    let (SERVER_HOST, SERVER_PORT, CORS_URL) = prepare_env();

    let app_data = init_postgres_repo().await;
    let app_data = Data::new(Arc::new(app_data));
    start_server(CORS_URL.clone(), SERVER_HOST.clone(), SERVER_PORT, app_data).await?;

    println!("🚀 Server started successfully");

    Ok(())
}

#[cfg(all(mysql))]
pub async fn start_2FA_server_with_mysql_repo() -> std::io::Result<()>
{
    let (SERVER_HOST, SERVER_PORT, CORS_URL) = prepare_env();

    let app_data = init_mysql_repo().await;
    let app_data = Data::new(Arc::new(app_data));
    start_server(CORS_URL.clone(), SERVER_HOST.clone(), SERVER_PORT, app_data).await?;

    println!("🚀 Server started successfully");

    Ok(())
}

#[cfg(all(sqlite))]
pub async fn start_2FA_server_with_sqlite_repo() -> std::io::Result<()>
{
    let (SERVER_HOST, SERVER_PORT, CORS_URL) = prepare_env();

    let app_data = init_sqlite_repo().await;
    let app_data = Data::new(Arc::new(app_data));
    start_server(CORS_URL.clone(), SERVER_HOST.clone(), SERVER_PORT, app_data).await?;

    println!("🚀 Server started successfully");

    Ok(())
}


#[cfg(all(postgres))]
// async fn init_postgres_repo() -> Result<Box<PostgresRepo<Postgres>>, Box<dyn Error+Send+Sync>>
async fn init_postgres_repo() -> Result<Box<PostgresRepo>, Box<dyn Error+Send+Sync>>
{
    let DATABASE_URL = env::var("DATABASE_URL").expect("error: no DATABASE_URL env var");
    let MAX_POOL_CONNECTIONS = env::var("MAX_POOL_CONNECTIONS")
        .expect("error: no MAX_POOL_CONNECTIONS env var")
        .parse::<u32>()
        .expect("parse error");
    PostgresRepo::init(MAX_POOL_CONNECTIONS, &DATABASE_URL).await
}

#[cfg(all(mysql))]
// async fn init_mysql_repo() -> Result<Box<MysqlRepo<MySql>>, Box<dyn Error+Send+Sync>>
async fn init_mysql_repo() -> Result<Box<MysqlRepo>, Box<dyn Error+Send+Sync>>
{
    let DATABASE_URL = env::var("DATABASE_URL").expect("error: no DATABASE_URL env var");
    let MAX_POOL_CONNECTIONS = env::var("MAX_POOL_CONNECTIONS")
        .expect("error: no MAX_POOL_CONNECTIONS env var")
        .parse::<u32>()
        .expect("parse error");
    MysqlRepo::init(MAX_POOL_CONNECTIONS, &DATABASE_URL).await
}

#[cfg(all(sqlite))]
// async fn init_sqlite_repo() -> Result<Box<SqliteRepo<Sqlite>>, Box<dyn Error+Send+Sync>>
async fn init_sqlite_repo() -> Result<Box<SqliteRepo>, Box<dyn Error+Send+Sync>>
{
    let DATABASE_URL = env::var("DATABASE_URL").expect("error: no DATABASE_URL env var");
    let MAX_POOL_CONNECTIONS = env::var("MAX_POOL_CONNECTIONS")
        .expect("error: no MAX_POOL_CONNECTIONS env var")
        .parse::<u32>()
        .expect("parse error");
    SqliteRepo::init(MAX_POOL_CONNECTIONS, &DATABASE_URL).await
}

#[cfg(all(mongo))]
async fn init_mongo_repo() -> Result<Box<MongoRepo>, Box<dyn Error + Send + Sync>>
{
    let MONGO_URI = env::var("MONGO_URI").expect("error: no MONGO_URI env var");
    MongoRepo::init(0, &MONGO_URI).await
}

/// returns (SERVER_HOST, SERVER_PORT, CORS_URL) env vars
fn prepare_env() -> (String, u16, String) {
    dotenv().ok();
    Builder::new().parse_env("LOG_LEVEL").init();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let SERVER_HOST = env::var("SERVER_HOST").expect("error: no SERVER_HOST env var");
    let SERVER_PORT = env::var("SERVER_PORT")
        .expect("error: no SERVER_PORT env var")
        .parse::<u16>()
        .expect("SERVER_PORT parse error");
    let CORS_HOST = env::var("CORS_HOST").expect("error: no CORS_HOST env var");
    let CORS_PORT = env::var("CORS_PORT").expect("error: no CORS_PORT env var");
    let CORS_URL = String::from(CORS_HOST) + ":" + &CORS_PORT;

    (SERVER_HOST, SERVER_PORT, CORS_URL)
}

async fn start_server<Repo: Send + Sync + 'static>(
    cors_url: String,
    server_host: String,
    server_port: u16,
    app_data: Data<Repo>,
)
    -> std::io::Result<()>
{
    let server_host = server_host.clone();
    HttpServer::new( move || {
        let cors = Cors::default()
            .allowed_origin(&*cors_url)
            .allowed_origin(&*cors_url)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            //.configure(self::service::config!(&mut web::ServiceConfig, Repo))
            .configure(service::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind((server_host, server_port))?
        .run()
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use actix_cors::Cors;
    use actix_web::{App, HttpServer, web};
    use actix_web::http::header;
    use actix_web::middleware::Logger;
    use dotenv::dotenv;
    //#[cfg(all(mongo))]
    use crate::db::mongo::{MONGO_URI, MongoRepo};
    use crate::service;

    // #[tokio::test]
    // #[cfg(all(mongo))]
    // async fn start_server_with_mongo_repo() -> std::io::Result<()>
    // {
    //     dotenv().ok();
    //
    //     if std::env::var_os("RUST_LOG").is_none() {
    //         std::env::set_var("RUST_LOG", "actix_web=info");
    //     }
    //     env_logger::init();
    //
    //     let mongo_repo = MongoRepo::init(MONGO_URI).await;
    //     let app_data = web::Data::new(mongo_repo);
    //
    //     println!("🚀 Server started successfully");
    //
    //     HttpServer::new( move || {
    //         let cors = Cors::default()
    //             .allowed_origin("http://localhost:3000")
    //             .allowed_origin("http://localhost:3000/")
    //             .allowed_methods(vec!["GET", "POST"])
    //             .allowed_headers(vec![
    //                 header::CONTENT_TYPE,
    //                 header::AUTHORIZATION,
    //                 header::ACCEPT,
    //             ])
    //             .supports_credentials();
    //
    //         App::new()
    //             .app_data(app_data.clone())
    //             .configure(service::config)
    //             .wrap(cors)
    //             .wrap(Logger::default())
    //     })
    //         .bind(("127.0.0.1", 8000))?
    //         .run()
    //         .await?;
    //
    //     Ok(())
    // }
}
