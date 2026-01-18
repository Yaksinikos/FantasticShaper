# FantasticShaper - Development Guide for AI Agents

## Project Overview
FantasticShaper is a Tauri-based Linux Desktop GUI for WonderShaper network traffic shaping tool.
- **Frontend Language**: TypeScript
- **Frontend UI Framework**: SvelteKit 5+
- **Frontend Runtime**: Deno 2.6+
- **Frontend Build Tool**: Vite 7.3+
- **Frontend Server**: Vite 7.3+
- **Backend Language**: Rust 2024
- **Rust Build System**: Cargo
- **Backend ToolKit**: Tauri 2+
- **Target**: Linux Cross-Distribution (Primary Arch-Linux)

## Build & Development Commands

### Primary Development
```bash
# Start development server (frontend + Tauri)
deno task dev           # Starts SvelteKit dev server on localhost:1420
deno run tauri dev       # Starts Tauri in development mode

# Production Build
deno task build         # Build SvelteKit frontend to ./build
deno run tauri build     # Build complete Tauri application

# Preview Production Build
deno run preview         # Preview built SvelteKit app

# Tauri Commands
deno run tauri <command>  # Access any Tauri CLI command
```

### Type Checking & Validation
```bash
deno run check           # Run SvelteKit type checking once
deno run check:watch     # Run type checking in watch mode

# Rust linting (if configured)
cd src-tauri && cargo clippy   # Rust linter
cd src-tauri && cargo fmt       # Rust formatter (check)
cd src-tauri && cargo fmt --all # Rust formatter (write)
```

### Testing Commands
```bash
# No test framework currently configured
# To add tests, consider adding vitest:
# npm install --save-dev vitest @vitest/ui

# Rust tests (if added)
cd src-tauri && cargo test          # Run all Rust tests
cd src-tauri && cargo test <name>   # Run specific test
```

## TypeScript/Svelte Code Guidelines

### Imports Organization
```typescript
// Tauri imports first
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

// Svelte imports second
import { onMount } from "svelte";

// Local imports last
import "./style.css";
```

### Variable Naming Conventions
- Use Hungarian notation prefixes:
  - `fn_` for functions: `fn_ui_update_active_interface()`
  - `p_` for parameters/props: `p_interface_value`
  - `v_` for variables: `v_interface_selector_a`
  - `gui_` for GUI state: `gui_custom_preset_valid`
  - `ws_` for WonderShaper state: `ws_version`, `ws_active`

### TypeScript Patterns
- Use strict type checking (enabled in tsconfig.json)
- Async functions with proper error handling:
```typescript
async function fn_ui_update_wondershaper_version() {
  let ressa: string = await invoke("fn_get_wondershaper_version", {});
  document.getElementById("id_wondershaper_version")?.setHTMLUnsafe(ressa);
}
```

### Svelte Component Guidelines
- Use Svelte 5 modern syntax
- Component-scoped styles in `<style>` blocks
- Event handlers with explicit typing:
```typescript
oninput={(e: Event) => {
  if (!e.target || !(e.target instanceof HTMLInputElement)) return;
  // Handle input
}}
```

## Rust Backend Guidelines

### Tauri Command Patterns
```rust
#[tauri::command]
async fn fn_get_wondershaper_version() -> String {
    let v_command = Command::new("sh")
        .arg("-c")
        .arg("wondershaper -v")
        .output()
        .expect("wondershaper not installed");
    
    // Handle result with proper error handling
    if v_command.status.success() {
        // Parse and return
    }
}
```

### Error Handling
- Use `match` for Result types
- Use `expect()` for fatal errors (when program cannot continue)
- Return meaningful error strings to frontend
- Print to stderr with `eprintln!()` for debugging

### Function Naming
- Use `fn_` prefix for all functions (consistent with frontend)
- Snake_case for variable names: `v_command`, `v_return_string`
- Descriptive names indicating purpose

## CSS & Styling Guidelines

### Theming System
- Use CSS custom properties defined in `src/routes/style.css`
- Support both dark/light modes via `@media (prefers-color-scheme)`
- High contrast support with `@media (prefers-contrast: more)`

### Class Naming
- Use descriptive class names: `.button_style`, `.input_slider_switch`
- Component-specific styles in component `<style>` blocks
- Inline styles sparingly for dynamic values

### Color Variables
```css
/* Use semantic color variables */
background-color: var(--bg-color);
color: var(--text-color);
border-color: var(--border-color);
```

## File Organization

### Directory Structure
```
src/                    # SvelteKit frontend
├── routes/             # SvelteKit pages
│   ├── +page.svelte   # Main application page
│   ├── +layout.svelte # Root layout
│   └── style.css      # Global styles
└── app.html           # HTML template

src-tauri/              # Rust backend
├── src/
│   ├── main.rs        # Application entry point
│   └── lib.rs         # Tauri commands
├── Cargo.toml         # Rust dependencies
└── tauri.conf.json    # Tauri configuration
```

### Configuration Files
- `package.json` - deno dependencies and scripts
- `deno.lock` - Deno dependency lock (used alongside npm)
- `svelte.config.js` - SvelteKit configuration with static adapter
- `vite.config.js` - Vite build configuration
- `tsconfig.json` - TypeScript strict mode enabled

## Testing & Quality Assurance

### Current State
- **Linting tools** ESLint, Clippy, rustfmt
- **Testing tools** Vitest
- Manual testing through development workflow
- Rust lints configured in Cargo.toml (unsafe_code forbidden, clippy warnings)

## Special Notes

### Tauri Integration
- Use `invoke()` for frontend-backend communication
- Commands must be registered in `tauri::generate_handler![]`
- Async commands should return `Future<Output = T>`

### Platform-Specific Code
- WonderShaper commands target Linux systems
- Network interface detection via `nmcli`
- System command execution through `std::process::Command`

### UI State Management
- No external state management library
- Use Svelte reactivity with `let` declarations
- DOM manipulation via `document.getElementById()` where needed

## Development Workflow
1. Start with `deno task dev`
2. Run `deno run tauri dev` for full application
3. Type check with `deno run check` before commits
4. Build with `deno task build` + `deno run tauri build` for release