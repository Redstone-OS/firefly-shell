//! # App Launcher
//!
//! Lançamento de aplicativos.

use redpowder::process;

/// Lança um aplicativo pelo caminho.
pub fn launch_app(path: &str) -> bool {
    redpowder::println!("[Launcher] Iniciando: {}", path);

    // spawn requer path e args
    let args: &[&str] = &[];

    match process::spawn(path, args) {
        Ok(pid) => {
            redpowder::println!("[Launcher] App iniciado (PID {})", pid);
            true
        }
        Err(e) => {
            redpowder::println!("[Launcher] Erro ao iniciar {}: {:?}", path, e);
            false
        }
    }
}

// TODO: Revisar no futuro
#[allow(unused)]
/// Lança app por AppInfo.
pub fn launch_app_info(app: &super::AppInfo) -> bool {
    launch_app(&app.path)
}
