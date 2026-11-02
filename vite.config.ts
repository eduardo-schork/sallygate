import process from 'node:process';

import react from '@vitejs/plugin-react';
import { defineConfig } from 'vitest/config';

// Set by `tauri dev` when the app runs on a device or emulator.
const tauriDevHost = process.env['TAURI_DEV_HOST'];

export default defineConfig({
  plugins: [react()],
  // Keeps Rust compiler errors visible in the terminal.
  clearScreen: false,
  server: {
    // Tauri expects a fixed port and must fail if it is taken.
    port: 1420,
    strictPort: true,
    host: tauriDevHost ?? false,
    ...(tauriDevHost ? { hmr: { protocol: 'ws', host: tauriDevHost, port: 1421 } } : {}),
    // Rust layers are rebuilt by `tauri dev`, not by Vite.
    watch: {
      ignored: [
        '**/src/domain/**',
        '**/src/application/**',
        '**/src/infrastructure/**',
        '**/src/presentation/desktop/**',
        '**/target/**',
      ],
    },
  },
  test: {
    environment: 'jsdom',
    setupFiles: ['./vitest.setup.ts'],
    include: ['src/presentation/ui/**/*.test.{ts,tsx}', 'eslint-rules/**/*.test.js'],
  },
});
