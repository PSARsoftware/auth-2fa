use std::error::Error;

use auth_2fa::start_2FA_server_with_mongo_repo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = start_2FA_server_with_mongo_repo().await;

    Ok(())
}
