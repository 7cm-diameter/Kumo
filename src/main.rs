extern crate yup_oauth2 as oauth2;

use kumo::gdrive::{
  app::GoogleDriveClient,
  response::{File, FileList},
};

const SCOPES: &[&str] = &[
  "https://www.googleapis.com/auth/drive",
  "https://www.googleapis.com/auth/drive.file",
  "https://www.googleapis.com/auth/drive.metadata",
];

#[tokio::main]
async fn main() {
  let app = GoogleDriveClient::default(SCOPES).await;

  let response = app
    .client
    .get("https://www.googleapis.com/drive/v3/files")
    .bearer_auth(app.access_token())
    .send()
    .await
    .unwrap();

  let x = response.json::<FileList>().await.unwrap();
  x.files.iter().for_each(|f: &File| {
    println!("{:}", &f.name);
  })
}
