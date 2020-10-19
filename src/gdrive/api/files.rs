use reqwest::Client;

use serde::{Deserialize, Serialize};

// https://developers.google.com/drive/api/v3/reference/files/list
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileList {
  pub kind:              Option<String>,
  pub next_page_token:   Option<String>,
  pub incomplete_search: Option<bool>,
  pub files:             Vec<File>,
}

// https://developers.google.com/drive/api/v3/reference/files#resource
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
  pub id:        String,
  pub kind:      String,
  pub mime_type: String,
  pub name:      String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesListQuery {
  drive_id:                      Option<String>,
  include_items_form_all_drives: bool,
  q:                             Option<String>,
  order_by:                      Option<String>,
  page_size:                     u16,
  page_token:                    Option<String>,
}

pub enum Order {
  CreatedTime,
  Folder,
  ModifiedByMeTime,
  ModifiedTime,
  Name,
  NameNatural,
  QuotaBytesUsed,
  Recency,
  SharedWithMeTime,
  Starred,
  ViewedByMeTime,
}

impl ToString for Order {
  fn to_string(&self) -> String {
    let s = match self {
      Order::CreatedTime => "createdTime",
      Order::Folder => "folder",
      Order::ModifiedByMeTime => "modifiedByMeTime",
      Order::ModifiedTime => "modifiedTime",
      Order::Name => "name",
      Order::NameNatural => "name_natural",
      Order::QuotaBytesUsed => "quotaBytesUsed",
      Order::Recency => "recency",
      Order::SharedWithMeTime => "sharedWithMeTime",
      Order::Starred => "starred",
      Order::ViewedByMeTime => "viewedTimeByMe",
    };
    String::from(s)
  }
}

impl Default for FilesListQuery {
  fn default() -> Self {
    Self {
      drive_id:                      None,
      include_items_form_all_drives: false,
      q:                             None,
      order_by:                      None,
      page_size:                     100,
      page_token:                    None,
    }
  }
}

impl FilesListQuery {
  pub fn set_drive_id(mut self, drive_id: String) -> Self {
    self.drive_id = Some(drive_id);
    self
  }

  pub fn include_items_form_all_drives(mut self, include: bool) -> Self {
    self.include_items_form_all_drives = include;
    self
  }

  pub fn set_q(mut self, q: String) -> Self {
    self.q = Some(q);
    self
  }

  pub fn set_order(mut self, order: Order) -> Self {
    self.order_by = Some(order.to_string());
    self
  }

  pub fn set_page_size(mut self, size: u16) -> Self {
    self.page_size = size;
    self
  }

  pub fn set_page_token(mut self, token: String) -> Self {
    self.page_token = Some(token);
    self
  }
}

pub async fn files_list(client: &Client, access_token: &str, params: FilesListQuery) -> FileList {
  client
    .get("https://www.googleapis.com/drive/v3/files")
    .bearer_auth(access_token)
    .query(&params)
    .send()
    .await
    .unwrap()
    .json()
    .await
    .unwrap()
}
