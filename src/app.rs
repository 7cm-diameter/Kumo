use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "7cm-diameter")]
pub struct App {
  #[clap(subcommand)]
  pub subcommand: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
  Ls(Ls),
  Fetch(Fetch),
  Upload(Upload),
}

// order of priority of competing arguments
// search space: i > q > s = t = FOLDER
// search file type: i > q > f > F
// search depth: i > r > d (cannot use without `r`)
#[derive(Clap)]
pub struct Ls {
  #[clap(name = "FOLDER", index = 1, takes_value = true)]
  pub folder: String,

  #[clap(short = 'l', long = "long")]
  pub with_metadata: bool,

  #[clap(short = 'r', long = "recursive")]
  pub recursive: bool,

  // TODO: Implement after other options are implemented.
  // #[clap(name = "DEPTH", takes_value = true, short = 'L', long = "level")]
  // pub depth_to_search: bool,
  #[clap(short = 'f', long = "only-file")]
  pub search_only_file: bool,

  #[clap(short = 'F', long = "only-folder")]
  pub search_foder: bool,

  #[clap(short = 't', long = "only-trashed")]
  pub search_only_trashed: bool,

  #[clap(short = 's', long = "only-shared")]
  pub search_only_shared: bool,

  // TODO: Implement after other options are implemented.
  // #[clap(short = 'i', long = "indiscriminate")]
  // pub indiscriminate: bool,
  #[clap(short = 'q', long = "query")]
  pub query: String,
}

#[derive(Clap)]
pub struct Fetch {
  #[clap(
    name = "NAME",
    index = 1,
    takes_value = true,
    required = true,
    multiple = true
  )]
  pub names: Vec<String>,

  #[clap(
    name = "DESTINATION",
    takes_value = true,
    short = 'd',
    long = "destination"
  )]
  pub destination: String,
}

#[derive(Clap)]
pub struct Upload {
  #[clap(
    name = "PATH",
    index = 1,
    takes_value = true,
    required = true,
    multiple = true
  )]
  pub paths: Vec<String>,

  #[clap(
    name = "DESTINATION",
    takes_value = true,
    short = 'd',
    long = "destination"
  )]
  pub destination: String,
}
