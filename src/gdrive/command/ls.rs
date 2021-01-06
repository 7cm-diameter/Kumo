use crate::{
  gdrive::api::files,
  share::util::{DisplayableFileData, FormatDisplay},
};
use clap::ArgMatches;
use reqwest::Client;

pub async fn ls<'a>(
  client: &Client,
  access_token: &str,
  args: &ArgMatches<'a>,
) -> Vec<DisplayableFileData> {
  let ls_query = from_args_into_ls_query(args);
  files::files_list(client, access_token, ls_query)
    .await
    .files
    .iter()
    .map(|f| f.format_display(true))
    .collect()
}

pub fn from_args_into_ls_query(args: &ArgMatches) -> files::FilesListQuery {
  let mut ls_query = files::FilesListQuery::default();
  ls_query
}
