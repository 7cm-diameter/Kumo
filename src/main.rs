extern crate yup_oauth2 as oauth2;

use clap::{App, Arg, SubCommand};
use kumo::gdrive::{api, app::GoogleDriveClient};

const SCOPES: &[&str] = &[
  "https://www.googleapis.com/auth/drive",
  "https://www.googleapis.com/auth/drive.file",
  "https://www.googleapis.com/auth/drive.metadata",
];

#[tokio::main]
async fn main() {
  let clasrgs = App::new("Kumo")
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
      SubCommand::with_name("ls").arg(
        Arg::with_name("query")
          .short("q")
          .long("auery")
          .takes_value(true),
      ),
      SubCommand::with_name("fetch").args(&[
        Arg::with_name("filename")
          .takes_value(true)
          .required(true)
          .index(1),
        Arg::with_name("destination").takes_value(true).index(2),
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

  let args = clasrgs.get_matches();

  let _clientsecret = args.value_of("clientsecret").unwrap();
  let _tokencache = args.value_of("tokencache").unwrap();

  if let Some(matches) = args.subcommand_matches("ls") {
    let q = matches.value_of("query");
    let fq = api::files::FilesListQuery::default().set_q(q);
    let list = app.files_list(fq).await;
    list
      .files
      .iter()
      .enumerate()
      .for_each(|(i, f)| println!("{:?}: {:?}", &i, &f.show(20),));
  }

  if let Some(matches) = args.subcommand_matches("fetch") {
    let filename = matches.value_of("filename").unwrap();
    let destination = matches.value_of("destination");
    let fq = api::files::FilesListQuery::default().set_q(Some(&format!("name = {:?}", &filename)));
    let list = app.files_list(fq).await;
    if let Some(file) = list.files.get(0) {
      app.fetch_file(file, destination, None).await;
    }
  }

  if let Some(matches) = args.subcommand_matches("upload") {
    let paths: Vec<&str> = matches.values_of("paths").unwrap().collect();
    let destination = matches.value_of("destination");
    app
      .upload_file(&paths, api::files::UploadType::Resumable, destination)
      .await;
  }
}
