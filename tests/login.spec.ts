import { test, expect } from '@playwright/test';

test.describe('Login Page', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://127.0.0.1:8080'); 
  });

  test('should render login page', async ({ page }) => {
    await expect(page.getByText('VOICE KOREA')).toBeVisible();
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Login' })).toBeVisible();
  });

  test('should allow a user to enter email and password', async ({ page }) => {
    await page.fill('input[type="email"]', 'testuser@example.com');
    await page.fill('input[type="password"]', 'SecurePass123');

    await expect(page.locator('input[type="email"]')).toHaveValue('testuser@example.com');
    await expect(page.locator('input[type="password"]')).toHaveValue('SecurePass123');
  });

  test('should display error for incorrect credentials', async ({ page }) => {
    await page.fill('input[type="email"]', 'wronguser@example.com');
    await page.fill('input[type="password"]', 'WrongPass123');
    await page.click('button:has-text("Login")');

    await expect(page.getByText('Login failed')).toBeVisible();
  });

  test('should login successfully with correct credentials', async ({ page }) => {
    await page.fill('input[type="email"]', 'validuser@example.com');
    await page.fill('input[type="password"]', 'ValidPass123');
    await page.click('button:has-text("Login")');

    await expect(page).toHaveURL("http://127.0.0.1:8080/surveys/"); 
  });
});
