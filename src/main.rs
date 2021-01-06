extern crate yup_oauth2 as oauth2;

use clap::{App, Arg, SubCommand};
use kumo::gdrive::GoogleDriveClient;

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
        Arg::with_name("folder").index(1).takes_value(true),
        Arg::with_name("query")
          .short("q")
          .long("query")
          .takes_value(true),
        Arg::with_name("page-size")
          .short("S")
          .long("max-size")
          .takes_value(true)
          .default_value("100"),
        Arg::with_name("only-trashed")
          .short("t")
          .long("only-trashed"),
        Arg::with_name("only-shared").short("s").long("only-shared"),
        Arg::with_name("only-file").short("f").long("only-file"),
        Arg::with_name("only-folder").short("F").long("only-folder"),
        Arg::with_name("long").short("l").long("long"),
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

  let _app = GoogleDriveClient::default(SCOPES).await;

  let _arg_matches = clap.get_matches();
}
