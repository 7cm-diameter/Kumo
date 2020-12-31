use reqwest::Client;
use std::{fs, io, path::PathBuf};

use crate::util;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

const DATE_FORMAT_CHAR_LENGHT: usize = 14; // e.g. 20 12 25 18:00 (14 chars)
const FILESIZE_FORMAT_CHAR_LENGTH: usize = 6; // e.g. 123.4K (6 chars)

// https://developers.google.com/drive/api/v3/reference/files/list
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileList {
  pub kind:              Option<String>,
  pub next_page_token:   Option<String>,
  pub incomplete_search: Option<bool>,
  pub files:             Vec<FileMeta>,
}

// https://developers.google.com/drive/api/v3/reference/files#resource
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileMeta {
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
  pub size:             Option<String>,
}

impl Default for FileMeta {
  fn default() -> Self {
    Self {
      kind:             None,
      id:               None,
      name:             None,
      mime_type:        None,
      description:      None,
      trashed:          None,
      parents:          None,
      web_content_link: None,
      web_view_link:    None,
      created_time:     None,
      modified_time:    None,
      size:             None,
    }
  }
}

impl FileMeta {
  pub fn set_kind(&mut self, kind: &str) -> Self {
    self.kind = Some(String::from(kind));
    self.clone()
  }

  pub fn set_id(&mut self, id: &str) -> Self {
    self.id = Some(String::from(id));
    self.clone()
  }

  pub fn set_name(&mut self, name: &str) -> Self {
    self.name = Some(String::from(name));
    self.clone()
  }

  pub fn set_mimetype(&mut self, mimetype: &str) -> Self {
    self.mime_type = Some(String::from(mimetype));
    self.clone()
  }

  pub fn set_description(&mut self, description: &str) -> Self {
    self.description = Some(String::from(description));
    self.clone()
  }

  pub fn set_trashed(&mut self, trashed: bool) -> Self {
    self.trashed = Some(trashed);
    self.clone()
  }

  pub fn set_parents(&mut self, parents: &[&str]) -> Self {
    self.parents = Some(parents.iter().map(|s| s.to_string()).collect());
    self.clone()
  }

  pub fn set_size(&mut self, size: usize) -> Self {
    self.size = Some(size.to_string());
    self.clone()
  }

  pub fn format_display(&self, with_metadata: bool) -> String {
    let filename = self
      .name
      .clone()
      .unwrap_or_else(|| String::from("Untitled"));

    if !with_metadata {
      return filename;
    };

    let datetime = &self
      .modified_time
      .map_or_else(|| String::new(), |t| util::format_datetime(&t));
    // To align vertically
    let datetime = util::padding_left_until(datetime, DATE_FORMAT_CHAR_LENGHT);

    let size = &self.size.clone().map_or_else(
      || String::new(),
      |s| util::size_of(s.parse::<f64>().unwrap(), util::SizeUnit::B),
    );
    let size = util::padding_left_until(size, FILESIZE_FORMAT_CHAR_LENGTH);

    format!("{} {} {}", size, datetime, filename)
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

pub enum ConditionConjunction {
  And,
  Or,
}

impl ToString for ConditionConjunction {
  fn to_string(&self) -> String {
    let s = match self {
      ConditionConjunction::And => "and",
      ConditionConjunction::Or => "or",
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
        Field::Size,
      ]),
      q:                             None,
      order_by:                      None,
      page_size:                     100,
      page_token:                    None,
    }
  }
}

impl FilesListQuery {
  pub fn set_drive_id(&mut self, drive_id: &str) -> Self {
    self.drive_id = Some(String::from(drive_id));
    self.clone()
  }

  pub fn set_fields(&mut self, fields: &[Field]) -> Self {
    self.fields = fields_to_query(fields);
    self.clone()
  }

  pub fn enqueue_filed_q(&mut self, fields: &[Field]) -> Self {
    let mut new_fileds_query = fields
      .iter()
      .fold(String::from(","), |acc, s| acc + &s.to_string() + ",");
    new_fileds_query.pop(); // remove last `,` from the query because no field follows next.

    let rparen = self.fields.pop(); // remove right paren temporary from the existed query to add new fields.
    self.fields += &new_fileds_query;
    self.fields += &rparen.unwrap().to_string();
    self.clone()
  }

  pub fn include_items_form_all_drives(&mut self, include: bool) -> Self {
    self.include_items_form_all_drives = include;
    self.clone()
  }

  pub fn enqueue_search_q(&mut self, q: Option<&str>, conj: ConditionConjunction) -> Self {
    if let Some(q) = q {
      if let Some(base) = &self.q {
        self.q = Some(base.to_string() + &format!(" {} {}", conj.to_string(), q));
        return self.clone();
      }
      self.q = Some(q.to_string());
    };
    self.clone()
  }

  pub fn return_trashed_only(&mut self, trashed: bool) -> Self {
    let q = if trashed {
      "trashed = true"
    } else {
      "trashed = false"
    };
    self.enqueue_search_q(Some(q), ConditionConjunction::And);
    self.clone()
  }

  pub fn return_shared_only(&mut self, shared: bool) -> Self {
    if shared {
      let q = if let Some(q) = &self.q {
        // Because no element in intersection between "'root' in parents" and "sharedWithMe",
        q.replace("'root' in parents", "sharedWithMe")
      } else {
        String::from("sharedWithMe")
      };
      self.q = Some(q);
    }
    self.clone()
  }

  pub fn return_file_only(&mut self, file: bool) -> Self {
    if file {
      self.enqueue_search_q(
        Some("mimeType != 'application/vnd.google-apps.folder'"),
        ConditionConjunction::And,
      );
    }
    self.clone()
  }

  pub fn return_folder_only(&mut self, folder: bool) -> Self {
    if folder {
      self.enqueue_search_q(
        Some("mimeType = 'application/vnd.google-apps.folder'"),
        ConditionConjunction::And,
      );
    }
    self.clone()
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
  Size,
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
      Field::Size => "size",
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

pub enum MimeType {
  BIN,
  CSS,
  CSV,
  DOC,
  FOLDER,
  GZ,
  GIF,
  HTML,
  JPEG,
  JSON,
  MD,
  MP3,
  MP4,
  MPEG,
  PDF,
  PLAIN,
  PNG,
  PPT,
  PPTX,
  TAR,
  TOML,
  TXT,
  XLS,
  XLSX,
  YAML,
  ZIP,
}

impl From<&str> for MimeType {
  fn from(s: &str) -> Self {
    match s {
      "bin" => MimeType::BIN,
      "css" => MimeType::CSS,
      "csv" => MimeType::CSV,
      "doc" => MimeType::DOC,
      "folder" => MimeType::FOLDER,
      "gz" => MimeType::GZ,
      "gif" => MimeType::GIF,
      "html" => MimeType::HTML,
      "jpeg" => MimeType::JPEG,
      "json" => MimeType::JSON,
      "md" => MimeType::MD,
      "mp3" | "MP3" => MimeType::MP3,
      "mp4" | "MP4" => MimeType::MP4,
      "mpeg" => MimeType::MPEG,
      "pdf" => MimeType::PDF,
      "png" => MimeType::PNG,
      "ppt" => MimeType::PPT,
      "pptx" => MimeType::PPTX,
      "tar" => MimeType::TAR,
      "toml" | "tml" => MimeType::TOML,
      "txt" => MimeType::TXT,
      "xls" => MimeType::XLS,
      "xlsx" => MimeType::XLSX,
      "yaml" | "yml" => MimeType::YAML,
      "zip" => MimeType::ZIP,
      _ => MimeType::PLAIN,
    }
  }
}

impl ToString for MimeType {
  fn to_string(&self) -> String {
    let s = match self {
      MimeType::BIN => "application/octet-stream",
      MimeType::CSS => "text/csc",
      MimeType::CSV => "text/csv",
      MimeType::DOC => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
      MimeType::FOLDER => "application/vnd.google-apps.folder",
      MimeType::GZ => "application/gzip",
      MimeType::GIF => "image/gif",
      MimeType::HTML => "text/html",
      MimeType::JPEG => "image/jpeg",
      MimeType::JSON => "application/json",
      MimeType::MP3 => "audio/mpeg",
      MimeType::MP4 => "video/mp4",
      MimeType::MPEG => "video/mpeg",
      MimeType::PDF => "application/pdf",
      MimeType::PLAIN => "text/plain",
      MimeType::PNG => "image/png",
      MimeType::PPT => "application/vnd.ms-powerpoint",
      MimeType::PPTX => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
      MimeType::TAR => "applicaiton/x-tar",
      MimeType::TOML => "application/toml",
      MimeType::TXT => "text/plain",
      MimeType::XLS => "application/vnd.ms-excel",
      MimeType::XLSX => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
      MimeType::YAML => "application/yaml",
      MimeType::ZIP => "application/zip",
      _ => "",
    };
    String::from(s)
  }
}

impl Into<&str> for MimeType {
  fn into(self) -> &'static str {
    match self {
      MimeType::BIN => "application/octet-stream",
      MimeType::CSS => "text/csc",
      MimeType::CSV => "text/csv",
      MimeType::DOC => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
      MimeType::FOLDER => "application/vnd.google-apps.folder",
      MimeType::GZ => "application/gzip",
      MimeType::GIF => "image/gif",
      MimeType::HTML => "text/html",
      MimeType::JPEG => "image/jpeg",
      MimeType::JSON => "application/json",
      MimeType::MP3 => "audio/mpeg",
      MimeType::MP4 => "video/mp4",
      MimeType::MPEG => "video/mpeg",
      MimeType::PDF => "application/pdf",
      MimeType::PLAIN => "text/plain",
      MimeType::PNG => "image/png",
      MimeType::PPT => "application/vnd.ms-powerpoint",
      MimeType::PPTX => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
      MimeType::TAR => "applicaiton/x-tar",
      MimeType::TOML => "application/toml",
      MimeType::TXT => "text/plain",
      MimeType::XLS => "application/vnd.ms-excel",
      MimeType::XLSX => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
      MimeType::YAML => "application/yaml",
      MimeType::ZIP => "application/zip",
      _ => "",
    }
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

pub async fn fetch_file(
  client: &Client,
  access_token: &str,
  file: &FileMeta,
  destination: Option<&str>,
) {
  let response = client
    .get(&format!(
      "https://www.googleapis.com/drive/v3/files/{:}?alt=media",
      file.id.as_ref().unwrap()
    ))
    .bearer_auth(access_token)
    .send()
    .await
    .unwrap();

  let path_to_save = destination.map_or_else(
    || PathBuf::from(file.name.as_ref().unwrap()),
    |d| PathBuf::from(d).join(file.name.as_ref().unwrap()),
  );

  let mut f = fs::File::create(path_to_save).unwrap();

  io::copy(&mut response.bytes().await.unwrap().as_ref(), &mut f).unwrap();
}

async fn upload_resumable(client: &Client, access_token: &str, path: &PathBuf, meta: FileMeta) {
  let response = client
    .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable")
    .bearer_auth(access_token)
    .header(
      reqwest::header::CONTENT_TYPE,
      "application/json; charset=UTF-8",
    )
    .header(
      reqwest::header::CONTENT_LENGTH,
      serde_json::to_string(&meta).unwrap().len(),
    )
    .json(&meta)
    .send()
    .await
    .unwrap();

  if let Some(location) = response.headers().get("location") {
    if let Ok(resumable_uri) = location.to_str() {
      let file = fs::read(path).unwrap();
      client
        .put(resumable_uri)
        .bearer_auth(access_token)
        .header(reqwest::header::CONTENT_LENGTH, file.len())
        .body(file)
        .send()
        .await
        .unwrap();
    }
  }
}

pub async fn upload_file(
  client: &Client,
  access_token: &str,
  path: &str,
  destination: Option<&str>,
) {
  let mut meta = FileMeta::default();
  if let Some(parent) = &destination {
    meta.set_parents(&[&parent]);
  }
  let path = PathBuf::from(path);

  if let Some(filename) = path.file_name() {
    meta.set_name(filename.to_str().unwrap());
  };
  if let Some(extension) = path.extension() {
    meta.set_mimetype(MimeType::from(extension.to_str().unwrap()).into());
  }

  upload_resumable(client, access_token, &path, meta).await;
}
