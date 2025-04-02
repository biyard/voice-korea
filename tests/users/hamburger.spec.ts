import { test, expect } from '@playwright/test';

test.describe('Corresponding Individual Hamburger Tests', () => {

    test('[Hamburger-ID-001] Confirm it has project details', async ({ page }) => {
        await page.goto(`https://dev.voice-korea.com/en/`);
        await page.screenshot({ path: `screenshots/users/Hamburger-ID-001/01-page-entered.png`, fullPage: true });

        const hamburger = page.getByRole('button').filter({ hasText: /^$/ })
        await expect(hamburger).toBeVisible();
        await hamburger.click();
        await page.screenshot({ path: `screenshots/users/Hamburger-ID-001/02-hamburger-clicked.png`, fullPage: true });

        const mainPage = page.getByRole('button', {name: "Main Page"})
        await expect(mainPage).toBeVisible();
        await page.screenshot({ path: `screenshots/users/Hamburger-ID-001/03-main-page-clicked.png`, fullPage: true });

        await hamburger.click();
        await page.screenshot({ path: `screenshots/users/Hamburger-ID-001/04-hamburger-clicked.png`, fullPage: true });
    });
})