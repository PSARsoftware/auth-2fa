use mongodb::Client;
use mongodb::options::ClientOptions;

pub(crate) mod user;

// TODO put this in config
pub(crate) const MONGO_URI: &str = "mongodb://127.0.0.1:27017";

pub async fn establish_connection(uri: &str) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.app_name = Some("Aichel".to_string());
    Client::with_options(client_options)
}
