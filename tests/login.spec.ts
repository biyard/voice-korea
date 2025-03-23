import { test, expect } from '@playwright/test';

const timeouts = {
  wait: process.env.WAIT_TIMEOUT as unknown as number,
  visible: process.env.VISIBLE_TIMEOUT as unknown as number,
  url: process.env.URL_TIMEOUT as unknown as number
}


test.describe('Login Page Flow', () => {
  test('[Login-001] Successful Login', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');
    await page.screenshot({ path: 'screenshots/Login-001/01-login-page.png', fullPage: true });

    await page.waitForLoadState('domcontentloaded');

    const emailInput = page.getByRole('textbox', { name: "Email" });
    await expect(emailInput).toBeVisible({ timeout: timeouts.visible});
    await emailInput.fill('jesuswrites20043@gmail.com');
    await page.screenshot({ path: 'screenshots/Login-001/02-email-filled.png', fullPage: true });

    const passwordInput = page.getByRole('textbox', { name: "Password" });
    await expect(passwordInput).toBeVisible({ timeout: timeouts.visible});
    await passwordInput.fill('12345678a#');
    await page.screenshot({ path: 'screenshots/Login-001/03-password-filled.png', fullPage: true });

    const loginButton = page.getByRole('button', { name: "Login" });
    await expect(loginButton).toBeVisible({ timeout: timeouts.visible});
    await loginButton.click();
    await page.screenshot({ path: 'screenshots/Login-001/04-login-clicked.png', fullPage: true });

    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);

    await expect(page).toHaveURL(/.*surveys/, { timeout: timeouts.url });
    await page.screenshot({ path: 'screenshots/Login-001/05-surveys-page.png', fullPage: true });
  });

  test('[Login-002] Validate Error Messages on Empty Fields', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');
    await page.screenshot({ path: 'screenshots/Login-002/01-empty-fields-login.png', fullPage: true });

    const loginButton = page.getByRole('button', { name: "Login" });
    await expect(loginButton).toBeVisible({ timeout: timeouts.visible});
    await loginButton.click();
    await page.screenshot({ path: 'screenshots/Login-002/02-error-messages.png', fullPage: true });

    const loginError = page.getByText('You have failed to log in.');

    await expect(loginError).toBeVisible();
  });

  test('[Login-003] Navigate to Reset Password Page', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');
    await page.screenshot({ path: 'screenshots/Login-003/01-reset-password-nav.png', fullPage: true });

    const resetPasswordLink = page.getByRole('link', { name: "Reset Password" });
    await expect(resetPasswordLink).toBeVisible();
    await resetPasswordLink.click();

    await expect(page).toHaveURL(/.*reset-password/);
    await page.screenshot({ path: 'screenshots/Login-003/02-reset-password-page.png', fullPage: true });
  });

  test('[Login-004] Navigate to Create Account Page', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');
    await page.screenshot({ path: 'screenshots/Login-004/01-create-account-nav.png', fullPage: true });

    const createAccountLink = page.getByRole('link', { name: "Create Account" });
    await expect(createAccountLink).toBeVisible();
    await createAccountLink.click();

    await expect(page).toHaveURL(/.*create/);
    await page.screenshot({ path: 'screenshots/Login-004/02-create-account-page.png', fullPage: true });
  });
});