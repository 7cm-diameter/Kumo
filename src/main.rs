extern crate yup_oauth2 as oauth2;

use kumo::gdrive::{
  api::{
    drives::DrivesListQuery,
    files::{FilesListQuery, Order},
  },
  app::GoogleDriveClient,
  response::{Drive, File},
};

const SCOPES: &[&str] = &[
  "https://www.googleapis.com/auth/drive",
  "https://www.googleapis.com/auth/drive.file",
  "https://www.googleapis.com/auth/drive.metadata",
];

#[tokio::main]
async fn main() {
  let app = GoogleDriveClient::default(SCOPES).await;

  let x = app
    .files_list(
      FilesListQuery::default()
        .set_page_size(1000)
        .include_items_form_all_drives(true)
        .set_order(Order::Name),
    )
    .await;

  x.files.iter().for_each(|f: &File| {
    println!("{:}: {:}", &f.name, &f.id);
  });

  let x = app.drives_list(DrivesListQuery::default()).await;

  x.drives.iter().for_each(|d: &Drive| {
    println!("{:}: {:}", &d.name, &d.id);
  })
}
