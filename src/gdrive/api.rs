use crate::gdrive::response;
use reqwest::{Client, Response};

async fn get(client: &Client, access_token: &str, url: &str) -> Response {
  client
    .get(url)
    .bearer_auth(access_token)
    .send()
    .await
    .unwrap()
}

pub async fn files_list(client: &Client, access_token: &str) -> response::FileList {
  get(
    client,
    access_token,
    "https://www.googleapis.com/drive/v3/files",
  )
  .await
  .json::<response::FileList>()
  .await
  .unwrap()
}
