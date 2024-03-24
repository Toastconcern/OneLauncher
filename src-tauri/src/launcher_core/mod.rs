use launcher_core::game::client::ClientManager;
use tauri::{plugin::TauriPlugin, Manager, State};
use tokio::sync::Mutex;

mod auth;
mod game;

pub struct GameManagerState {
    pub client_manager: ClientManager,
}

unsafe impl Send for GameManagerState {}
unsafe impl Sync for GameManagerState {}

pub fn init() -> TauriPlugin<tauri::Wry> {
	tauri::plugin::Builder::new("launcher-core")
		.setup(|app, _| {
			app.manage(Mutex::new(GameManagerState {
                client_manager: ClientManager::new().expect("Failed to initialize client manager"),
            }));
            
			Ok(())
		})
		.invoke_handler(tauri::generate_handler![
            auth::login_msa,
            game::create_instance,
            game::get_instances,
            game::get_instance,
            game::get_manifest,
            refresh_client_manager,
        ])
		.build()
}

#[tauri::command]
async fn refresh_client_manager(
    state: State<'_, Mutex<GameManagerState>>
) -> Result<(), String> {
    let mut state = state.lock().await;
    state.client_manager = ClientManager::new()?;
    Ok(())
}