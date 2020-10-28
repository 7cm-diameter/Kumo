extern crate yup_oauth2 as oauth2;

use kumo::gdrive::{api, app::GoogleDriveClient};
use std::env::args;

const SCOPES: &[&str] = &[
  "https://www.googleapis.com/auth/drive",
  "https://www.googleapis.com/auth/drive.file",
  "https://www.googleapis.com/auth/drive.metadata",
];

#[tokio::main]
async fn main() {
  let clargs: Vec<String> = args().collect();

  let app = GoogleDriveClient::default(SCOPES).await;

  if let Some(first_arg) = clargs.get(1) {
    match first_arg.as_str() {
      "ls" => {
        let x = app
          .files_list(
            api::files::FilesListQuery::default()
              .set_page_size(50)
              .include_items_form_all_drives(true)
              .set_order(api::files::Order::ModifiedTime),
          )
          .await;

        x.files
          .iter()
          .enumerate()
          .for_each(|(i, f)| println!("{:?}: {:?}", &i, &f.name));
      }
      "fetch" => {
        let x = app
          .files_list(
            api::files::FilesListQuery::default()
              .set_page_size(50)
              .include_items_form_all_drives(true)
              .set_order(api::files::Order::ModifiedTime),
          )
          .await;
        let n = if let Some(n) = clargs.get(2) {
          n.parse().unwrap_or_else(|_| 0)
        } else {
          0
        };
        app.fetch_file(&x.files[n], None, None).await;
      }
      "upload" => println!("Not implemented yet."),
      _ => println!("Command {:} does not exist.", first_arg),
    }
  } else {
    println!("No command will be executed.");
  }
}
