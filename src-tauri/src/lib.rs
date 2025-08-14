use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[tauri::command]
async fn fn_get_wondershaper_version() -> String {
    let v_command = Command::new("sh")
        .arg("-c")
        .arg("wondershaper -v")
        .output()
        .expect("wondershaper not installed");

    let mut v_return_string: String = String::new();
    if v_command.status.success() {
        let v_stdout = v_command.stdout;
        let v_stringified_stdout = String::from_utf8(v_stdout);

        // Error Handling
        match v_stringified_stdout {
            Ok(s) => {
                println!("success parsed version");
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
async fn fn_get_active_interface() -> String {
    let v_command = Command::new("sh")
        .arg("-c")
        .arg("nmcli -t -f DEVICE,STATE device status | grep 'connected' | head -n1 | cut -d: -f1")
        .output()
        .expect("ip not installed");

    let mut v_return_string: String = String::new();
    if v_command.status.success() {
        let v_stringified_stdout = String::from_utf8(v_command.stdout);

        // Error Handling
        match v_stringified_stdout {
            Ok(s) => {
                println!("success parsed interface");
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
            eprintln!("interface parser failed");
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
        .invoke_handler(tauri::generate_handler![
            fn_get_wondershaper_version,
            fn_get_active_interface,
            fn_lib_cmd_fireup,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
