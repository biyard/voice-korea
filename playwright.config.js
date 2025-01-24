// @ts-check
const { defineConfig, devices } = require("@playwright/test");

/**
 * Read environment variables from file.
 * https://github.com/motdotla/dotenv
 */
// require('dotenv').config({ path: path.resolve(__dirname, '.env') });

/**
 * @see https://playwright.dev/docs/test-configuration
 */
module.exports = defineConfig({
  testDir: "./tests",
  testMatch: ["**/*.spec.ts", "**/*.test.ts", "**/*.spec.js", "**/*.test.js"],
  /* Run tests in files in parallel */
  fullyParallel: true,
  /* Fail the build on CI if you accidentally left test.only in the source code. */
  forbidOnly: !!process.env.CI,
  /* Retry on CI only */
  retries: process.env.CI ? 2 : 0,
  /* Opt out of parallel tests on CI. */
  workers: undefined,
  /* Reporter to use. See https://playwright.dev/docs/test-reporters */
  reporter: "html",
  /* Shared settings for all the projects below. See https://playwright.dev/docs/api/class-testoptions. */
  use: {
    baseURL: process.env.BASE_URL,
    trace: "on-first-retry",
  },

  /* Configure projects for major browsers */
  projects: [
    {
      name: "setup",
      testMatch: /.*\.setup\.js/,
      use: {
        headless: process.env.HEADLESS !== "false",
        storageState: {
          cookies: [],
          origins: [
            {
              origin: process.env.BASE_URL || "",
              localStorage: [
                {
                  name: "identity",
                  value: `"${process.env.IDENTITY}"` || "",
                },
              ],
            },
          ],
        },
        defaultBrowserType: "chromium",
        viewport: {
          width: 1920,
          height: 1080,
        },
        userAgent: devices["Desktop Chrome"].userAgent,
        deviceScaleFactor: 1,
        isMobile: false,
        hasTouch: false,
      },
    },
    {
      name: "chromium-with-authorized-user",
      testMatch: /.*\.spec\.js/,
      timeout: 30000,
      use: {
        headless: process.env.HEADLESS !== "false",
        storageState: {
          cookies: [],
          origins: [
            {
              origin: process.env.BASE_URL || "",
              localStorage: [
                {
                  name: "identity",
                  value: `"${process.env.IDENTITY}"` || "",
                },
              ],
            },
          ],
        },
        defaultBrowserType: "chromium",
        viewport: {
          width: 1920,
          height: 1080,
        },
        userAgent: devices["Desktop Chrome"].userAgent,
        deviceScaleFactor: 1,
        isMobile: false,
        hasTouch: false,
      },
      dependencies: ["setup"],
    },
    {
      name: "chromium-with-anonymous-user",
      testMatch: /.*\.aspec\.js/,
      timeout: 30000,
      use: {
        headless: process.env.HEADLESS !== "false",
        defaultBrowserType: "chromium",
        viewport: {
          width: 1920,
          height: 1080,
        },
        userAgent: devices["Desktop Chrome"].userAgent,
        deviceScaleFactor: 1,
        isMobile: false,
        hasTouch: false,
      },
    },
  ],
});
