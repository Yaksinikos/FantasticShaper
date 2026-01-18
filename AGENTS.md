# FantasticShaper - Development Guide for AI Agents

## Project Overview
FantasticShaper is a Tauri-based Linux Desktop GUI for WonderShaper network traffic shaping tool.
- **Version Control**: Git (no commits allowed)
- **Development Orchestrator**: Deno ^2.6 (main package manager)
- **Frontend Meta Framework**: SvelteKit ^5
- **Frontend UI Language**: Svelte ^5 with TypeScript
- **Backend Language**: Rust 2024 with Tauri ^2
- **Cross-platform application window creation library**: TAO
- **Frontend Runtime**: WebView
- **Cross-platform WebView rendering library**: WRY
- **Target**: Linux (primary Arch)
- **Local Development Server**: Vite ^7.3
- **Production Build Tool**: Vite ^7.3
- **Backend Build System**: Cargo (tauri-build)
- **Backend Glue Crate (Runtime)**: tauri-runtime
- **Backend Macro Crate**: tauri-macros
- **Backend Utils Crate**: tauri-utils

## Build & Development Commands

### Primary Development (Deno-focused)
```bash
# Start development server (frontend only)
deno task dev              # Starts SvelteKit dev server on localhost:1420

# Full application development
deno task tauri dev        # Starts Tauri in development mode

# Production builds
deno task build            # Build SvelteKit frontend to ./build
deno task tauri build      # Build complete Tauri application

# Preview and utilities
deno task preview          # Preview built SvelteKit app
deno task tauri <command>  # Access any Tauri CLI command
```

## Type Checking & Linting (Deno Native)

```bash
# TypeScript checking
deno task check            # Run SvelteKit type checking once
deno task check:watch      # Run type checking in watch mode

# Deno linting
deno task lint             # Run Deno linter on all files
deno task lint:fix         # Run Deno linter with auto-fixes
deno fmt                   # Format code with Deno formatter
deno fmt --check           # Check formatting without making changes
```

### Rust Backend Linting
```bash
cd src-tauri && cargo clippy    # Rust linter
cd src-tauri && cargo fmt       # Rust formatter (check)
cd src-tauri && cargo fmt --all # Rust formatter (write)
```

## Testing Commands

### Frontend Testing
```bash
deno task test             # Run all tests in watch mode
deno task test:run         # Run tests once
deno task test:ui          # Run tests with UI interface

# Single test examples
deno task test -- src/lib/specific.test.ts     # Run specific test file
deno task test -- -t "test name"               # Run tests matching name
deno task test -- --reporter=verbose           # Verbose output
```

### Rust Backend Testing
```bash
cd src-tauri && cargo test                      # Run all Rust tests
cd src-tauri && cargo test specific_mod         # Run specific module
cd src-tauri && cargo test specific_test_name   # Run specific test
cd src-tauri && cargo test -- --nocapture       # Show print output
```

## Code Style Guidelines

### TypeScript/Svelte Conventions

#### Imports Organization
```typescript
// Tauri imports first
import { getCurrentWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api/core";

// Svelte imports second  
import { onMount } from "svelte";

// Local imports last
import "./style.css";
```

#### Variable Naming Conventions
- Use Hungarian notation prefixes:
  - `fn_` for functions: `fn_ui_update_active_interface()`
  - `p_` for parameters/props: `p_interface_value`
  - `v_` for variables: `v_interface_selector_a`
  - `gui_` for GUI state: `gui_custom_preset_valid`
  - `ws_` for WonderShaper state: `ws_version`, `ws_active`

#### TypeScript Patterns
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

#### Svelte Component Guidelines
- Use Svelte 5 modern syntax
- Component-scoped styles in `<style>` blocks
- Event handlers with explicit typing:
```typescript
oninput={(e: Event) => {
  if (!e.target || !(e.target instanceof HTMLInputElement)) return;
  // Handle input
}}
```

### Rust Backend Guidelines

#### Tauri Command Patterns
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

#### Error Handling
- Use `match` for Result types
- Use `expect()` for fatal errors (when program cannot continue)
- Return meaningful error strings to frontend
- Print to stderr with `eprintln!()` for debugging

#### Function Naming
- Use `fn_` prefix for all functions (consistent with frontend)
- Snake_case for variable names: `v_command`, `v_return_string`
- Descriptive names indicating purpose

## Configuration Files

### Deno Configuration
- `deno.json` - Main orchestrator with tasks and linting
- Deno linter with "recommended" rules
- Formatter supports TypeScript, JavaScript, and Svelte files

### Frontend Configuration
- `tsconfig.json` - TypeScript strict mode enabled
- `svelte.config.js` - SvelteKit with static adapter
- `vite.config.js` - Build configuration
- `vitest.config.js` - Test configuration

### Rust Configuration
- `Cargo.toml` - Rust dependencies with strict lints:
  - `unsafe_code = "forbid"`
  - Clippy warnings enabled

## CSS & Styling Guidelines

### Theming System
- Use CSS custom properties defined in `src/routes/style.css`
- Support both dark/light modes via `@media (prefers-color-scheme)`
- High contrast support with `@media (prefers-contrast: more)`

### Class Naming
- Use descriptive class names: `.button_style`, `.input_slider_switch`
- Component-specific styles in component `<style>` blocks
- Inline styles sparingly for dynamic values

## Special Notes

### Tauri Integration
- Use `invoke()` for frontend-backend communication
- Commands follow `fn_` naming convention

### Platform-Specific Code
- WonderShaper commands target Linux systems
- Network interface detection via `nmcli`
- System command execution through `std::process::Command`

### UI State Management
- DOM manipulation via `document.getElementById()` where needed
- Reactive state with Svelte 5 runes

## Development Workflow
1. Start with `deno task dev`
2. Run `deno task tauri dev` for full application
3. Type check with `deno task check` before commits
4. Run linting with `deno task lint` before commits
5. Format code with `deno fmt` before commits
6. Run tests with `deno task test:run` before commits
7. Build with `deno task build` + `deno task tauri build` for release