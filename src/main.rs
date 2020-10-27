extern crate yup_oauth2 as oauth2;

use kumo::gdrive::{api, app::GoogleDriveClient};

const SCOPES: &[&str] = &[
  "https://www.googleapis.com/auth/drive",
  "https://www.googleapis.com/auth/drive.file",
  "https://www.googleapis.com/auth/drive.metadata",
];

#[tokio::main]
async fn main() {
  let app = GoogleDriveClient::default(SCOPES).await;

  app.create_file().await;
  // let x = app
  //   .files_list(
  //     api::files::FilesListQuery::default()
  //       .set_page_size(50)
  //       .include_items_form_all_drives(true)
  //       .set_order(api::files::Order::ModifiedTime),
  //   )
  //   .await;

  // x.files.iter().for_each(|f: &api::files::File| {
  //   println!("{:?}: {:?}", &f.name.as_ref(), &f.id.as_ref());
  // });

  // app.fetch_file(&x.files[2], None, None).await;
}
