import { test, expect } from '@playwright/test';

const timeouts = {
  wait: parseInt(process.env.WAIT_TIMEOUT || "5000", 10),
  visible: parseInt(process.env.VISIBLE_TIMEOUT || "10000", 10),
  url: parseInt(process.env.URL_TIMEOUT || "15000", 10)
};



test.describe('Deliberations Page Flow', () => {

  test('[Survey-001] Login and Start A Public Opinion', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');
    await page.screenshot({ path: 'screenshots/Survey-001/01-login-page.png', fullPage: true });

    await page.waitForLoadState('domcontentloaded');

    const emailInput = page.getByRole('textbox').first();
    await expect(emailInput).toBeVisible({ timeout: timeouts.visible});
    await emailInput.fill('jesuswrites20043@gmail.com');
    await page.screenshot({ path: 'screenshots/Survey-001/02-email-filled.png', fullPage: true });

    const passwordInput = page.getByRole('textbox').nth(1);
    await expect(passwordInput).toBeVisible({ timeout: timeouts.visible});
    await passwordInput.fill('12345678a#');
    await page.screenshot({ path: 'screenshots/Survey-001/03-password-filled.png', fullPage: true });

    const loginButton = page.getByRole('button', { name: "Login" });
    await expect(loginButton).toBeVisible({ timeout: timeouts.visible});
    await loginButton.click();
    await page.screenshot({ path: 'screenshots/Survey-001/04-login-clicked.png', fullPage: true });

    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);

    await expect(page).toHaveURL(/.*surveys/, { timeout: timeouts.url });
    await page.screenshot({ path: 'screenshots/Survey-001/05-survey-page.png', fullPage: true });

    await page.goto('https://voice-korea.dev.biyard.co/en/surveys');

    const goToDeliberationsButton = page.getByRole('link', { name: '공론 조사' })
    await expect(goToDeliberationsButton).toBeVisible();
    await goToDeliberationsButton.click();
    await page.screenshot({ path: 'screenshots/Survey-001/06-survey-started.png', fullPage: true });

    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);

    await expect(page).toHaveURL('https://voice-korea.dev.biyard.co/en/deliberations', { timeout: timeouts.url });
    await page.screenshot({ path: 'screenshots/Survey-001/07-survey-questions.png', fullPage: true });

  });

  test('[Survey-002] Login, Go to Deliberations, and Logout', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en/');
    await page.screenshot({ path: 'screenshots/Survey-002/01-login-page.png', fullPage: true });

    await page.waitForLoadState('domcontentloaded');

    const emailInput = page.getByRole('textbox').first();
    await expect(emailInput).toBeVisible({ timeout: timeouts.visible});
    await emailInput.fill('jesuswrites20043@gmail.com');
    await page.screenshot({ path: 'screenshots/Survey-002/02-email-filled.png', fullPage: true });

    const passwordInput = page.getByRole('textbox').nth(1);
    await expect(passwordInput).toBeVisible({ timeout: timeouts.visible});
    await passwordInput.fill('12345678a#');
    await page.screenshot({ path: 'screenshots/Survey-002/03-password-filled.png', fullPage: true });

    const loginButton = page.getByRole('button', { name: "Login" });
    await expect(loginButton).toBeVisible({ timeout: timeouts.visible});
    await loginButton.click();
    await page.screenshot({ path: 'screenshots/Survey-002/04-login-clicked.png', fullPage: true });

    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);

    await expect(page).toHaveURL("https://voice-korea.dev.biyard.co/en/deliberations", { timeout: timeouts.url });
    await page.screenshot({ path: 'screenshots/Survey-002/05-survey-page.png', fullPage: true });

    const logoutButton = page.getByRole('link', { name: "Logout" });
    await expect(logoutButton).toBeVisible();
    await logoutButton.click();
    await page.screenshot({ path: 'screenshots/Survey-002/06-logout-clicked.png', fullPage: true });

    await expect(page).toHaveURL('https://voice-korea.dev.biyard.co/en/', { timeout: timeouts.url });
    await page.screenshot({ path: 'screenshots/Survey-002/07-logged-out.png', fullPage: true });
  });

});
