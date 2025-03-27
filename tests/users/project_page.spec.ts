import { test, expect } from '@playwright/test';

const devMobileViewports = [
    { device: 'iPhone 12', width: 390, height: 844 },
    // { device: 'Samsung Galaxy S20', width: 412, height: 915 },
    // { device: 'Pixel 5', width: 393, height: 851 },
    // { device: 'iPad Mini', width: 768, height: 1024 },
];

test.describe('UI Mobile Responsiveness & Project Page Tests', () => {
    // Mobile responsiveness tests
    for (const viewport of devMobileViewports) {
        test(`Check responsiveness on ${viewport.device} (${viewport.width}x${viewport.height})`, async ({ page }) => {
            await page.setViewportSize({ width: viewport.width, height: viewport.height });
            await page.goto('https://dev.voice-korea.com/en/projects');

            const bodyOverflowX = await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth);
            expect(bodyOverflowX).toBeTruthy();

            await page.screenshot({ path: `screenshots/ui-${viewport.device}.png`, fullPage: true });
        });
    }

    test('Clicking a project navigates to the correct page', async ({ page }) => {
        await page.goto('https://dev.voice-korea.com/en/projects');
        const randomProject = page.locator('.max-w-\\[1300px\\] > div:nth-child(2) > div:nth-child(2) > div:nth-child(2) > div');
        await randomProject.click();

        const url = page.url();
        const match = url.match(/\/projects\/(\d+)$/); 

        expect(match).not.toBeNull(); 
        expect(Number(match?.[1])).not.toBeNaN(); 
    });

    test('Searching filters projects', async ({ page }) => {
        await page.goto('https://dev.voice-korea.com/en/projects');
        const searchBox = page.getByRole('textbox', {name: "Search"}); 
        await expect(searchBox).toBeVisible();
        await searchBox.fill('Economy'); 
        await page.keyboard.press('Enter'); 
    });

    test('Set Oldest-Newest Filters', async ({ page }) => {
        await page.goto('https://dev.voice-korea.com/en/projects');
        const searchBox = page.locator('summary') 
        await searchBox.click();
        const oldestButton = await page.getByRole("button", { name: "Oldest" });
        await expect(oldestButton).toBeVisible();
        await oldestButton.click();
        const newestButton = await page.getByRole("button", { name: "Newest" });
        await expect(newestButton).toBeVisible();
        await newestButton.click();
    });
});
