//! Portable mode: keeps all launcher data next to the executable, so the app
//! can run from a removable drive without touching the user's app data.
//!
//! Portable mode is enabled by placing a `portable.txt` file beside the
//! executable, or by setting the `MODRINTH_APP_PORTABLE` environment variable.
//! Settings, instances, caches and logs then live in `data/` beside the
//! executable, and the WebView2 profile in `data/webview2/`.

use std::path::PathBuf;

/// File next to the executable that enables portable mode.
pub const MARKER_FILE: &str = "portable.txt";

/// Directory beside the executable that holds all portable app data.
pub const DATA_DIR: &str = "data";

/// Enables portable storage when portable mode is active, returning the data
/// directory that is in use.
pub fn init() -> Option<PathBuf> {
    let data_dir = portable_data_dir()?;

    if let Err(error) = std::fs::create_dir_all(&data_dir) {
        eprintln!(
            "Could not create portable data directory at {}: {error}",
            data_dir.display()
        );
        return None;
    }

    // SAFETY: called at the start of `main`, before the runtime is started and
    // before any other thread can read the environment.
    unsafe {
        std::env::set_var("THESEUS_CONFIG_DIR", &data_dir);
        std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", data_dir.join("webview2"));
    }

    Some(data_dir)
}

fn portable_data_dir() -> Option<PathBuf> {
    let exe_dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
    let enabled = std::env::var_os("MODRINTH_APP_PORTABLE").is_some()
        || exe_dir.join(MARKER_FILE).is_file();

    enabled.then(|| exe_dir.join(DATA_DIR))
}
