use crate::{
  gdrive::api::files,
  gdrive::utils::find_parents_id,
  share::util::{DisplayableFileData, FormatDisplay},
};
use clap::ArgMatches;
use reqwest::Client;

pub async fn ls(
  client: &Client,
  access_token: &str,
  args: &ArgMatches<'_>,
) -> Vec<DisplayableFileData> {
  let mut ls_query = from_args_into_ls_query(client, access_token, args).await;
  let show_metadata = args.is_present("long");
  let mut displayed_file: Vec<DisplayableFileData> = Vec::new();
  // TODO: Must be refactored
  loop {
    let resp = files::files_list(client, access_token, &ls_query).await;
    let mut tmp = resp
      .files
      .iter()
      .map(|f| f.format_display(show_metadata))
      .collect();
    displayed_file.append(&mut tmp);
    if let Some(next_page_token) = resp.next_page_token {
      ls_query.set_page_token(&next_page_token);
    } else {
      break;
    }
  }
  displayed_file
}

// order of priority of competing arguments and flgas
// search space: A > q > s = t = FOLDER
// search file type: A > q > f > F
// search file name: A > q > m > x
// search depth: A > r > d (cannot use without `r`)
pub async fn from_args_into_ls_query(
  client: &Client,
  access_token: &str,
  args: &ArgMatches<'_>,
) -> files::FilesListQuery {
  let mut ls_query = files::FilesListQuery::default();

  if let Some(query) = args.value_of("query") {
    ls_query.overwrite_search_q(query);
    return ls_query;
  }

  if let Some(folder) = args.value_of("folder") {
    let parent_id = if folder == "root" {
      String::from(folder)
    } else {
      find_parents_id(client, access_token, folder).await
    };
    ls_query.overwrite_search_q(&format!("'{}' in parents", parent_id));
  }

  if args.is_present("search-shared-only") {
    ls_query.overwrite_search_q("sharedWithMe");
  }

  ls_query.enqueue_search_q(
    &format!("trashed = {:?}", args.is_present("search-only-trashed")),
    files::Conjunction::And,
  );

  if args.is_present("search-file-only") {
    ls_query.enqueue_search_q(
      "mimeType != 'application/vnd.google-apps.folder'",
      files::Conjunction::And,
    );
    return ls_query;
  }

  if args.is_present("search-folder-only") {
    ls_query.enqueue_search_q(
      "mimeType = 'application/vnd.google-apps.folder'",
      files::Conjunction::And,
    );
  }

  ls_query
}
