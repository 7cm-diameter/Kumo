extern crate yup_oauth2 as oauth2;

use kumo::gdrive::GoogleDriveClient;

const SCOPES: &[&str] = &[
  "https://www.googleapis.com/auth/drive",
  "https://www.googleapis.com/auth/drive.file",
  "https://www.googleapis.com/auth/drive.metadata",
];

#[tokio::main]
async fn main() {
  let _app = GoogleDriveClient::default(SCOPES).await;
  todo!();
}
