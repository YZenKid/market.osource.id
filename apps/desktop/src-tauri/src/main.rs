#[tauri::command]
fn desktop_runtime_status() -> serde_json::Value {
    serde_json::json!({
        "postgres": {
            "bundled": true,
            "bind": "127.0.0.1",
            "running": false,
            "note": "sidecar supervision is wired in later phases"
        },
        "backend": {
            "running": false,
            "note": "Axum process supervision is wired in later phases"
        },
        "tunnel": {
            "enabled": false,
            "running": false,
            "note": "cloudflared is opt-in"
        }
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![desktop_runtime_status])
        .run(tauri::generate_context!())
        .expect("failed to run market.osource.id desktop shell");
}
