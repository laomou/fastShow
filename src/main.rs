slint::include_modules!();
use libloading::Library;
use libloading::Symbol;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Mutex;
mod plugin;
use plugin::ImagePlugin;

lazy_static::lazy_static! {
    static ref PLUGINS: Mutex<Vec<Box<dyn ImagePlugin + Send + Sync>>> = Mutex::new(Vec::new());
}

fn load_plugins() {
    let plugins_dir = Path::new("plugins");
    if !plugins_dir.exists() {
        fs::create_dir(plugins_dir).unwrap();
        return;
    }

    for entry in fs::read_dir(plugins_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path
            .extension()
            .map_or(false, |ext| ext == "so" || ext == "dll" || ext == "dylib")
        {
            unsafe {
                if let Ok(lib) = Library::new(&path) {
                    if let Ok(constructor) = lib.get::<Symbol<
                        unsafe extern "Rust" fn() -> Box<dyn ImagePlugin + Send + Sync>,
                    >>(b"new_plugin")
                    {
                        let plugin = constructor();
                        PLUGINS.lock().unwrap().push(plugin);
                    }
                }
            }
        }
    }
}

fn supported_extensions() -> Vec<String> {
    PLUGINS
        .lock()
        .unwrap()
        .iter()
        .flat_map(|p| p.supported_extensions())
        .collect()
}

fn main() -> Result<(), slint::PlatformError> {
    load_plugins();

    let main_window = MainWindow::new()?;
    let weak_window = main_window.as_weak();

    let current_dir = Mutex::new(PathBuf::from("."));

    fn refresh_files(window: slint::Weak<MainWindow>, dir: &Path) {
        let mut files = Vec::new();
        let extensions = supported_extensions();

        if let Some(parent) = dir.parent() {
            files.push(FileItem {
            });
        }

        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let is_dir = path.is_dir();
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_lowercase())
                    .unwrap_or_default();

                if is_dir || extensions.iter().any(|e| e.to_lowercase() == ext) {
                    files.push(FileItem {
                    });
                }
            }
        }

        // window.unwrap().set_files(files.into());
    }

    refresh_files(weak_window.clone(), &current_dir.lock().unwrap());

    main_window.run()
}
