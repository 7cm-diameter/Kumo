use crate::gdrive::api::files;
use clap::ArgMatches;
use reqwest::Client;

type DisplayableFileData = String;

pub fn ls<'a>(
  client: &Client,
  access_token: &str,
  args: &ArgMatches<'a>,
) -> Vec<DisplayableFileData> {
  todo!();
}

pub fn from_args_into_ls_query(args: &ArgMatches) -> files::FilesListQuery {
  todo!();
}
