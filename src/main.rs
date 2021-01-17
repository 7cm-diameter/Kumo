extern crate yup_oauth2 as oauth2;

use clap::{App, Arg, SubCommand};
use kumo::gdrive::GoogleDriveClient;

use kumo::share::interface::Command;

const SCOPES: &[&str] = &[
  "https://www.googleapis.com/auth/drive",
  "https://www.googleapis.com/auth/drive.file",
  "https://www.googleapis.com/auth/drive.metadata",
];

#[tokio::main]
async fn main() {
  let clap = App::new("Kumo")
    .version("0.1.1")
    .author("7cm-diameter")
    .arg(
      Arg::with_name("clientsecret")
        .short("c")
        .long("client-secret")
        .takes_value(true)
        .default_value("./clientsecret.json"),
    )
    .arg(
      Arg::with_name("tokencache")
        .short("t")
        .long("token-cache")
        .takes_value(true)
        .default_value("./tokencache.json"),
    )
    .subcommands(vec![
      SubCommand::with_name("ls").args(&[
        // arguments that takes value
        Arg::with_name("folder")
          .value_name("FOLDER")
          .index(1)
          .takes_value(true),
        Arg::with_name("ordered-by")
          .short("o")
          .long("ordered-by")
          .takes_value(true),
        Arg::with_name("query")
          .short("q")
          .long("query")
          .takes_value(true),
        Arg::with_name("name-contains")
          .short("c")
          .long("name-contains")
          .takes_value(true)
          .multiple(true),
        Arg::with_name("name-matched")
          .short("m")
          .long("name-matched")
          .takes_value(true)
          .multiple(true),
        Arg::with_name("modified-before")
          .short("b")
          .long("modified-before")
          .takes_value(true),
        Arg::with_name("modified-after")
          .short("a")
          .long("modified-after")
          .takes_value(true),
        Arg::with_name("depth")
          .short("d")
          .long("depth")
          .takes_value(true),
        // arguments that do not takes value (flags)
        Arg::with_name("long").short("l").long("long"),
        Arg::with_name("recursive").short("r").long("recursive"),
        Arg::with_name("search-trashed-only")
          .short("t")
          .long("search-trashed-only"),
        Arg::with_name("search-shared-only")
          .short("s")
          .long("search-shared-only"),
        Arg::with_name("search-file-only")
          .short("f")
          .long("search-file-file"),
        Arg::with_name("search-folder-only")
          .short("F")
          .long("search-folder-only"),
      ]),
      SubCommand::with_name("fetch").args(&[
        Arg::with_name("filename")
          .short("f")
          .long("filename")
          .takes_value(true)
          .required(true)
          .multiple(true)
          .index(1),
        Arg::with_name("destination")
          .short("d")
          .long("destination")
          .takes_value(true),
      ]),
      SubCommand::with_name("upload").args(&[
        Arg::with_name("paths")
          .short("p")
          .long("paths")
          .takes_value(true)
          .required(true)
          .multiple(true)
          .index(1),
        Arg::with_name("destination")
          .short("d")
          .long("destination")
          .takes_value(true),
      ]),
    ]);

  let app = GoogleDriveClient::default(SCOPES).await;

  let arg_matches = clap.get_matches();

  if let Some(args) = arg_matches.subcommand_matches("ls") {
    let files = app.ls(args).await;
    files.iter().for_each(|f| println!("{}", f));
  }
}
