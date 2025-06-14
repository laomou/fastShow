use std::path::PathBuf;
use crate::FileEntry;

#[derive(Clone)]
pub struct FileManager {
    current_path: PathBuf,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            current_path: std::env::current_dir().unwrap(),
        }
    }

    pub fn get_current_path(&self) -> String {
        self.current_path.to_string_lossy().into_owned()
    }

    pub fn get_tree_items(&self) -> Vec<FileEntry> {
        let mut items = Vec::new();

        // 添加磁盘驱动器
        #[cfg(target_os = "windows")]
        {
            for drive in ('A'..='Z').map(|c| format!("{}:", c)) {
                let path = PathBuf::from(&drive);
                if path.exists() {
                    items.push(FileEntry {
                        name: format!("磁盘 {}", drive).into(),
                        is_directory: true,
                        path: drive.into(),
                    });
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            items.push(FileEntry {
                name: "此电脑".into(),
                is_directory: true,
                path: "/".into(),
            });
        }

        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn get_contents(&self) -> Vec<FileEntry> {
        let mut contents = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(metadata) = entry.metadata() {
                    let is_dir = metadata.is_dir();
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .into_owned();

                    contents.push(FileEntry {
                        name: name.into(),
                        is_directory: is_dir,
                        path: path.to_string_lossy().into_owned().into(),
                    });
                }
            }
        }

        // 排序：目录在前，文件在后
        contents.sort_by(|a, b| {
            if a.is_directory && !b.is_directory {
                std::cmp::Ordering::Less
            } else if !a.is_directory && b.is_directory {
                std::cmp::Ordering::Greater
            } else {
                a.name.cmp(&b.name)
            }
        });

        contents
    }

    pub fn navigate(&mut self, path: &str) {
        let new_path = PathBuf::from(path);
        if new_path.is_dir() {
            self.current_path = new_path;
        }
    }
}
