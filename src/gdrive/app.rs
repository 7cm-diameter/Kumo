extern crate yup_oauth2 as oauth2;

use crate::gdrive::{api, response};
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
  pub async fn files_list(&self, params: api::files::FileListQuery) -> response::FileList {
    api::files::files_list(&self.client, self.access_token(), params).await
  }

  pub async fn drives_list(&self) -> response::DriveList {
    api::drives::drives_list(&self.client, self.access_token()).await
  }
}
