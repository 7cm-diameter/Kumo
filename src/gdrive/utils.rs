use crate::gdrive::api::files;
use reqwest::Client;
use std::collections::HashMap;

pub async fn find_parents_id<'a>(
  client: &'a Client,
  access_token: &'a str,
  parents_name: &'a str,
) -> String {
  // Fetch folders' metadata from drive
  let mut ls_query = files::FilesListQuery::default();
  ls_query.enqueue_search_q(
    "mimeType = 'application/vnd.google-apps.folder'",
    files::Conjunction::And,
  );
  let folders = files::files_list(client, access_token, &ls_query)
    .await
    .files;

  let path_components: Vec<&str> = parents_name.split('/').collect();
  let candidate_folders: Vec<files::FileMeta> = folders
    .iter()
    .filter(|f| {
      let filename = f.name.clone().unwrap_or_else(String::new);
      let basename = path_components.last().unwrap_or(&"");
      basename == &filename.as_str()
    })
    .cloned()
    .collect();

  let mut id_to_metadata: HashMap<String, files::FileMeta> = HashMap::new();
  folders.iter().for_each(|m| {
    let id = m.id.clone().unwrap_or_else(String::new);
    id_to_metadata.insert(id, m.clone());
  });

  let ids_lead_to_candidates: Vec<Vec<String>> = candidate_folders
    .iter()
    .map(|f| {
      let ids = vec![f.id.clone().unwrap_or_else(String::new)];
      append_parents_id(ids, &id_to_metadata)
    })
    .flatten()
    .map(|mut ids| {
      ids.reverse();
      ids
    })
    .collect();

  let paths_to_candidates: Vec<String> = ids_lead_to_candidates
    .iter()
    .map(|ids| {
      let names: Vec<String> = ids
        .iter()
        .filter(|id| id_to_metadata.contains_key(&id.to_string()))
        .map(|id| {
          let meta = id_to_metadata.get(id).unwrap();
          meta.name.clone().unwrap_or_else(String::new)
        })
        .collect();
      names.join("/")
    })
    .collect();

  for (path, ids) in paths_to_candidates.iter().zip(ids_lead_to_candidates) {
    if path == parents_name {
      return ids.last().unwrap().clone();
    }
  }
  String::new()
}

// TODO: Need refactoring!
fn append_parents_id(
  ids: Vec<String>,
  id_to_meta: &HashMap<String, files::FileMeta>,
) -> Vec<Vec<String>> {
  let shallowest_folder_id = ids.last().unwrap();
  if let Some(shallowest_folder) = id_to_meta.get(shallowest_folder_id) {
    if let Some(parents_id) = &shallowest_folder.parents {
      let ids_with_parents: Vec<Vec<String>> = parents_id
        .iter()
        .map(|p| {
          let mut tmp = ids.clone();
          tmp.push(p.to_string());
          tmp
        })
        .collect();
      ids_with_parents
        .iter()
        .map(|ids| append_parents_id(ids.to_vec(), id_to_meta))
        .flatten()
        .collect()
    } else {
      vec![ids]
    }
  } else {
    vec![ids]
  }
}
