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
