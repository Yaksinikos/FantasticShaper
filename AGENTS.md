# FantasticShaper - Development Guide for AI Agents

## Project Overview
FantasticShaper is a Tauri-based Linux Desktop GUI for WonderShaper network traffic shaping tool.
- **Version Control**: Git (no commits allowed)
- **Development Orchestrator**: Deno ^2.6
- **Package Manager**: Deno ^2.6
- **Project Toolkit Setup**: Tauri ^2
- **Cross-platform application window creation library**: TAO
- **Frontend Runtime**: WebView
- **Cross-platform WebView rendering library**: WRY
- **Target**: Linux (primary Arch)

### Frontend
- **Frontend Meta Framework**: SvelteKit ^5
- **Frontend UI Language**: Svelte ^5
- **Frontend Language Flavor**: TypeScript

- **Local Development Server**: Vite ^7.3
- **Production Build Tool**: Vite ^7.3

### Backend
- **Inter-Process Communication**: Tauri API

- **Backend Language**: Rust 2024
- **Backend Build System**: Cargo (tauri-build)
- **Backend Framework**: Tauri ^2
- **Backend Glue Crate (Runtime)**: tauri-runtime
- **Backend Macro Crate**: tauri-macros
- **Backend Utils Crate**: tauri-utils

## File Organization
- first: Project: Follow Tauri Project Structure Guidelines
- second: Backend: Follow Cargo Project Structure Guidelines
- third: Frontend: Follow Svelte Project Structure Guidelines

### Directory Structure

### Configuration Files
- `package.json` - dependencies and scripts (orchestrated by Deno tasks)
- `deno.json` - Deno configuration and tasks
- `deno.lock` - Deno dependency lock
- `svelte.config.js` - SvelteKit configuration with static adapter
- `vite.config.js` - Vite build configuration
- `tsconfig.json` - TypeScript strict mode enabled

## Build & Development Commands

### Primary Development
```bash
# Start development server (frontend + Tauri)
deno task dev           # Starts SvelteKit dev server on localhost:1420
deno task tauri dev      # Starts Tauri in development mode

# Production Build
deno task build         # Build SvelteKit frontend to ./build
deno task tauri build     # Build complete Tauri application

# Preview Production Build
deno task preview         # Preview built SvelteKit app

# Tauri Commands
deno task tauri <command>  # Access any Tauri CLI command
```

### Type Checking & Validation
```bash
deno task check           # Run SvelteKit type checking once
deno task check:watch     # Run type checking in watch mode

# Rust linting
cd src-tauri && cargo clippy   # Rust linter
cd src-tauri && cargo fmt       # Rust formatter (check)
cd src-tauri && cargo fmt --all # Rust formatter (write)
```

### Testing Commands
```bash
# Frontend Tests
deno task test           # Run all tests in watch mode
deno task test:run       # Run tests once
deno task test:ui        # Run tests with UI interface

# Rust tests
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
- Always use `lang="ts"` in Svelte `<script>` tags
- Use Svelte 5 runes (`$state`, `$derived`, `$effect`) for reactivity
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

## Testing & Quality Assurance

### Current State
- **Linting tools**: 
- Deno
- Clippy
- rustfmt

- **Testing tools**: 
- Deno frontend testing
- Vitest frontend testing
- Cargo backend testing
- Rust lints configured in Cargo.toml (unsafe_code forbidden, clippy warnings)

## Special Notes

### Tauri Integration
- Use `invoke()` for frontend-backend communication

### Platform-Specific Code
- WonderShaper commands target Linux systems
- Network interface detection via `nmcli`
- System command execution through `std::process::Command`

### UI State Management
- DOM manipulation via `document.getElementById()` where needed

## Development Workflow
1. Start with `deno task dev`
2. Run `deno task tauri dev` for full application
3. Type check with `deno task check` before commits
4. Run linting with `deno task lint` before commits
5. Run tests with `deno task test:run` before commits
6. Build with `deno task build` + `deno task tauri build` for release