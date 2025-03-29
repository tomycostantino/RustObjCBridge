use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use serde_json::{Value, from_str};

#[link(name = "NSWorkspaceWrapper", kind = "static")]
unsafe extern "C" {
    fn openFile(filePath: *const c_char) -> bool;
    fn openURL(urlString: *const c_char) -> bool;
    fn launchApplication(bundleIdentifier: *const c_char) -> bool;
    fn getApplicationPath(bundleIdentifier: *const c_char) -> *const c_char;
    fn getRunningApplications() -> *const c_char;
    fn hideApplication(bundleIdentifier: *const c_char) -> bool;
    fn unhideApplication(bundleIdentifier: *const c_char) -> bool;
    fn getFrontmostApplication() -> *const c_char;
    fn freeString(str: *const c_char);
}

/// Safe wrapper around NSWorkspace functionality
pub struct NSWorkspace;

impl NSWorkspace {
    /// Creates a new NSWorkspace instance
    pub fn new() -> Self {
        NSWorkspace
    }

    /// Opens a file with the default application
    pub fn open_file<P: AsRef<str>>(&self, file_path: P) -> Result<bool, String> {
        let c_path = match CString::new(file_path.as_ref()) {
            Ok(p) => p,
            Err(_) => return Err("Invalid file path containing null bytes".to_string()),
        };

        let result = unsafe { openFile(c_path.as_ptr()) };
        Ok(result)
    }

    /// Opens a URL with the default browser
    pub fn open_url<U: AsRef<str>>(&self, url: U) -> Result<bool, String> {
        let c_url = match CString::new(url.as_ref()) {
            Ok(u) => u,
            Err(_) => return Err("Invalid URL containing null bytes".to_string()),
        };

        let result = unsafe { openURL(c_url.as_ptr()) };
        Ok(result)
    }

    /// Launches an application by bundle identifier
    pub fn launch_application<B: AsRef<str>>(&self, bundle_id: B) -> Result<bool, String> {
        let c_bundle_id = match CString::new(bundle_id.as_ref()) {
            Ok(b) => b,
            Err(_) => return Err("Invalid bundle identifier containing null bytes".to_string()),
        };

        let result = unsafe { launchApplication(c_bundle_id.as_ptr()) };
        Ok(result)
    }

    /// Gets the path of an application by bundle identifier
    pub fn get_application_path<B: AsRef<str>>(&self, bundle_id: B) -> Result<Option<String>, String> {
        let c_bundle_id = match CString::new(bundle_id.as_ref()) {
            Ok(b) => b,
            Err(_) => return Err("Invalid bundle identifier containing null bytes".to_string()),
        };

        let path_ptr = unsafe { getApplicationPath(c_bundle_id.as_ptr()) };

        if path_ptr.is_null() {
            return Ok(None);
        }

        let path_str = unsafe {
            let result = CStr::from_ptr(path_ptr).to_string_lossy().into_owned();
            freeString(path_ptr);
            result
        };

        Ok(Some(path_str))
    }

    /// Gets information about all running applications
    pub fn get_running_applications(&self) -> Result<Vec<RunningApp>, String> {
        let json_ptr = unsafe { getRunningApplications() };

        if json_ptr.is_null() {
            return Err("Failed to get running applications".to_string());
        }

        let json_str = unsafe {
            let result = CStr::from_ptr(json_ptr).to_string_lossy().into_owned();
            freeString(json_ptr);
            result
        };

        let json_value: Value = match from_str(&json_str) {
            Ok(v) => v,
            Err(_) => return Err("Failed to parse running applications JSON".to_string()),
        };

        let apps = json_value.as_array()
            .ok_or("Invalid JSON structure for running applications".to_string())?
            .iter()
            .filter_map(|app| {
                let bundle_id = app.get("bundleIdentifier")?.as_str()?;
                let name = app.get("localizedName")?.as_str()?;
                let executable_path = app.get("executableURL")?.as_str()?;

                Some(RunningApp {
                    bundle_identifier: bundle_id.to_string(),
                    localized_name: name.to_string(),
                    executable_path: executable_path.to_string(),
                })
            })
            .collect();

        Ok(apps)
    }

    /// Hides an application by bundle identifier
    pub fn hide_application<B: AsRef<str>>(&self, bundle_id: B) -> Result<bool, String> {
        let c_bundle_id = match CString::new(bundle_id.as_ref()) {
            Ok(b) => b,
            Err(_) => return Err("Invalid bundle identifier containing null bytes".to_string()),
        };

        let result = unsafe { hideApplication(c_bundle_id.as_ptr()) };
        Ok(result)
    }

    /// Unhides an application by bundle identifier
    pub fn unhide_application<B: AsRef<str>>(&self, bundle_id: B) -> Result<bool, String> {
        let c_bundle_id = match CString::new(bundle_id.as_ref()) {
            Ok(b) => b,
            Err(_) => return Err("Invalid bundle identifier containing null bytes".to_string()),
        };

        let result = unsafe { unhideApplication(c_bundle_id.as_ptr()) };
        Ok(result)
    }
    pub fn get_frontmost_application(&self) -> Result<Option<FrontmostApp>, String> {
        let json_ptr = unsafe { getFrontmostApplication() };

        if json_ptr.is_null() {
            return Ok(None);
        }

        let json_str = unsafe {
            let result = CStr::from_ptr(json_ptr).to_string_lossy().into_owned();
            freeString(json_ptr);
            result
        };

        let json_value: Value = match from_str(&json_str) {
            Ok(v) => v,
            Err(_) => return Err("Failed to parse frontmost application JSON".to_string()),
        };

        let app = FrontmostApp {
            bundle_identifier: json_value.get("bundleIdentifier")
                .and_then(|v| v.as_str())
                .unwrap_or("").to_string(),
            localized_name: json_value.get("localizedName")
                .and_then(|v| v.as_str())
                .unwrap_or("").to_string(),
            executable_path: json_value.get("executableURL")
                .and_then(|v| v.as_str())
                .unwrap_or("").to_string(),
            process_id: json_value.get("processIdentifier")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32,
            launch_date: json_value.get("launchDate")
                .and_then(|v| v.as_str())
                .unwrap_or("").to_string(),
        };

        Ok(Some(app))
    }
}

/// Structure representing a running application
#[derive(Debug, Clone)]
pub struct RunningApp {
    pub bundle_identifier: String,
    pub localized_name: String,
    pub executable_path: String,
}

/// Structure representing the frontmost (active) application
#[derive(Debug, Clone)]
pub struct FrontmostApp {
    pub bundle_identifier: String,
    pub localized_name: String,
    pub executable_path: String,
    pub process_id: u32,
    pub launch_date: String,
}