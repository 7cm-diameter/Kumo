extern crate yup_oauth2 as oauth2;

use crate::gdrive::api;
use oauth2::{
  read_application_secret, AccessToken, InstalledFlowAuthenticator,
  InstalledFlowReturnMethod::HTTPRedirect,
};
use reqwest::Client;

pub struct GoogleDriveClient {
  access_token: AccessToken,
  pub client:   Client,
}

impl GoogleDriveClient {
  // TODO: Handling error
  pub async fn default(scopes: &[&str]) -> Self {
    let secret = read_application_secret("clientsecret.json").await.unwrap();

    let auth = InstalledFlowAuthenticator::builder(secret, HTTPRedirect)
      .persist_tokens_to_disk("tokencache.json")
      .build()
      .await
      .unwrap();

    let access_token = auth.token(scopes).await.unwrap();

    let access_token = if access_token.is_expired() {
      auth.force_refreshed_token(scopes).await.unwrap()
    } else {
      access_token
    };

    Self {
      access_token,
      client: Client::new(),
    }
  }

  pub fn is_expired(&self) -> bool {
    self.access_token.is_expired()
  }

  pub fn access_token(&self) -> &str {
    self.access_token.as_str()
  }
}

impl GoogleDriveClient {
  pub async fn files_list(&self, params: api::files::FilesListQuery) -> api::files::FileList {
    api::files::files_list(&self.client, self.access_token(), params).await
  }

  pub async fn fetch_file(
    &self,
    file: &api::files::FileMeta,
    parent: Option<&str>,
    filename: Option<&str>,
  ) {
    api::files::fetch_file(&self.client, self.access_token(), file, parent, filename).await;
  }

  pub async fn drives_list(&self, params: api::drives::DrivesListQuery) -> api::drives::DriveList {
    api::drives::drives_list(&self.client, self.access_token(), params).await
  }

  pub async fn upload_file(&self, path: &str, upload_type: api::files::UploadType) {
    api::files::upload_file(&self.client, self.access_token(), path, upload_type).await;
  }
}
