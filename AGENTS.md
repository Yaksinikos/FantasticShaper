# Global Rule
Always be extremely concise. Sacrifice grammar for the sake of concision.

# FantasticShaper - Development Guide for AI Agents

## Project Overview
FantasticShaper is a Tauri-based Linux Desktop GUI for WonderShaper network traffic shaping tool.
- **Version Control**: Git (no commits allowed)
- **Development Orchestrator**: Deno ^2.6 (runtime/checker)
- **Frontend Meta Framework**: SvelteKit ^5
- **Frontend UI Language**: Svelte ^5 with TypeScript
- **Backend Language**: Rust 2024 with Tauri ^2
- **Target**: Linux (primary Arch)
- **Frontend Runtime**: WebView
- **Backend Build System**: Cargo (tauri-build)

## Build & Development Commands

### Primary Development
```bash
# Start frontend development server
deno task dev

# Full application development
deno task tauri dev

# Production builds
deno task build            # Build SvelteKit frontend to ./build
deno task tauri build      # Build complete Tauri application
```

## Type Checking & Linting

### Frontend (Deno Native)
```bash
# TypeScript checking
deno task check            # Run SvelteKit type checking once

# Deno linting
deno task lint             # Run Deno linter on all files
deno fmt                   # Format code with Deno formatter
deno fmt --check           # Check formatting without changes
```

### Rust Backend
```bash
cd src-tauri && cargo clippy    # Rust linter
cd src-tauri && cargo fmt --all # Rust formatter
```

## Testing Commands

### Frontend Testing
```bash
deno task test             # Run all tests in watch mode
deno task test:run         # Run tests once
deno task test -- src/lib/specific.test.ts     # Run specific test file
deno task test -- -t "test name"               # Run tests matching name
```

### Rust Backend Testing
```bash
cd src-tauri && cargo test                      # Run all Rust tests
cd src-tauri && cargo test specific_test_name   # Run specific test
```

## Code Style Guidelines

### TypeScript/Svelte Conventions

#### Imports Organization
```typescript
// Tauri imports first
import { invoke } from "@tauri-apps/api/core";
// Svelte imports second
import { onMount } from "svelte";
// Local imports last
import "./style.css";
```

#### Variable Naming Conventions
- `fn_` for functions: `fn_ui_update_active_interface()`
- `p_` for parameters/props: `p_interface_value`
- `v_` for variables: `v_interface_selector_a`
- `gui_` for GUI state: `gui_custom_preset_valid`
- `ws_` for WonderShaper state: `ws_version`

#### TypeScript Patterns
- Strict type checking
- Always use `lang="ts"` in Svelte `<script>`
- Use Svelte 5 runes (`$state`, `$derived`, `$effect`) for reactivity
- Async functions with error handling:
```typescript
async function fn_ui_update_wondershaper_version() {
  let ressa: string = await invoke("fn_get_wondershaper_version", {});
  // Error handling example
  if (ressa) {
      // Use result
  }
}
```

#### Svelte Component Guidelines
- Use Svelte 5 modern syntax
- Component-scoped styles in `<style>` blocks
- Event handlers with explicit typing

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
    // Handle result
    if v_command.status.success() {
        // Parse and return
    } else {
        // Error handling
    }
}
```

#### Error Handling
- Use `match` for Result types
- `expect()` for fatal errors
- Return meaningful error strings to frontend
- `eprintln!()` for debugging

#### Function Naming
- `fn_` prefix for all functions
- `snake_case` for variable names
- Descriptive names

## Special Notes

### Tauri Integration
- Use `invoke()` for frontend-backend communication
- Commands follow `fn_` naming convention

### UI State Management
- Reactive state with Svelte 5 runes

## Development Workflow
1. Start with `deno task dev`
2. Run `deno task tauri dev` for full application
3. Type check, lint, format, and test before commits.
4. Build with `deno task build` + `deno task tauri build` for release.
