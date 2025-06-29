use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(serde::Serialize, Clone)]
struct CheckResult {
    name: String,
    installed: bool,
    version: String,
}

#[tauri::command]
fn doctor_check() -> Vec<CheckResult> {
    let mut results = Vec::new();

    // Check for Git
    let git_output = Command::new("git").arg("--version").output();
    results.push(CheckResult {
        name: "Git".to_string(),
        installed: git_output.is_ok(),
        version: git_output.map_or("Not found".to_string(), |o| String::from_utf8_lossy(&o.stdout).trim().to_string()),
    });

    // Check for Node.js
    let node_output = Command::new("node").arg("--version").output();
    results.push(CheckResult {
        name: "Node.js".to_string(),
        installed: node_output.is_ok(),
        version: node_output.map_or("Not found".to_string(), |o| String::from_utf8_lossy(&o.stdout).trim().to_string()),
    });

    // Check for Android SDK (adb)
    let adb_output = Command::new("adb").arg("--version").output();
    results.push(CheckResult {
        name: "Android SDK".to_string(),
        installed: adb_output.is_ok(),
        version: adb_output.map_or("Not found".to_string(), |o| String::from_utf8_lossy(&o.stdout).trim().to_string()),
    });

    // Check for ios-deploy
    let ios_deploy_output = Command::new("ios-deploy").arg("--version").output();
    results.push(CheckResult {
        name: "ios-deploy".to_string(),
        installed: ios_deploy_output.is_ok(),
        version: ios_deploy_output.map_or("Not found".to_string(), |o| String::from_utf8_lossy(&o.stdout).trim().to_string()),
    });

    results
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, doctor_check])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
