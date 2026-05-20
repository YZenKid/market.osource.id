import { defineConfig, devices } from '@playwright/test';

const playwrightPort = process.env.PLAYWRIGHT_PORT ?? '4173';
const playwrightOrigin = process.env.PLAYWRIGHT_ORIGIN ?? `http://127.0.0.1:${playwrightPort}`;
const playwrightBaseUrl = process.env.PLAYWRIGHT_BASE_URL ?? playwrightOrigin;
const playwrightApiBaseUrl = process.env.PLAYWRIGHT_PUBLIC_API_BASE_URL ?? 'http://127.0.0.1:7301';

export default defineConfig({
  testDir: './tests',
  timeout: 30_000,
  retries: 0,
  reporter: [['list'], ['html', { open: 'never', outputFolder: 'playwright-report' }]],
  webServer: {
    command: `PUBLIC_API_BASE_URL=${playwrightApiBaseUrl} ORIGIN=${playwrightOrigin} PORT=${playwrightPort} node build/index.js`,
    url: playwrightOrigin,
    reuseExistingServer: true,
    timeout: 30_000
  },
  use: {
    baseURL: playwrightBaseUrl,
    trace: 'on-first-retry',
    screenshot: 'only-on-failure'
  },
  projects: [
    {
      name: 'desktop',
      use: { ...devices['Desktop Chrome'], viewport: { width: 1440, height: 900 } }
    },
    {
      // Chromium emulation of tablet viewport. Not Safari/WebKit parity.
      // WebKit system deps unavailable on this host; Chromium used for layout/responsive validation.
      name: 'tablet-chromium',
      use: { ...devices['Desktop Chrome'], viewport: { width: 768, height: 1024 } }
    },
    {
      // Chromium emulation of mobile viewport. Not Safari/WebKit parity.
      // WebKit system deps unavailable on this host; Chromium used for layout/responsive validation.
      name: 'mobile-chromium',
      use: { ...devices['Desktop Chrome'], viewport: { width: 390, height: 844 }, isMobile: true }
    }
  ]
});
