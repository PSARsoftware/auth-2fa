use std::env;
use std::error::Error;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use dotenv::dotenv;
use env_logger::Builder;
use crate::db::mongo::MongoRepo;
use crate::db::Repo;
use crate::db::sql::postgres::PostgresRepo;


mod otp_handlers;
#[macro_use]
mod service;
mod response;
mod models;
mod db;
mod schema;

//#[cfg(all(mongo))]
pub async fn start_2FA_server_with_mongo_repo() -> std::io::Result<()>
{
    let (SERVER_HOST, SERVER_PORT, CORS_URL) = prepare_env();

    let repo = MongoRepo::init().await.expect("could not initialize mongo db repository");
    let app_data = Data::new(Arc::new(repo));
    start_server(CORS_URL.clone(), SERVER_HOST.clone(), SERVER_PORT, app_data).await?;

    println!("🚀 Server started successfully");

    Ok(())
}

//#[cfg(all(postgres))]
pub async fn start_2FA_server_with_postgres_repo() -> std::io::Result<()>
{
    let (SERVER_HOST, SERVER_PORT, CORS_URL) = prepare_env();

    let repo = init_postgres_repo().await;
    let app_data = Data::new(Arc::new(repo));
    start_server(CORS_URL.clone(), SERVER_HOST.clone(), SERVER_PORT, app_data).await?;

    println!("🚀 Server started successfully");

    Ok(())
}


//#[cfg(all(postgres))]
async fn init_postgres_repo() -> Result<Box<PostgresRepo>, Box<dyn Error+Send+Sync>>
{
    PostgresRepo::init().await
}



/// returns (SERVER_HOST, SERVER_PORT, CORS_URL) env vars
fn prepare_env() -> (String, u16, String) {
    dotenv().ok();
    Builder::new().parse_env("LOG_LEVEL").init();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    //env_logger::init();

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

    use crate::{service, start_2FA_server_with_mongo_repo, start_2FA_server_with_postgres_repo};

    #[tokio::test]
    async fn start_server_with_mongo_repo() -> std::io::Result<()>
    {
        start_2FA_server_with_mongo_repo().await
    }

    #[tokio::test]
    async fn start_server_with_postgres_repo() -> std::io::Result<()>
    {
        start_2FA_server_with_postgres_repo().await
    }
}
