import { test, expect } from '@playwright/test';

test.describe('Login Page Flow', () => {
  test('[Login-001] Successful Login', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');
    await page.waitForLoadState('domcontentloaded');

    const emailInput = page.getByRole('textbox', { name: "Email" });
    await expect(emailInput).toBeVisible({ timeout: 10000 });
    await emailInput.fill('jesuswrites20043@gmail.com');

    const passwordInput = page.getByRole('textbox', { name: "Password" });
    await expect(passwordInput).toBeVisible({ timeout: 10000 });
    await passwordInput.fill('12345678a#');

    const loginButton = page.getByRole('button', { name: "Login" });
    await expect(loginButton).toBeVisible({ timeout: 10000 });
    await loginButton.click();

    await expect(page).toHaveURL(/.*surveys/);

  });

  test('[Login-002] Navigate to Reset Password Page', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');
    await page.waitForLoadState('domcontentloaded');

    const resetPasswordLink = page.getByRole('link', { name: "Reset Password" });
    await expect(resetPasswordLink).toBeVisible();
    await resetPasswordLink.click();

    await expect(page).toHaveURL(/.*reset-password/);
  });

  test('[Login-003] Navigate to Create Account Page', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');
    await page.waitForLoadState('domcontentloaded');

    const createAccountLink = page.getByRole('link', { name: "Create Account" });
    await expect(createAccountLink).toBeVisible();
    await createAccountLink.click();

    await expect(page).toHaveURL(/.*create/);
  });
});
