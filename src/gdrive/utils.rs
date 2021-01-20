use crate::gdrive::{api::files, get_gdirve_client};
use once_cell::sync::OnceCell;
use reqwest::Client;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::PathBuf;

static ID2FOLDER: OnceCell<IdToFileMeta> = OnceCell::new();

pub async fn get_id2folder() -> &'static IdToFileMeta {
  if let Some(id2folder) = ID2FOLDER.get() {
    return id2folder;
  }

  let gclient = get_gdirve_client().await;
  let all_folders = fetch_all_folders(&gclient.client, gclient.access_token()).await;
  let id2folder = hash_id_to_metadata(&all_folders);
  ID2FOLDER
    .set(id2folder)
    .expect("Failed to fetch metafiles of folders in drive.");
  ID2FOLDER.get().unwrap()
}

pub async fn find_parents_id(expected_path: &str) -> String {
  let path_components: Vec<&str> = expected_path.split('/').collect();
  let id2meta = get_id2folder().await;
  let candidate_folders: Vec<files::FileMeta> = id2meta
    .values()
    .filter(|f| {
      let filename = f.name.clone().unwrap_or_else(String::new);
      let basename = path_components.last().unwrap_or(&"");
      basename == &filename.as_str()
    })
    .cloned()
    .collect();

  let ids_lead_to_candidates: Vec<Vec<String>> = candidate_folders
    .iter()
    .filter_map(|f| trace_id_paths(f, &id2meta))
    .flatten()
    .map(|mut ids| {
      ids.reverse();
      ids
    })
    .collect();

  let paths_to_candidates: Vec<String> = ids_lead_to_candidates
    .iter()
    .filter_map(|ids| path_from_ids(ids, &id2meta))
    .collect();

  for (path, ids) in paths_to_candidates.iter().zip(ids_lead_to_candidates) {
    if path == expected_path {
      return ids.last().unwrap().clone();
    }
  }
  String::new()
}

pub async fn fetch_all_folders<'a>(
  client: &'a Client,
  access_token: &'a str,
) -> Vec<files::FileMeta> {
  let mut ls_query = files::FilesListQuery::default();
  ls_query.enqueue_search_q(
    "mimeType = 'application/vnd.google-apps.folder'",
    files::Conjunction::And,
  );
  files::files_list(client, access_token, &ls_query)
    .await
    .files
}

pub async fn fetch_all_contents<'a>(
  client: &'a Client,
  access_token: &'a str,
) -> Vec<files::FileMeta> {
  let ls_query = files::FilesListQuery::default();
  files::files_list(client, access_token, &ls_query)
    .await
    .files
}

pub type IdToFileMeta = HashMap<String, files::FileMeta>;

pub fn hash_id_to_metadata(metadata: &[files::FileMeta]) -> IdToFileMeta {
  let mut hash = HashMap::new();
  metadata.iter().for_each(|m| {
    let id = m.id.clone().unwrap_or_else(String::new);
    hash.insert(id, m.clone());
  });
  hash
}

type IdPath = Vec<String>;

fn append_parents_id(id_path: IdPath, id2meta: &IdToFileMeta) -> Vec<Vec<String>> {
  let shallowest_folder_id = id_path.last().unwrap();
  if let Some(shallowest_folder) = id2meta.get(shallowest_folder_id) {
    if let Some(parents_id) = &shallowest_folder.parents {
      let ids_with_parents: Vec<Vec<String>> = parents_id
        .iter()
        .map(|p| {
          let mut tmp = id_path.clone();
          tmp.push(p.to_string());
          tmp
        })
        .collect();
      ids_with_parents
        .iter()
        .map(|ids| append_parents_id(ids.to_vec(), id2meta))
        .flatten()
        .collect()
    } else {
      vec![id_path]
    }
  } else {
    vec![id_path]
  }
}

pub fn trace_id_paths(end: &files::FileMeta, id2meta: &IdToFileMeta) -> Option<Vec<IdPath>> {
  let end_id = vec![end.id.clone()?];
  let paths_to_end = append_parents_id(end_id, id2meta);
  Some(paths_to_end)
}

pub fn path_from_ids(id_path: &[String], id2meta: &IdToFileMeta) -> Option<String> {
  let names = id_path
    .iter()
    .filter(|id| id2meta.contains_key(&id.to_string()))
    .map(|id| {
      let meta = id2meta.get(id).unwrap();
      meta.name.clone().unwrap_or_else(String::new)
    });
  let path = PathBuf::from_iter(names);
  Some(path.to_str()?.to_owned())
}
