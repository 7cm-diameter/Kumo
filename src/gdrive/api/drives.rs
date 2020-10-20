use reqwest::Client;

use serde::{Deserialize, Serialize};

// https://developers.google.com/drive/api/v3/reference/drives/list
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveList {
  pub kind:            String,
  pub next_page_token: Option<String>,
  pub drives:          Vec<Drive>,
}

// https://developers.google.com/drive/api/v3/reference/drives#resource
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Drive {
  pub kind: String,
  pub id:   String,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DrivesListQuery {
  page_size:  u16,
  page_token: Option<String>,
  q:          Option<String>,
}

impl Default for DrivesListQuery {
  fn default() -> Self {
    Self {
      page_size:  100,
      page_token: None,
      q:          None,
    }
  }
}

impl DrivesListQuery {
  pub fn set_page_size(mut self, size: u16) -> Self {
    self.page_size = size;
    self
  }

  pub fn set_page_token(mut self, token: &str) -> Self {
    self.page_token = Some(String::from(token));
    self
  }

  pub fn set_q(mut self, q: &str) -> Self {
    self.q = Some(String::from(q));
    self
  }
}

pub async fn drives_list(
  client: &Client,
  access_token: &str,
  params: DrivesListQuery,
) -> DriveList {
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
