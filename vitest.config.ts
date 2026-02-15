import { defineConfig } from 'vitest/config';
import path from 'node:path';

export default defineConfig({
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./tests/setup.ts'],
    reporters: ['default'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'lcov'],
      lines: 80,
      functions: 80,
      statements: 80,
      branches: 75,
      exclude: [
        '**/node_modules/**',
        '**/.next/**',
        '**/dist/**',
        '**/*.config.{js,ts}',
        '**/*.test.{js,ts,tsx}'
      ]
    },
    alias: {
      '@': path.resolve(__dirname, 'frontend')
    }
  }
});
