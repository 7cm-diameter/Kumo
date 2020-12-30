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

  let arg_matches = clap.get_matches();

  if let Some(given_arguments) = arg_matches.subcommand_matches("ls") {
    let target_folder_in_cloud = given_arguments.value_of("folder").unwrap_or_else(|| "root");
    let q_for_filter_file = given_arguments.value_of("query");
    let return_trashed_only = given_arguments.is_present("only-trashed");
    let return_shared_only = given_arguments.is_present("only-shared");
    let return_file_only = given_arguments.is_present("only-file");
    let return_folder_only = given_arguments.is_present("only-folder");
    let max_size = given_arguments
      .value_of("page-size")
      .unwrap()
      .parse::<u16>()
      .unwrap();
    let with_metadata = given_arguments.is_present("long");

    let ls_query = api::files::FilesListQuery::default()
      .enqueue_search_q(Some(&format!("'{}' in parents", target_folder_in_cloud)))
      .return_trashed_only(return_trashed_only)
      .return_shared_only(return_shared_only)
      .return_file_only(return_file_only)
      .return_folder_only(return_folder_only)
      .enqueue_search_q(q_for_filter_file)
      .set_page_size(max_size);

    let filelist = app.files_list(ls_query).await;

    filelist
      .files
      .iter()
      .for_each(|f| println!("{}", &f.format_display(with_metadata)));
  }

  if let Some(given_arguments) = arg_matches.subcommand_matches("fetch") {
    let file_tobe_fetched_from_cloud = given_arguments.value_of("filename").unwrap();
    let local_path_to_save = given_arguments.value_of("destination");
    let ls_query = api::files::FilesListQuery::default()
      .enqueue_search_q(Some(&format!("name = {:?}", &file_tobe_fetched_from_cloud))); // fetch files metadata which name is `fetched_file`

    let filelist = app.files_list(ls_query).await;
    // HACK: how to choose one item from filelist when it contains 2 or more elements.
    if let Some(file) = filelist.files.get(0) {
      app.fetch_file(file, local_path_to_save, None).await;
    }
  }

  if let Some(given_arguments) = arg_matches.subcommand_matches("upload") {
    let files_tobe_uploaded: Vec<&str> = given_arguments.values_of("paths").unwrap().collect();
    let destination_in_cloud = given_arguments.value_of("destination");

    app
      .upload_file(
        &files_tobe_uploaded,
        api::files::UploadType::Resumable,
        destination_in_cloud,
      )
      .await;
  }
}
