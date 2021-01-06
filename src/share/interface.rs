use crate::share::util::DisplayableFileData;
use async_trait::async_trait;
use clap::ArgMatches;

#[async_trait]
pub trait Command {
  async fn ls<'a>(&self, args: &ArgMatches<'a>) -> Vec<DisplayableFileData>;
}
