//! # App Discovery
//!
//! Módulo para descobrir aplicativos disponíveis no sistema.
//!
//! Os aplicativos são buscados em `/apps/` recursivamente.

use alloc::string::String;
use alloc::vec::Vec;
use redpowder::fs::{exists, is_dir, Dir};
use redpowder::println;

/// Representação de um aplicativo descoberto
#[derive(Clone)]
pub struct AppInfo {
    /// Nome de exibição do app
    pub name: String,
    /// Caminho completo do executável
    pub path: String,
    /// Ícone (placeholder por enquanto)
    pub icon: AppIcon,
    /// Categoria (system, user, game, etc)
    pub category: AppCategory,
}

/// Ícone do aplicativo
#[derive(Clone, Copy, PartialEq)]
pub enum AppIcon {
    Terminal,
    Settings,
    FileManager,
    Game,
    Editor,
    Browser,
    Generic,
}

/// Categoria do aplicativo
#[derive(Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum AppCategory {
    System,
    Utility,
    Game,
    Other,
}

impl AppInfo {
    /// Cria novo AppInfo
    pub fn new(name: &str, path: &str) -> Self {
        let name_lower = name.to_lowercase();

        // Detectar ícone baseado no nome
        let icon = if name_lower.contains("terminal") {
            AppIcon::Terminal
        } else if name_lower.contains("settings") || name_lower.contains("config") {
            AppIcon::Settings
        } else if name_lower.contains("files") || name_lower.contains("filemanager") {
            AppIcon::FileManager
        } else if name_lower.contains("game") {
            AppIcon::Game
        } else if name_lower.contains("editor") || name_lower.contains("notepad") {
            AppIcon::Editor
        } else if name_lower.contains("browser") || name_lower.contains("web") {
            AppIcon::Browser
        } else {
            AppIcon::Generic
        };

        // Detectar categoria pelo path
        let category = if path.contains("/system/") {
            AppCategory::System
        } else if path.contains("/games/") {
            AppCategory::Game
        } else {
            AppCategory::Other
        };

        Self {
            name: String::from(name),
            path: String::from(path),
            icon,
            category,
        }
    }
}

/// Descobre todos os aplicativos no sistema
pub fn discover_apps() -> Vec<AppInfo> {
    let mut apps = Vec::new();

    println!("[Shell] Escaneando /apps...");

    // Buscar em /apps recursivamente
    scan_directory("/apps", &mut apps, 0);

    // Ordenar por nome
    apps.sort_by(|a, b| a.name.cmp(&b.name));

    apps
}

/// Escaneia um diretório recursivamente buscando executáveis
fn scan_directory(path: &str, apps: &mut Vec<AppInfo>, depth: usize) {
    // Limitar profundidade para evitar loops infinitos
    if depth > 4 {
        return;
    }

    println!("[Shell] Escaneando: {} (depth {})", path, depth);

    let dir = match Dir::open(path) {
        Ok(d) => d,
        Err(e) => {
            println!("[Shell] Erro ao abrir {}: {:?}", path, e);
            return;
        }
    };

    for entry in dir.entries() {
        let name = entry.name();

        // Ignorar . e .. e arquivos ocultos
        if name.is_empty() || name.starts_with('.') {
            continue;
        }

        // Construir path completo
        let full_path = join_path(path, name);

        println!("[Shell]   Entry: {} (is_dir={})", name, entry.is_dir());

        if entry.is_dir() {
            // Se for diretório, verificar se contém um executável com o mesmo nome
            // Ex: /apps/system/terminal/terminal -> app "terminal"
            let potential_exe = join_path(&full_path, name);

            if file_exists(&potential_exe) {
                println!("[Shell]   -> App encontrado: {}", potential_exe);
                let display_name = capitalize(name);
                apps.push(AppInfo::new(&display_name, &potential_exe));
            } else {
                // Recursar no diretório
                scan_directory(&full_path, apps, depth + 1);
            }
        } else {
            // Se for arquivo executável diretamente em /apps
            if is_executable_name(name) {
                println!("[Shell]   -> Executável: {}", full_path);
                let display_name = capitalize(stem(name));
                apps.push(AppInfo::new(&display_name, &full_path));
            }
        }
    }
}

/// Verifica se um arquivo é potencialmente executável
fn is_executable_name(name: &str) -> bool {
    // Sem extensão ou .elf
    if name.contains('.') {
        name.ends_with(".elf")
    } else {
        true
    }
}

/// Verifica se um path existe como arquivo (não diretório)
fn file_exists(path: &str) -> bool {
    exists(path) && !is_dir(path)
}

/// Junta dois paths
fn join_path(base: &str, child: &str) -> String {
    let mut path = String::from(base.trim_end_matches('/'));
    path.push('/');
    path.push_str(child);
    path
}

/// Obtém o nome base sem extensão
fn stem(name: &str) -> &str {
    if let Some(pos) = name.rfind('.') {
        if pos > 0 {
            return &name[..pos];
        }
    }
    name
}

/// Capitaliza a primeira letra
fn capitalize(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();

    if let Some(first) = chars.next() {
        result.push(first.to_ascii_uppercase());
    }
    for c in chars {
        result.push(c);
    }

    result
}
