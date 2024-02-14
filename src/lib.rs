use std::env;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::http::header;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use env_logger::Builder;
use crate::db::mongo::{establish_connection, MONGO_URI, MongoRepo};
use crate::db::sql::SqlRepoImpl;
use crate::DbDriver::Postgres;
use crate::models::AppState;

mod otp_handlers;
mod service;
mod response;
mod models;
mod db;

pub enum DbDriver {
    Mongo, Postgres, MySql, SQLite,
}
pub async fn start_2FA_server(
    db: DbDriver,
    cors_uri: &'static str,
    server_ip: &'static str,
    server_port: u16,
) -> std::io::Result<()>
{
    dotenv().ok();
    Builder::new().parse_env("LOG_LEVEL").init();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let app_data;
    if cfg!(postgres) || cfg!(mysql) || cfg!(sqlite) {
        let DATABASE_URL = env::var("DATABASE_URL").expect("");
        let MAX_POOL_CONNECTIONS = env::var("MAX_POOL_CONNECTIONS").expect("");
        app_data = SqlRepoImpl::init(5, &DATABASE_URL).await;
    }
    // else if cfg!(mongo) {
    //     let client = establish_connection(MONGO_URI).await.expect("could not connect to mongo db");
    //     app_data = MongoRepo::init(client).await;
    // }

    println!("🚀 Server started successfully");

    HttpServer::new( move || {
        let cors = Cors::default()
            .allowed_origin(cors_uri)
            .allowed_origin(cors_uri)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(app_data.clone())
            .configure(service::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind((server_ip, server_port))?
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
    use crate::models::AppState;
    use crate::service;

    #[tokio::test]
    async fn start_server() -> std::io::Result<()>
    {
        dotenv().ok();

        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
        env_logger::init();

        let db = AppState::init().await;
        let app_data = web::Data::new(db);

        println!("🚀 Server started successfully");

        HttpServer::new( move || {
            let cors = Cors::default()
                .allowed_origin("http://localhost:3000")
                .allowed_origin("http://localhost:3000/")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![
                    header::CONTENT_TYPE,
                    header::AUTHORIZATION,
                    header::ACCEPT,
                ])
                .supports_credentials();

            App::new()
                .app_data(app_data.clone())
                .configure(service::config)
                .wrap(cors)
                .wrap(Logger::default())
        })
            .bind(("127.0.0.1", 8000))?
            .run()
            .await?;

        Ok(())
    }
}
