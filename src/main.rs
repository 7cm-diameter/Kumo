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

  let x = app
    .files_list(
      api::files::FilesListQuery::default()
        .set_page_size(1000)
        .include_items_form_all_drives(true)
        .set_order(api::files::Order::Name),
    )
    .await;

  x.files.iter().for_each(|f: &api::files::File| {
    println!("{:?}: {:?}", &f.name.as_ref(), &f.web_content_link.as_ref());
  });
}
