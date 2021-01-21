pub mod api;
pub mod command;
pub mod util;

extern crate yup_oauth2 as oauth2;

use crate::{
  gdrive::command::ls::ls,
  share::{interface::Command, util::DisplayableFileData},
};
use async_trait::async_trait;
use clap::ArgMatches;
use oauth2::{
  read_application_secret, AccessToken, InstalledFlowAuthenticator,
  InstalledFlowReturnMethod::HTTPRedirect,
};
use once_cell::sync::OnceCell;
use reqwest::Client;

pub const SCOPES: &[&str] = &[
  "https://www.googleapis.com/auth/drive",
  "https://www.googleapis.com/auth/drive.file",
  "https://www.googleapis.com/auth/drive.metadata",
];

static GDIRVE: OnceCell<GoogleDriveClient> = OnceCell::new();

pub async fn get_gdirve_client() -> &'static GoogleDriveClient {
  if let Some(gclient) = GDIRVE.get() {
    return gclient;
  }

  let gclient = GoogleDriveClient::default(SCOPES).await;
  GDIRVE.set(gclient).expect("Failed to initialize client.");
  GDIRVE.get().unwrap()
}

#[derive(Debug)]
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

#[async_trait]
impl Command for GoogleDriveClient {
  async fn ls<'a>(&self, args: &ArgMatches<'a>) -> Vec<DisplayableFileData> {
    ls(&self.client, self.access_token.as_str(), args).await
  }
}
