import { test, expect } from '@playwright/test';

test('Save login state', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');

    const emailInput = page.getByRole('textbox').first();
    await expect(emailInput).toBeVisible();
    await emailInput.fill('jesuswrites20043@gmail.com');

    const passwordInput = page.getByRole('textbox').nth(1);
    await expect(passwordInput).toBeVisible();
    await passwordInput.fill('12345678a#');

    const loginButton = page.getByRole('button', { name: "Login" });
    await expect(loginButton).toBeVisible();
    await loginButton.click();

    await page.waitForLoadState('networkidle');

    await page.context().storageState({ path: 'storage/auth.json' });

    console.log('Login session saved!');
});