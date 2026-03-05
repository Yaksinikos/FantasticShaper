import { defineConfig, mergeConfig } from "vitest/config";
import { playwright } from "@vitest/browser-playwright";
import viteConfig from "./vite.config.ts";

// https://vite.dev/config/
export default defineConfig((configEnv) =>
  mergeConfig(
    viteConfig(configEnv),
    defineConfig({
      test: {
        expect: { requireAssertions: true },
        projects: [
          {
            extends: "./vite.config.ts",
            test: {
              name: "client",
              browser: {
                enabled: true,
                provider: playwright(),
                instances: [{ browser: "chromium", headless: true }],
              },
              include: ["src/**/*.svelte.{test,spec}.{js,ts}"],
              exclude: ["src/lib/server/**"],
            },
          },

          {
            extends: "./vite.config.ts",
            test: {
              name: "server",
              environment: "node",
              include: ["src/**/*.{test,spec}.{js,ts}"],
              exclude: ["src/**/*.svelte.{test,spec}.{js,ts}"],
            },
          },
        ],
      },
    })
  )
);
