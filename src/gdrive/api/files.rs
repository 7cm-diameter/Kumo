use reqwest::Client;
use std::{fs, io};

use chrono::{DateTime, Local};
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
  pub kind:             Option<String>,
  pub id:               Option<String>,
  pub name:             Option<String>,
  pub mime_type:        Option<String>,
  pub description:      Option<String>,
  pub trashed:          Option<bool>,
  pub parents:          Option<Vec<String>>,
  pub web_content_link: Option<String>,
  pub web_view_link:    Option<String>,
  pub created_time:     Option<DateTime<Local>>,
  pub modified_time:    Option<DateTime<Local>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilesListQuery {
  drive_id:                      Option<String>,
  include_items_form_all_drives: bool,
  fields:                        String,
  q:                             Option<String>,
  order_by:                      Option<String>,
  page_size:                     u16,
  page_token:                    Option<String>,
}

#[derive(Clone)]
pub enum Field {
  Kind,
  Id,
  Name,
  MimeType,
  Description,
  Trashed,
  Parents,
  WebContentLink,
  WebViewLink,
  CreatedTime,
  ModifiedTime,
}

impl ToString for Field {
  fn to_string(&self) -> String {
    let s = match self {
      Field::Kind => "kind",
      Field::Id => "id",
      Field::Name => "name",
      Field::MimeType => "mimeType",
      Field::Description => "description",
      Field::Trashed => "trashed",
      Field::Parents => "parents",
      Field::WebContentLink => "webContentLink",
      Field::WebViewLink => "webViewLink",
      Field::CreatedTime => "createdTime",
      Field::ModifiedTime => "modifiedTime",
    };
    String::from(s)
  }
}

fn fields_to_query(fields: &[Field]) -> String {
  let mut query = fields
    .iter()
    .fold(String::from("files("), |acc, s| acc + &s.to_string() + ",");
  query.pop(); // remove redundant `,` from the query.
  query += ")";
  query
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
      fields:                        fields_to_query(&[
        Field::Id,
        Field::Name,
        Field::MimeType,
        Field::CreatedTime,
        Field::ModifiedTime,
        Field::WebContentLink,
      ]),
      q:                             None,
      order_by:                      None,
      page_size:                     100,
      page_token:                    None,
    }
  }
}

impl FilesListQuery {
  pub fn set_drive_id(mut self, drive_id: &str) -> Self {
    self.drive_id = Some(String::from(drive_id));
    self
  }

  pub fn set_fields(mut self, fields: &[Field]) -> Self {
    self.fields = fields_to_query(fields);
    self
  }

  pub fn add_fields(mut self, fields: &[Field]) -> Self {
    let mut additional = fields
      .iter()
      .fold(String::from(","), |acc, s| acc + &s.to_string() + ",");
    additional.pop(); // remove redundant `,` from the query.
    self.fields.pop(); // remove right paren from the existed field query.
    self.fields += &additional;
    self.fields += ")";
    self
  }

  pub fn include_items_form_all_drives(mut self, include: bool) -> Self {
    self.include_items_form_all_drives = include;
    self
  }

  pub fn set_q(mut self, q: &str) -> Self {
    self.q = Some(String::from(q));
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

  pub fn set_page_token(mut self, token: &str) -> Self {
    self.page_token = Some(String::from(token));
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

pub async fn fetch_file(client: &Client, access_token: &str, file: &File) {
  let response = client
    .get(&format!(
      "https://www.googleapis.com/drive/v3/files/{:}?alt=media",
      file.id.as_ref().unwrap()
    ))
    .bearer_auth(access_token)
    .send()
    .await
    .unwrap();

  let mut f = fs::File::create(file.name.as_ref().unwrap()).unwrap();
  io::copy(&mut response.bytes().await.unwrap().as_ref(), &mut f).unwrap();
}
