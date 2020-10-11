extern crate yup_oauth2 as oauth2;

use oauth2::{read_application_secret, InstalledFlowAuthenticator, InstalledFlowReturnMethod::HTTPRedirect};

#[tokio::main]
async fn main() {
  let secret = read_application_secret("clientsecret.json").await.unwrap();

  let mut auth = InstalledFlowAuthenticator::builder(secret, HTTPRedirect)
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .unwrap();

  let scopes = &["https://www.googleapis.com/auth/drive.file"];

  match auth.token(scopes).await {
    Ok(token) => println!("The token is {:?}", &token),
    Err(e) => println!("error: {:?}", &e),
  };
}
