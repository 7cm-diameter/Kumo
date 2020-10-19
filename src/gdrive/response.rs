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
  pub id:   String,
  pub kind: String,
  pub name: String,
}
