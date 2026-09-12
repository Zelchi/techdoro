import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
    root: 'src',
    plugins: [solid()],
    clearScreen: false,
    server: {
        port: 1420,
        strictPort: true,
        host: host || '127.0.0.1',
        hmr: host
            ? {
                  protocol: 'ws',
                  host,
                  port: 1421,
              }
            : undefined,
        watch: {
            ignored: ['**/tauri/**'],
        },
    },
    envPrefix: ['VITE_', 'TAURI_ENV_*'],
    build: {
        outDir: '../dist',
        emptyOutDir: true,
        target: process.env.TAURI_ENV_PLATFORM === 'windows' ? 'chrome105' : 'safari13',
        minify: !process.env.TAURI_ENV_DEBUG,
        sourcemap: Boolean(process.env.TAURI_ENV_DEBUG),
    },
});
