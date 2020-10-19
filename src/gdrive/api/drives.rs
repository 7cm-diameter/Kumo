use crate::gdrive::response;
use reqwest::Client;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrivesListQuery {
  page_size:  u16,
  page_token: Option<String>,
}

impl Default for DrivesListQuery {
  fn default() -> Self {
    Self {
      page_size:  100,
      page_token: None,
    }
  }
}

impl DrivesListQuery {
  pub fn set_page_size(mut self, size: u16) -> Self {
    self.page_size = size;
    self
  }

  pub fn set_page_token(mut self, token: String) -> Self {
    self.page_token = Some(token);
    self
  }
}

pub async fn drives_list(
  client: &Client,
  access_token: &str,
  params: DrivesListQuery,
) -> response::DriveList {
  client
    .get("https://www.googleapis.com/drive/v3/drives")
    .bearer_auth(access_token)
    .query(&params)
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap()
}
