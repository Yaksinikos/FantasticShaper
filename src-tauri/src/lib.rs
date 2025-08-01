use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn fn_get_wondershaper_version() -> String {
    let v_command = Command::new("wondershaper")
        .arg("-v")
        .output()
        .expect("wondershaper not installed");

    let mut v_return_string: String = String::new();
    if v_command.status.success() {
        let v_stdout = v_command.stdout;
        let v_stringified_stdout = String::from_utf8(v_stdout);

        // Error Handling
        match v_stringified_stdout {
            Ok(s) => {
                println!("success");
                v_return_string = s;
            }
            Err(e) => {
                eprintln!("execution of command failed: {}", e);
            }
        }
    }

    v_return_string = match v_return_string.split_whitespace().last() {
        Some(v_string) => v_string.to_string(),
        None => {
            eprintln!("version parser failed");
            return String::from("unknown");
        }
    };
    return v_return_string;
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn fn_lib_cmd_fireup() {
    Command::new("konsole").spawn().expect("");
    print!("Hello World");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![fn_lib_cmd_fireup])
        .invoke_handler(tauri::generate_handler![fn_get_wondershaper_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
