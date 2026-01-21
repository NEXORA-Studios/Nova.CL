use crate::app::state::{AppState, Running};

#[tauri::command]
async fn http_get(
    state: tauri::State<'_, AppState<Running>>,
    url: String,
) -> Result<String, String> {
    state.http.get(&url).await.map_err(|e| e.to_string())
}
