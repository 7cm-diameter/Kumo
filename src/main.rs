extern crate yup_oauth2 as oauth2;

use kumo::gdrive::{app::GoogleDriveClient, response::File};

const SCOPES: &[&str] = &[
  "https://www.googleapis.com/auth/drive",
  "https://www.googleapis.com/auth/drive.file",
  "https://www.googleapis.com/auth/drive.metadata",
];

#[tokio::main]
async fn main() {
  let app = GoogleDriveClient::default(SCOPES).await;

  let x = app.files_list().await;

  x.files.iter().for_each(|f: &File| {
    println!("{:}", &f.name);
  })
}
