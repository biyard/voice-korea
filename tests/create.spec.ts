import { test, expect } from '@playwright/test';

test.describe('Create Page Flow', () => {
  test('[Create-001] Successful Account Creation', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/create');
    await page.screenshot({ path: 'screenshots/Create-001/01-create-page.png', fullPage: true });

    await page.waitForLoadState('domcontentloaded');

    const emailInput = page.getByRole('textbox').first();
    await expect(emailInput).toBeVisible({ timeout: 10000 });
    await emailInput.fill('jesuswrites20043@gmail.com');
    await page.screenshot({ path: 'screenshots/Create-001/02-email-filled.png', fullPage: true });

    const authButton = page.getByRole('button', { name: "Send Authentication Number" });
    await expect(authButton).toBeVisible({ timeout: 10000 });
    await authButton.click();
    await page.screenshot({ path: 'screenshots/Create-001/03-auth-requested.png', fullPage: true });

    const authNumberInput = page.getByRole('textbox').nth(1);
    await expect(authNumberInput).toBeVisible({ timeout: 10000 });
    await authNumberInput.fill('123456');
    await page.screenshot({ path: 'screenshots/Create-001/04-auth-filled.png', fullPage: true });

    const inputPassword = page.getByRole('textbox').nth(2);
    await expect(inputPassword).toBeVisible({ timeout: 10000 });
    await inputPassword.fill('12345678a#');
    await page.screenshot({ path: 'screenshots/Create-001/05-password-filled.png', fullPage: true });

    const confirmPassword = page.getByRole('textbox').nth(3);
    await expect(confirmPassword).toBeVisible({ timeout: 10000 });
    await confirmPassword.fill('12345678a#');
    await page.screenshot({ path: 'screenshots/Create-001/06-confirm-password-filled.png', fullPage: true });

    const createButton = page.getByRole('button', { name: "Membership Registration" });
    await expect(createButton).toBeVisible({ timeout: 10000 });
    await createButton.click();
    await page.screenshot({ path: 'screenshots/Create-001/07-registration-clicked.png', fullPage: true });

    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(5000);

    await expect(page).toHaveURL(/.*/, { timeout: 15000 });
    await page.screenshot({ path: 'screenshots/Create-001/08-success.png', fullPage: true });
  });

  test('[Create-002] Validate Empty Field Errors', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/create');
    await page.screenshot({ path: 'screenshots/Create-002/01-create-page.png', fullPage: true });

    const authButton = page.getByRole('button', { name: "Send Authentication Number" });
    await expect(authButton).toBeVisible({ timeout: 10000 });
    await authButton.click();
    await page.screenshot({ path: 'screenshots/Create-002/02-email-button-error.png', fullPage: true });

    const emailError = page.getByText('The email format is incorrect.');
    await expect(emailError).toBeVisible();

    const createButton = page.getByRole('button', { name: "Membership Registration" });
    await expect(createButton).toBeVisible({ timeout: 10000 });
    await createButton.click();
    await page.screenshot({ path: 'screenshots/Create-002/03-other-validation-errors.png', fullPage: true });
    
    const authError = page.getByText('Authentication number is required');
    await expect(authError).toBeVisible();

    const passwordError = page.getByText('Please enter your password.');
    await expect(passwordError).toBeVisible();
  });
});
