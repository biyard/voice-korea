import { test, expect } from '@playwright/test';

const timeouts = {
  wait: parseInt(process.env.WAIT_TIMEOUT || "5000", 10),
  visible: parseInt(process.env.VISIBLE_TIMEOUT || "10000", 10),
  url: parseInt(process.env.URL_TIMEOUT || "15000", 10)
};


test.describe('Find Email Page Flow', () => {
    test('[FindEmail-001] Successful Email Retrieval', async ({ page }) => {
        await page.goto('https://voice-korea.dev.biyard.co/en/find-email', {timeout: 60000});
        await page.screenshot({ path: 'screenshots/FindEmail-001/01-find-email-page.png', fullPage: true });

        await page.waitForLoadState('domcontentloaded');

        const nameInput = page.getByRole('textbox', { name: 'OOO' });
        await expect(nameInput).toBeVisible({ timeout: timeouts.visible });
        await nameInput.fill('John Doe');
        await page.screenshot({ path: 'screenshots/FindEmail-001/02-name-filled.png', fullPage: true });

        const cellphoneInput = page.getByRole('textbox', { name: '-0000-0000' });
        await expect(cellphoneInput).toBeVisible({ timeout: timeouts.visible });
        await cellphoneInput.fill('1234567890');
        await page.screenshot({ path: 'screenshots/FindEmail-001/03-cellphone-filled.png', fullPage: true });

        const authButton = page.getByText('Send Authentication Number');
        await expect(authButton).toBeVisible({ timeout: timeouts.visible });
        await authButton.click();
        await page.screenshot({ path: 'screenshots/FindEmail-001/04-auth-requested.png', fullPage: true });

        const authNumberInput = page.getByRole('textbox').nth(2);
        await expect(authNumberInput).toBeVisible({ timeout: timeouts.visible });
        await authNumberInput.fill('123456');
        await page.screenshot({ path: 'screenshots/FindEmail-001/05-auth-filled.png', fullPage: true });

        const findEmailButton = page.getByText('Find Email').nth(1);
        await expect(findEmailButton).toBeVisible({ timeout: timeouts.visible });
        await findEmailButton.click();
        await page.screenshot({ path: 'screenshots/FindEmail-001/06-find-email-clicked.png', fullPage: true });

        // await page.waitForLoadState('networkidle');
        await page.waitForTimeout(timeouts.wait);

        await expect(page).toHaveURL('https://voice-korea.dev.biyard.co/en/find-email', { timeout: timeouts.url });
        await page.screenshot({ path: 'screenshots/FindEmail-001/07-success.png', fullPage: true });
    });

    test('[FindEmail-002] Validate Empty Field Errors', async ({ page }) => {
        await page.goto('https://voice-korea.dev.biyard.co/en/find-email');
        await page.screenshot({ path: 'screenshots/FindEmail-002/01-find-email-page.png', fullPage: true });

        const authEmailButton = page.getByText('Send Authentication Number');
        await expect(authEmailButton).toBeVisible({ timeout: timeouts.visible });
        await authEmailButton.click();
        await page.screenshot({ path: 'screenshots/FindEmail-002/02-validation-errors.png', fullPage: true });
    });
});
