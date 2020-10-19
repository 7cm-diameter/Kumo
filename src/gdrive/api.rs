use crate::gdrive::response;
use reqwest::Client;

pub async fn files_list(client: &Client, access_token: &str) -> response::FileList {
  client
    .get("https://www.googleapis.com/drive/v3/files")
    .bearer_auth(access_token)
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap()
}

pub async fn drives_list(client: &Client, access_token: &str) -> response::DriveList {
  client
    .get("https://www.googleapis.com/drive/v3/drives")
    .bearer_auth(access_token)
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap()
}
