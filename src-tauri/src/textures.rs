use serde::{Deserialize, Serialize};
use std::{
  fs,
  io::{self, Cursor, Read},
  path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TexturePack {
  author: String,
  description: String,
  version: String,
  path: Option<PathBuf>,
}

#[tauri::command]
pub async fn extract_textures(app_handle: tauri::AppHandle, textures_array: Vec<String>) {
  let text_dir = app_handle
    .path_resolver()
    .app_dir()
    .unwrap()
    .join("data/texture_replacements");

  let target_dir = PathBuf::from(text_dir.clone()); // Doesn't need to exist

  // for path in textures_array {
  //     println!("Extracting texture pack: {:?}", path.clone());

  //     let archive: Vec<u8> = fs::read(&path.clone()).unwrap();
  //     // The third parameter allows you to strip away toplevel directories.
  //     // If `archive` contained a single directory, its contents would be extracted instead.
  //     match zip_extract::extract(Cursor::new(archive), &target_dir, true) {
  //         Ok(_) => continue,
  //         Err(err) => println!("{:?}", err),
  //     }
  // }
}

fn read_texture_json_file(file_path: PathBuf) -> Result<TexturePack, io::Error> {
  let zipfile = std::fs::File::open(&file_path)?;
  let mut zip = zip::ZipArchive::new(zipfile).unwrap();

  // TODO: Figure out some top level schenanigans here similar to the zip extract ignoring toplevel
  let mut contents = String::new();
  zip
    .by_name("texture_replacements/about.json")?
    .read_to_string(&mut contents)?;

  let pack: TexturePack = TexturePack {
    path: Some(file_path),
    ..serde_json::from_str(&contents).unwrap()
  };
  Ok(pack)
}

#[tauri::command]
pub fn get_all_texture_packs(dir: String) -> Vec<TexturePack> {
  let dir_path = Path::new(&dir).exists();
  if !dir_path {
    println!("Textures directory doesn't exist, creating it now.");
    fs::create_dir(dir.clone()).unwrap();
    return Vec::new();
  }

  let entries = fs::read_dir(dir).unwrap();

  let mut texture_pack_data: Vec<TexturePack> = Vec::new();
  for entry in entries {
    let path = entry.unwrap().path();
    match path.extension() {
      Some(ext) if ext == "zip" => {
        let files = match read_texture_json_file(path.clone()) {
          Ok(pack) => pack,
          Err(_e) => {
            // if the about.json file isn't inside of the expected directory this error happens
            // TODO: add this error to a logs file so players know when they install a bad texture pack
            println!("File doesn't have proper about.json: {:?}", path);
            continue;
          }
        };
        texture_pack_data.push(files);
      }
      _ => continue,
    }
  }
  return texture_pack_data;
}
