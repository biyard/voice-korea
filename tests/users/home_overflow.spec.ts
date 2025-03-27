import { test, expect } from '@playwright/test';

const devMobileViewports = [
  { device: 'iPhone 12', width: 390, height: 844 },
  { device: 'Samsung Galaxy S20', width: 412, height: 915 },
  { device: 'Pixel 5', width: 393, height: 851 },
  { device: 'iPad Mini', width: 768, height: 1024 }, 
];

test.describe('UI Mobile Responsiveness', () => {
  for (const viewport of devMobileViewports) {
    test(`Check responsiveness on ${viewport.device} (${viewport.width}x${viewport.height})`, async ({ page }) => {
      await page.setViewportSize({ width: viewport.width, height: viewport.height });
      await page.goto('https://dev.voice-korea.com/en/');

      const bodyOverflowX = await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth);
      expect(bodyOverflowX).toBeTruthy();

      await page.screenshot({ path: `screenshots/ui-${viewport.device}.png`, fullPage: true });
    });
  }
});
