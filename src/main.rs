extern crate yup_oauth2 as oauth2;

use kumo::gdrive::app::GoogleDriveClient;
use serde_json::Value;

#[tokio::main]
async fn main() {
  let scopes = &[
    "https://www.googleapis.com/auth/drive",
    "https://www.googleapis.com/auth/drive.file",
    "https://www.googleapis.com/auth/drive.metadata",
  ];

  let app = GoogleDriveClient::default(scopes).await;
  println!("3");
  let response = app
    .client
    .get("https://www.googleapis.com/drive/v3/files")
    .bearer_auth(app.access_token())
    .send()
    .await
    .unwrap();

  let ls = response
    .json::<Value>()
    .await
    .unwrap()
    .get("files")
    .unwrap()
    .clone();

  println!("{:?}", &ls);
}
