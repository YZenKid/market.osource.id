#[tauri::command]
fn desktop_runtime_status() -> serde_json::Value {
    let postgres = sidecar_postgres::default_desktop_status();
    let tunnel = sidecar_cloudflared::default_tunnel_status();

    serde_json::json!({
        "postgres": postgres,
        "backend": {
            "running": false,
            "bind": "127.0.0.1",
            "note": "Axum process supervision is wired in a later Gate C slice"
        },
        "tunnel": tunnel,
        "note": "desktop status is read-only; no sidecar process is started by this command"
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![desktop_runtime_status])
        .run(tauri::generate_context!())
        .expect("failed to run market.osource.id desktop shell");
}
