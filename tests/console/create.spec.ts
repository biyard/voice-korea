import { test, expect } from '@playwright/test';

const timeouts = {
  wait: parseInt(process.env.WAIT_TIMEOUT || "5000", 10),
  visible: parseInt(process.env.VISIBLE_TIMEOUT || "10000", 10),
  url: parseInt(process.env.URL_TIMEOUT || "15000", 10)
};


test.describe('Create Page Flow', () => {
  test('[Create-001] Successful Account Creation', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/create');
    await page.screenshot({ path: 'screenshots/console/Create-001/01-create-page.png', fullPage: true });

    await page.waitForLoadState('domcontentloaded');

    const emailInput = page.getByRole('textbox').first();
    await expect(emailInput).toBeVisible({ timeout: timeouts.visible });
    await emailInput.fill('jesuswrites20043@gmail.com');
    await page.screenshot({ path: 'screenshots/console/Create-001/02-email-filled.png', fullPage: true });

    const authButton = page.getByRole('button', { name: "Send Authentication Number" });
    await expect(authButton).toBeVisible({ timeout: timeouts.visible });
    await authButton.click();
    await page.screenshot({ path: 'screenshots/console/Create-001/03-auth-requested.png', fullPage: true });

    const authNumberInput = page.getByRole('textbox').nth(1);
    await expect(authNumberInput).toBeVisible({ timeout: timeouts.visible });
    await authNumberInput.fill('123456');
    await page.screenshot({ path: 'screenshots/console/Create-001/04-auth-filled.png', fullPage: true });

    const inputPassword = page.getByRole('textbox').nth(2);
    await expect(inputPassword).toBeVisible({ timeout: timeouts.visible });
    await inputPassword.fill('12345678a#');
    await page.screenshot({ path: 'screenshots/console/Create-001/05-password-filled.png', fullPage: true });

    const confirmPassword = page.getByRole('textbox').nth(3);
    await expect(confirmPassword).toBeVisible({ timeout: timeouts.visible });
    await confirmPassword.fill('12345678a#');
    await page.screenshot({ path: 'screenshots/console/Create-001/06-confirm-password-filled.png', fullPage: true });

    const createButton = page.getByRole('button', { name: "Membership Registration" });
    await expect(createButton).toBeVisible({ timeout: timeouts.visible });
    await createButton.click();
    await page.screenshot({ path: 'screenshots/console/Create-001/07-registration-clicked.png', fullPage: true });

    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);

    await expect(page).toHaveURL(/.*/, { timeout: timeouts.url });
    await page.screenshot({ path: 'screenshots/console/Create-001/08-success.png', fullPage: true });
  });

  test('[Create-002] Validate Empty Field Errors', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/create');
    await page.screenshot({ path: 'screenshots/console/Create-002/01-create-page.png', fullPage: true });

    const authButton = page.getByRole('button', { name: "Send Authentication Number" });
    await expect(authButton).toBeVisible({ timeout: timeouts.visible });
    await authButton.click();
    await page.screenshot({ path: 'screenshots/console/Create-002/02-email-button-error.png', fullPage: true });

    const emailError = page.getByText('The email format is incorrect.');
    await expect(emailError).toBeVisible();

    const createButton = page.getByRole('button', { name: "Membership Registration" });
    await expect(createButton).toBeVisible({ timeout: timeouts.visible });
    await createButton.click();
    await page.screenshot({ path: 'screenshots/console/Create-002/03-other-validation-errors.png', fullPage: true });

    const authError = page.getByText('Authentication number is required');
    await expect(authError).toBeVisible();

    const passwordError = page.getByText('Please enter your password.');
    await expect(passwordError).toBeVisible();
  });
});
