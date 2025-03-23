import { test, expect } from '@playwright/test';

const timeouts = {
    wait: process.env.WAIT_TIMEOUT as unknown as number | 5000,
    visible: process.env.VISIBLE_TIMEOUT as unknown as number | 10000,
    url: process.env.URL_TIMEOUT as unknown as number | 15000
  }


test.describe('Reset Password Flow', () => {
    test('[Reset-001] Request Authentication Number', async ({ page }) => {
        await page.goto('https://voice-korea.dev.biyard.co/en/reset-password');
        await page.screenshot({ path: 'screenshots/reset-password-001/01-reset-password-page.png', fullPage: true });

        await page.waitForLoadState('domcontentloaded');

        const emailInput = page.getByRole('textbox').first();
        await expect(emailInput).toBeVisible({ timeout: timeouts.visible });
        await emailInput.fill('testuser@example.com');
        await page.screenshot({ path: 'screenshots/reset-password-001/02-reset-password-email.png', fullPage: true });

        await page.waitForSelector('text=Send Authentication Number', { timeout: 10000 });
        const sendAuthButton = page.getByText('Send Authentication Number');
        await expect(sendAuthButton).toBeVisible({ timeout: timeouts.visible });
        await sendAuthButton.click();
        await page.screenshot({ path: 'screenshots/reset-password-001/03-auth-number-sent.png', fullPage: true });

        const authNumberInput = page.getByRole('textbox').nth(1);
        await expect(authNumberInput).toBeVisible({ timeout: timeouts.visible });

        await authNumberInput.fill('123456');
        await page.screenshot({ path: 'screenshots/reset-password-001/04-auth-number-filled.png', fullPage: true });

        await page.waitForSelector('text=Reset Password', { timeout: 10000 });
        const resetPasswordButton = page.getByText('Reset Password').nth(1);
        await expect(resetPasswordButton).toBeVisible({ timeout: timeouts.visible });
        await resetPasswordButton.click();
        await page.screenshot({ path: 'screenshots/reset-password-001/05-reset-password-clicked.png', fullPage: true });
    });

    test('[Reset-002] Validate Error Messages on Empty Fields', async ({ page }) => {
        await page.goto('https://voice-korea.dev.biyard.co/en/reset-password');
        await page.screenshot({ path: 'screenshots/reset-password-002/01-reset-password-page.png', fullPage: true });

        const sendAuthButton = page.getByText('Send Authentication Number');
        await expect(sendAuthButton).toBeVisible({ timeout: timeouts.visible });
        await sendAuthButton.click();
        await page.screenshot({ path: 'screenshots/reset-password-002/02-reset-password-error.png', fullPage: true });

        const sendAuthError = page.getByText('The email format is incorrect.');
        await expect(sendAuthError).toBeVisible();

        const resetPasswordButton = page.getByText('Reset Password').nth(1);
        await expect(resetPasswordButton).toBeVisible({ timeout: timeouts.visible });
        await resetPasswordButton.click();
        await page.screenshot({ path: 'screenshots/reset-password-002/03-error-missing-auth-number.png', fullPage: true });

        const resetPasswordError1 = page.getByText('The email format is incorrect.');
        await expect(resetPasswordError1).toBeVisible();
    }
    );
});