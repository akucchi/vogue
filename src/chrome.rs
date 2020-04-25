//! ## Vogue's chrome backend.
//!
//! It just finds and launches Chrome in a tabless window.

use std::process::Command;

/// Struct containing path to Chrome and method for opening tabless window.
///
/// Example:
/// ```
/// use vogue::chrome::Chrome;
/// Chrome::new()
///     .open("https://example.com")
///     .unwrap();
/// ```
#[derive(Default, Debug, Clone)]
pub struct Chrome {
    path: Option<String>,
}

impl Chrome {
    /// Return `Chrome` object with found path.
    /// If cannot locate Chrome executable, `path` field is set to `None`.
    pub fn new() -> Self {
        Self { path: find_path() }
    }

    /// Uses Chrome path and opens window without tabs with specified URL.
    pub fn open(&mut self, url: &str) -> Result<(), &str> {
        // If Chrome path is not set, we can't spawn it.
        if self.path.is_none() {
            return Err("Chrome cannot be located");
        }

        // Spawn Chrome.
        let p = Command::new(self.path.as_ref().unwrap())
            .arg(format!("--app={}", url))
            .arg("--new-window")
            .spawn();

        // Handle error.
        if p.is_err() {
            Err("Cannot open Chrome")
        } else {
            Ok(())
        }
    }
}

const REG_PATH: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\chrome.exe";

/// Find chrome executable on Windows.
fn find_path_windows() -> Option<String> {
    use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    use winreg::RegKey;

    let mut path = None;

    for install_type in vec![HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE] {
        let reg_key = RegKey::predef(install_type).open_subkey(REG_PATH);

        if reg_key.is_err() {
            // If REG_PATH doesn't exists in HKEY_CURRENT_USER,
            // try looking for it in HKEY_LOCAL_MACHINE.
            continue;
        }

        // Get `Result<..., ...>` value if its not `Err(...)`.
        let reg_key = reg_key.unwrap();

        // Get `Path`, if for some reason, REG_PATH is found,
        // but `Path` not, we can continue and path will be still set to None.
        match reg_key.get_value::<String, &str>("Path") {
            Ok(p) => path = Some(format!("{}\\chrome.exe", p)),
            Err(_) => continue,
        }
    }

    path
}

/// Cross-platform function for finding Chrome's path.
pub fn find_path() -> Option<String> {
    if cfg!(windows) {
        find_path_windows()
    } else {
        None
    }
}
