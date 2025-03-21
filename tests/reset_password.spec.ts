import { test, expect } from '@playwright/test';

test.describe('Reset Password Flow', () => {
  test('[Reset-001] Request Authentication Number', async ({ page }) => {

    await page.goto('https://voice-korea.dev.biyard.co/en/reset-password');
    await page.waitForLoadState('domcontentloaded');

    const emailInput = page.getByRole('textbox').first();
    await expect(emailInput).toBeVisible({ timeout: 10000 });
    await emailInput.fill('testuser@example.com');

    await page.waitForSelector('text=Send Authentication Number', { timeout: 10000 });
    const sendAuthButton = page.getByText('Send Authentication Number');
    await expect(sendAuthButton).toBeVisible({ timeout: 10000 });
    await sendAuthButton.click();

    const authNumberInput = page.getByRole('textbox').nth(1);
    await expect(authNumberInput).toBeVisible({ timeout: 10000 });

    await authNumberInput.fill('123456');

    await page.waitForSelector('text=Reset Password', { timeout: 10000 });
    const resetPasswordButton = page.getByText('Reset Password').nth(1);
    await expect(resetPasswordButton).toBeVisible({ timeout: 10000 });
    await resetPasswordButton.click();

  });
});
