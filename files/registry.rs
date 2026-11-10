//! Windows registry read helpers for Reflect Notes.

pub const APP_KEY: &str = "Software\\Reflect\\Reflect Notes";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Reflect Notes";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Reflect Notes") },
        RegValue { name: "Version",     value: String::from("v2.2.9") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Reflect Notes") },
        RegValue { name: "Publisher",       value: String::from("Reflect") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Reflect Notes\\uninstall.exe\"") },
    ]
}
