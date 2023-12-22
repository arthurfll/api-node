use mongodb::{options::ClientOptions, Client};
use std::env;





pub async fn establish_connection() -> Result<Client, mongodb::error::Error> {
   let mongo_uri = env::var("URI_MONGODB").expect("MONGOURI must be set");

   let mut client_options = ClientOptions::parse(&mongo_uri).await?;
   client_options.app_name = Some("Legado".into());

   let client = Client::with_options(client_options)?;

   Ok(client)
}