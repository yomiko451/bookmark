use serde::{Deserialize, Serialize};
use std::{
    io::{Seek, SeekFrom},
    path::PathBuf,
    sync::atomic::{AtomicI32, Ordering},
};
use tauri::AppHandle;

static ID: AtomicI32 = AtomicI32::new(1);

#[derive(Serialize, Deserialize)]
pub struct BookMark {
    id: String,
    name: String,
    page: String,
    description: String,
    created_at: String,
    updated_at: String,
}

impl BookMark {
    pub fn build(name: String, page: String, description: String) -> Self {
        let created_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let id = ID.fetch_add(1, Ordering::SeqCst);
        BookMark {
            name,
            page,
            description,
            created_at,
            updated_at: String::default(),
            id: format!("{:03}", id),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Config {
    max_id: i32,
    path: PathBuf,
    app_info: String,
}

#[tauri::command]
pub fn init(app_handle: AppHandle) -> Vec<BookMark> {
    let path = app_handle.path_resolver().app_local_data_dir().unwrap();
    let storage_directory = path.join("bookmark");
    let config_path = path.join("config.json");
    let bookmark_arr: Vec<BookMark> = if !storage_directory.exists() {
        std::fs::create_dir(storage_directory).unwrap();
        Vec::new()
    } else {
        std::fs::read_dir(storage_directory)
            .unwrap()
            .map(|entry| {
                let file = std::fs::File::open(entry.unwrap().path()).unwrap();
                serde_json::from_reader(file).unwrap()
            })
            .collect()
    };
    if !config_path.exists() {
        let file = std::fs::File::create(config_path).unwrap();
        let config = Config {
            max_id: ID.load(Ordering::SeqCst),
            path,
            app_info: String::default(),
        };
        serde_json::to_writer_pretty(file, &config).unwrap();
    } else {
        let file = std::fs::File::open(config_path).unwrap();
        let config: Config = serde_json::from_reader(file).unwrap();
        ID.fetch_add(config.max_id - 1, Ordering::SeqCst);
    }

    bookmark_arr
}

#[tauri::command]
pub fn add_bookmark(
    app_handle: AppHandle,
    name: String,
    page: String,
    description: String,
) -> BookMark {
    let new_bookmark = BookMark::build(name, page, description);
    let path = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join(format!("bookmark/{}.json", new_bookmark.id));
    let file = std::fs::File::create(path).unwrap();
    serde_json::to_writer_pretty(file, &new_bookmark).unwrap();

    new_bookmark
}

#[tauri::command]
pub fn modify_bookmark(
    app_handle: AppHandle,
    name: String,
    page: String,
    description: String,
    id: String,
) -> BookMark {
    let path = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join(format!("bookmark/{}.json", id));
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut bookmark: BookMark = serde_json::from_reader(&file).unwrap();
    bookmark.name = name;
    bookmark.page = page;
    bookmark.description = description;
    file.seek(SeekFrom::Start(0)).unwrap();
    file.set_len(0).unwrap();
    serde_json::to_writer_pretty(file, &bookmark).unwrap();

    bookmark
}

#[tauri::command]
pub fn delete_bookmark(app_handle: AppHandle, id: String) {
    std::fs::remove_file(
        app_handle
            .path_resolver()
            .app_local_data_dir()
            .unwrap()
            .join(format!("bookmark/{}.json", id)),
    )
    .unwrap();
}

#[tauri::command]
pub fn update_id(app_handle: AppHandle) {
    let path = app_handle
        .path_resolver()
        .app_local_data_dir()
        .unwrap()
        .join("config.json");
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut config: Config = serde_json::from_reader(&file).unwrap();
    config.max_id = ID.load(Ordering::SeqCst);
    file.seek(SeekFrom::Start(0)).unwrap();
    file.set_len(0).unwrap();
    serde_json::to_writer_pretty(file, &config).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmark_new() {
        let bm = BookMark::build("test".to_string(), "8".to_string(), "test".to_string());
        assert_eq!(bm.id, "001");
        assert_eq!(bm.name, "test");
        assert_eq!(bm.description, "test");
        assert_eq!(bm.updated_at, "");
    }
}
