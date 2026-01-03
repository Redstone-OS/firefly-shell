//! # App Discovery
//!
//! Descoberta de aplicativos no sistema via cache.
//!
//! ## Formato do Cache
//!
//! O cache fica em `/state/indexes/apps/apps.cache` e usa formato texto simples:
//!
//! ```text
//! # Comentários começam com #
//! vendor|name|display_name|icon_path|category
//! system|terminal|Terminal|/apps/system/terminal/assets/terminal.svg|system
//! ```
//!
//! ## Estrutura de Apps (referência para geração do cache)
//!
//! ```text
//! /apps/<vendor>/<name>/
//! ├── <name>.app         # Executável
//! ├── app.toml           # Metadados
//! └── assets/
//!     └── <icon>.svg     # Ícone
//! ```

#![allow(dead_code)]
#![allow(unused_imports)]

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use redpowder::fs::File;

// =============================================================================
// CONSTANTES
// =============================================================================

/// Caminho do cache de apps.
const APPS_CACHE_PATH: &str = "/state/indexes/apps/apps.cache";

/// Diretório raiz de apps (para construir paths).
const APPS_ROOT: &str = "/apps";

/// Tamanho máximo do buffer de leitura do cache.
const CACHE_BUFFER_SIZE: usize = 2048;

// =============================================================================
// APP INFO
// =============================================================================

/// Informações de um aplicativo.
#[derive(Debug, Clone)]
pub struct AppInfo {
    /// ID único (vendor.name).
    pub id: String,
    /// Nome de exibição.
    pub name: String,
    /// Vendor/publisher.
    pub vendor: String,
    /// Caminho do executável.
    pub path: String,
    /// Caminho do ícone.
    pub icon_path: Option<String>,
    /// Categoria.
    pub category: String,
}

impl AppInfo {
    /// Cria AppInfo a partir de uma linha do cache.
    ///
    /// Formato: vendor|name|display_name|icon_path|category
    fn from_cache_line(line: &str) -> Option<Self> {
        let line = line.trim();

        // Ignorar linhas vazias e comentários
        if line.is_empty() || line.starts_with('#') {
            return None;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() < 5 {
            redpowder::println!("[Discovery] WARN: Linha invalida no cache: {}", line);
            return None;
        }

        let vendor = parts[0].trim();
        let name = parts[1].trim();
        let display_name = parts[2].trim();
        let icon_path = parts[3].trim();
        let category = parts[4].trim();

        // Construir caminho do executável
        let exec_path = alloc::format!("{}/{}/{}/{}.app", APPS_ROOT, vendor, name, name);

        // Icon path pode ser vazio
        let icon = if icon_path.is_empty() {
            None
        } else {
            Some(icon_path.to_string())
        };

        Some(Self {
            id: alloc::format!("{}.{}", vendor, name),
            name: display_name.to_string(),
            vendor: vendor.to_string(),
            path: exec_path,
            icon_path: icon,
            category: category.to_string(),
        })
    }
}

/// Ícone de app (placeholder por enquanto).
#[derive(Debug, Clone)]
pub struct AppIcon {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u32>,
}

impl Default for AppIcon {
    fn default() -> Self {
        Self {
            width: 32,
            height: 32,
            data: Vec::new(),
        }
    }
}

// =============================================================================
// DISCOVERY VIA CACHE
// =============================================================================

/// Descobre todos os apps instalados lendo do cache.
///
/// O cache está em `/state/indexes/apps/apps.cache` e evita múltiplas
/// syscalls de listagem de diretório que podem causar instabilidade.
///
/// Se o cache não existir ou estiver vazio, retorna vetor vazio.
pub fn discover_apps() -> Vec<AppInfo> {
    redpowder::println!("[Discovery] Lendo cache de apps...");

    let mut apps = Vec::new();

    // Tentar abrir o arquivo de cache
    let file = match File::open(APPS_CACHE_PATH) {
        Ok(f) => f,
        Err(e) => {
            redpowder::println!("[Discovery] WARN: Cache nao encontrado: {:?}", e);
            redpowder::println!("[Discovery] Crie manualmente: {}", APPS_CACHE_PATH);
            return apps;
        }
    };

    // Ler conteúdo do cache
    let mut buffer = [0u8; CACHE_BUFFER_SIZE];
    let bytes_read = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            redpowder::println!("[Discovery] ERRO: Falha ao ler cache: {:?}", e);
            return apps;
        }
    };

    if bytes_read == 0 {
        redpowder::println!("[Discovery] Cache vazio");
        return apps;
    }

    // Converter para string
    let content = match core::str::from_utf8(&buffer[..bytes_read]) {
        Ok(s) => s,
        Err(_) => {
            redpowder::println!("[Discovery] ERRO: Cache nao e UTF-8 valido");
            return apps;
        }
    };

    // Processar cada linha
    for line in content.lines() {
        if let Some(app_info) = AppInfo::from_cache_line(line) {
            redpowder::println!("[Discovery] App: {} ({})", app_info.name, app_info.id);
            apps.push(app_info);
        }
    }

    redpowder::println!("[Discovery] {} apps encontrados", apps.len());
    apps
}

// =============================================================================
// LEGACY - DESCOBERTA DINÂMICA (DESABILITADA)
// =============================================================================

/// Descoberta dinâmica de apps (desabilitada por instabilidade).
///
/// Este código é mantido para referência. A descoberta dinâmica causa
/// kernel panic devido a:
/// - Múltiplas syscalls de listagem de diretório
/// - Alocações recursivas de memória
/// - Stack overflow no processo do shell
///
/// Usar `discover_apps()` que lê do cache é a solução estável.
#[allow(dead_code)]
mod legacy {
    use super::*;
    use redpowder::fs::list_dir;

    const APPS_ROOT_LEGACY: &str = "/apps";
    const APP_MANIFEST: &str = "app.toml";

    /// Descobre apps dinamicamente (INSTÁVEL - NÃO USAR).
    pub fn discover_apps_dynamic() -> Vec<AppInfo> {
        redpowder::println!("[Discovery] WARN: Descoberta dinamica desabilitada");
        redpowder::println!("[Discovery] Causa: instabilidade no filesystem/memoria");
        Vec::new()

        // O código original está comentado abaixo para referência:
        /*
        let mut apps = Vec::new();

        redpowder::println!("[Discovery] Buscando apps em {}", APPS_ROOT_LEGACY);

        // Listar vendors
        if let Ok(vendors) = list_dir(APPS_ROOT_LEGACY) {
            for vendor_entry in vendors {
                if !vendor_entry.is_dir() {
                    continue;
                }

                let vendor_name = vendor_entry.name();

                // Ignorar . e ..
                if vendor_name == "." || vendor_name == ".." {
                    continue;
                }

                let vendor_path = alloc::format!("{}/{}", APPS_ROOT_LEGACY, vendor_name);

                redpowder::println!("[Discovery] Vendor: {}", vendor_name);

                // Listar apps do vendor
                if let Ok(app_dirs) = list_dir(&vendor_path) {
                    for app_entry in app_dirs {
                        if !app_entry.is_dir() {
                            continue;
                        }

                        let app_name = app_entry.name();

                        // Ignorar . e ..
                        if app_name == "." || app_name == ".." {
                            continue;
                        }

                        // Criar AppInfo básico
                        let base_path = alloc::format!("{}/{}", vendor_path, app_name);
                        let exec_path = alloc::format!("{}/{}.app", base_path, app_name);

                        apps.push(AppInfo {
                            id: alloc::format!("{}.{}", vendor_name, app_name),
                            name: app_name.to_string(),
                            vendor: vendor_name.to_string(),
                            path: exec_path,
                            icon_path: None,
                            category: "other".to_string(),
                        });
                    }
                }
            }
        } else {
            redpowder::println!("[Discovery] Falha ao ler {}", APPS_ROOT_LEGACY);
        }

        redpowder::println!("[Discovery] {} apps encontrados", apps.len());
        apps
        */
    }
}
