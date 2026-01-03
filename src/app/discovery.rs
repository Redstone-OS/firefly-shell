//! # App Discovery
//!
//! Descoberta de aplicativos no sistema.
//!
//! ## Estrutura de Apps
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
use redpowder::fs::{list_dir, File};

// =============================================================================
// CONSTANTES
// =============================================================================

/// Diretório raiz de apps.
const APPS_ROOT: &str = "/apps";

/// Nome do arquivo de manifesto.
const APP_MANIFEST: &str = "app.toml";

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
    /// Cria AppInfo a partir de um diretório.
    fn from_directory(vendor: &str, app_dir: &str) -> Option<Self> {
        let base_path = alloc::format!("{}/{}/{}", APPS_ROOT, vendor, app_dir);

        // Procurar arquivo .app
        let app_file = Self::find_app_file(&base_path)?;

        // Tentar ler app.toml
        let manifest_path = alloc::format!("{}/{}", base_path, APP_MANIFEST);
        let (name, icon_path, category) = Self::parse_manifest(&manifest_path, &base_path, app_dir);

        Some(Self {
            id: alloc::format!("{}.{}", vendor, app_dir),
            name,
            vendor: vendor.to_string(),
            path: alloc::format!("{}/{}", base_path, app_file),
            icon_path,
            category,
        })
    }

    /// Encontra arquivo .app no diretório.
    fn find_app_file(base_path: &str) -> Option<String> {
        if let Ok(entries) = list_dir(base_path) {
            for entry in entries {
                let name = entry.name();
                if name.ends_with(".app") {
                    return Some(name.to_string());
                }
            }
        }
        None
    }

    /// Parse do app.toml.
    fn parse_manifest(
        manifest_path: &str,
        base_path: &str,
        fallback_name: &str,
    ) -> (String, Option<String>, String) {
        let mut name = fallback_name.to_string();
        let mut icon_path: Option<String> = None;
        let mut category = "other".to_string();

        if let Ok(file) = File::open(manifest_path) {
            // Ler conteúdo
            let mut buf = [0u8; 1024];
            if let Ok(n) = file.read(&mut buf) {
                if let Ok(content) = core::str::from_utf8(&buf[..n]) {
                    // Parser TOML simples
                    for line in content.lines() {
                        let line = line.trim();

                        if line.starts_with("name") {
                            if let Some(value) = Self::extract_string_value(line) {
                                name = value;
                            }
                        } else if line.starts_with("icon") {
                            if let Some(value) = Self::extract_string_value(line) {
                                icon_path = Some(alloc::format!("{}/assets/{}", base_path, value));
                            }
                        } else if line.starts_with("category") {
                            if let Some(value) = Self::extract_string_value(line) {
                                category = value;
                            }
                        }
                    }
                }
            }
        }

        (name, icon_path, category)
    }

    /// Extrai valor de string de uma linha TOML.
    fn extract_string_value(line: &str) -> Option<String> {
        if let Some(eq_pos) = line.find('=') {
            let value = line[eq_pos + 1..].trim();
            if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
                return Some(value[1..value.len() - 1].to_string());
            }
        }
        None
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
// DISCOVERY
// =============================================================================

/// Descobre todos os apps instalados.
///
/// TODO: Re-habilitar quando o filesystem estiver mais estável.
/// Por enquanto retorna vetor vazio para evitar crash durante boot.
pub fn discover_apps() -> Vec<AppInfo> {
    redpowder::println!("[Discovery] Descoberta de apps desabilitada temporariamente");
    Vec::new()

    /* ORIGINAL - reabilitar quando filesystem estiver estável:
    let mut apps = Vec::new();

    redpowder::println!("[Discovery] Buscando apps em {}", APPS_ROOT);

    // Listar vendors
    if let Ok(vendors) = list_dir(APPS_ROOT) {
        for vendor_entry in vendors {
            if !vendor_entry.is_dir() {
                continue;
            }

            let vendor_name = vendor_entry.name();

            // Ignorar . e ..
            if vendor_name == "." || vendor_name == ".." {
                continue;
            }

            let vendor_path = alloc::format!("{}/{}", APPS_ROOT, vendor_name);

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

                    if let Some(app_info) = AppInfo::from_directory(vendor_name, app_name) {
                        redpowder::println!(
                            "[Discovery]   App: {} ({})",
                            app_info.name,
                            app_info.path
                        );
                        apps.push(app_info);
                    }
                }
            }
        }
    } else {
        redpowder::println!("[Discovery] Falha ao ler {}", APPS_ROOT);
    }

    redpowder::println!("[Discovery] {} apps encontrados", apps.len());
    apps
    */
}
