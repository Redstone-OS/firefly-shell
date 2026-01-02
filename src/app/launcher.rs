//! # App Launcher
//!
//! Módulo para lançar aplicativos.

use alloc::string::String;
use redpowder::syscall::{check_error, syscall2};
use redpowder::syscall::{SysResult, SYS_SPAWN};

/// Lança um aplicativo pelo caminho
pub fn launch_app(path: &str) -> SysResult<u32> {
    redpowder::println!("[Shell] Lançando: {}", path);

    let ret = syscall2(SYS_SPAWN, path.as_ptr() as usize, path.len());

    match check_error(ret) {
        Ok(pid) => {
            redpowder::println!("[Shell] App iniciado com PID {}", pid);
            Ok(pid as u32)
        }
        Err(e) => {
            redpowder::println!("[Shell] Erro ao lançar {}: {:?}", path, e);
            Err(e)
        }
    }
}

/// Informações de um processo em execução
pub struct RunningApp {
    pub pid: u32,
    pub name: String,
    pub path: String,
}
