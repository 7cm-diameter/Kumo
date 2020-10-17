use serde::{Deserialize, Serialize};

// https://developers.google.com/drive/api/v3/reference/files/list
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileList {
  pub kind:              String,
  pub next_page_token:   String,
  pub incomplete_search: bool,
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
