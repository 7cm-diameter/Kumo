use crate::gdrive::response;
use reqwest::Client;

use serde::{Deserialize, Serialize};

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
