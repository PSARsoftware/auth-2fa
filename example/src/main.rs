use std::env;
use std::error::Error;
use actix_web::http::header;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use dotenv::dotenv;

use auth_2fa::{DB, init_mongo_repo, service, start_2fa_server};

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let (server_host, server_port, cors_url) = prepare_env();

    let mongo_repo = init_mongo_repo().await;
    let app_data = web::Data::new(mongo_repo);

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
            .configure(service::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
        .bind((server_host, server_port))?
        .run()
        .await?;

    Ok(())
}

fn prepare_env() -> (String, u16, String) {
    dotenv().ok();
    // Builder::new().parse_env("LOG_LEVEL").init();

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
