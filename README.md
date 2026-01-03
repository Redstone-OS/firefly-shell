# 🔥 Firefly Shell

O **Firefly Shell** é o desktop environment do RedstoneOS, responsável por fornecer a interface gráfica do usuário para interação com o sistema operacional.

## 📋 Visão Geral

O Shell é um aplicativo userspace que se comunica com o **Firefly Compositor** para criar e gerenciar a interface do desktop. Ele implementa:

- **Wallpaper** com suporte a gradiente fallback
- **Taskbar** flutuante com três barras separadas
- **Painéis popup** (Menu Iniciar, Widgets, Quick Settings)
- **Descoberta de aplicativos** (em desenvolvimento)

## 🏗️ Arquitetura

```
shell/src/
├── main.rs              # Entry point
├── app/                 # Lógica de aplicação
│   ├── mod.rs           # Declarações do módulo
│   ├── desktop.rs       # Desktop Environment principal
│   ├── discovery.rs     # Descoberta de apps (app.toml)
│   └── launcher.rs      # Lançamento de processos
├── ui/                  # Componentes visuais
│   ├── mod.rs           # Declarações do módulo
│   ├── wallpaper.rs     # Papel de parede
│   ├── taskbar.rs       # Barras flutuantes
│   └── panels/          # Painéis popup
│       ├── mod.rs       # Trait Panel + PanelType
│       ├── widget_panel.rs
│       ├── start_menu.rs
│       └── quick_settings.rs
├── theme/               # Sistema de design
│   ├── mod.rs           # Declarações do módulo
│   ├── colors.rs        # Paleta de cores
│   ├── glass.rs         # Efeito glassmorphism
│   └── metrics.rs       # Constantes de layout
└── render/              # Renderização
    ├── mod.rs           # Declarações do módulo
    └── font.rs          # Fonte bitmap 8x8
```

## 🎨 Design System

### Cores (Redstone Theme)

| Cor | Hex | Uso |
|-----|-----|-----|
| Accent | `#E8521F` | Cor principal (laranja Redstone) |
| Glass BG | `#1A1A1A` @ 70% | Fundo dos painéis |
| Glass Border | `#3A3A3A` | Bordas sutis |
| Text Primary | `#FFFFFF` | Texto principal |
| Text Secondary | `#A0A0A0` | Texto secundário |

### Glassmorphism

Todos os elementos UI utilizam o efeito **glass** com:
- Fundo semi-transparente escuro
- Bordas sutis com gradiente
- Cantos arredondados
- Blur simulado (placeholder)

## 🖥️ Componentes

### Taskbar (`ui/taskbar.rs`)

A taskbar é dividida em **três barras flutuantes** separadas:

```
┌─────────┐                 ┌─────────────────┐                 ┌─────────┐
│ Widgets │                 │ ◉ Menu │ Apps   │                 │  12:34  │
└─────────┘                 └─────────────────┘                 └─────────┘
   Left                           Center                           Right
```

- **Barra Esquerda**: Botão para abrir painel de widgets
- **Barra Central**: Menu iniciar + apps em execução
- **Barra Direita**: Relógio + Quick Settings

### Painéis (`ui/panels/`)

| Painel | Descrição |
|--------|-----------|
| `WidgetPanel` | Painel de widgets (placeholder) |
| `StartMenuPanel` | Menu iniciar com lista de apps |
| `QuickSettingsPanel` | Configurações rápidas (WiFi, Volume, etc.) |

Todos os painéis:
- Implementam o trait `Panel`
- Possuem animação slide-up
- Usam efeito glass

### Wallpaper (`ui/wallpaper.rs`)

O wallpaper suporta:
- **Imagem WebP** de `/system/resources/wallpapers/default.webp` (TODO)
- **Gradiente fallback** com cores Redstone quando imagem não disponível

## 📱 Descoberta de Apps

O sistema descobre apps automaticamente de `/apps/<vendor>/<name>/`:

```
/apps/
└── SYSTEM/
    └── terminal/
        ├── terminal.app      # Executável
        ├── app.toml          # Metadados
        └── assets/
            └── icon.svg      # Ícone
```

### Formato `app.toml`

```toml
[app]
name = "Terminal"
icon = "icon.svg"
category = "system"
```

> ⚠️ **Nota**: A descoberta está temporariamente desabilitada devido a problemas de estabilidade do filesystem.

## 🔧 Compilação

```bash
# Build debug
cargo build

# Build release
cargo build --release

# Build optimizado (para produção)
cargo build --profile opt-release
```

## 📦 Dependências

| Crate | Descrição |
|-------|-----------|
| `redpowder` | SDK do RedstoneOS |
| `gfx_types` | Tipos gráficos (Color, Rect, Size) |
| `rdsmath` | Funções matemáticas |

## 🚀 Roadmap

- [ ] Carregamento real de wallpaper WebP
- [ ] Renderização de ícones SVG
- [ ] Funcionalidade real do Quick Settings
- [ ] Widgets funcionais no painel de widgets
- [ ] Animações mais suaves
- [ ] Notificações do sistema
- [ ] Busca de apps no Menu Iniciar

## 📝 Notas de Desenvolvimento

### Comunicação com Compositor

O Shell se comunica com o Firefly Compositor via **IPC Ports**:

```rust
// Criar janela
Port::connect("firefly.compositor").send(CreateWindowRequest {...})

// Registrar como taskbar
compositor.send(RegisterTaskbarRequest { listener_port: "shell.taskbar" })
```

### Event Loop

```rust
loop {
    // 1. Processar eventos do compositor
    process_lifecycle_events();
    
    // 2. Processar input
    process_input();
    
    // 3. Atualizar animações
    update_animations();
    
    // 4. Redesenhar se necessário
    if dirty { redraw(); }
    
    // 5. Sleep para ~60fps
    sleep(16ms);
}
```

## 📄 Licença

Parte do projeto RedstoneOS - veja licença na raiz do repositório.

---

*Firefly Shell v0.3.0 - RedstoneOS Desktop Environment*
