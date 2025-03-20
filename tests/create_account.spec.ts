import { test, expect } from '@playwright/test';

test.describe('Create Account Flow - Step 3 & Step 4', () => {

    test.beforeEach(async ({ page }) => {
        await page.goto('http://127.0.0.1:8080/create');
    });

    // Step 3
    test('Step 3: Should render authentication form correctly', async ({ page }) => {
        await expect(page.getByText('Join the Membership')).toBeVisible();
        await expect(page.getByLabel('Email Address')).toBeVisible();
        await expect(page.getByRole('button', { name: 'Send Authentication Number' })).toBeVisible();
    });

    test('Step 3: Should validate email field', async ({ page }) => {
        await page.getByLabel('Email Address').fill('invalid-email');
        await page.getByRole('button', { name: 'Send Authentication Number' }).click();
        await expect(page.getByText('The email format is incorrect.')).toBeVisible();
    });

    test('Step 3: Should allow entering authentication number', async ({ page }) => {
        await page.getByLabel('Authentication Number').fill('123456');
        await expect(page.getByLabel('Authentication Number')).toHaveValue('123456');
    });

    test('Step 3: Should validate password field', async ({ page }) => {
        await page.getByLabel('Input Password').fill('short');
        await expect(page.getByText('Please enter your password.')).toBeVisible();
    });

    test('Step 3: Should match password confirmation', async ({ page }) => {
        await page.getByLabel('Input Password').fill('StrongPassword123!');
        await page.getByLabel('Check Password').fill('WrongPassword123!');
        await expect(page.getByText('The two passwords do not match.')).toBeVisible();
    });

    test('Step 3: Should proceed to Step 4 after successful authentication', async ({ page }) => {
        await page.getByLabel('Email Address').fill('test@example.com');
        await page.getByRole('button', { name: 'Send Authentication Number' }).click();
        await page.getByLabel('Authentication Number').fill('123456');
        await page.getByLabel('Input Password').fill('StrongPassword123!');
        await page.getByLabel('Check Password').fill('StrongPassword123!');
        await page.getByRole('button', { name: 'Membership Registration Completed' }).click();

        await expect(page).toHaveURL('http://127.0.0.1:8080/create');
    });

    // Step 4
    test('Step 4: Should display complete registration form', async ({ page }) => {
        await page.goto('http://127.0.0.1:8080/create');

        await expect(page.getByText('Congratulations on completing your membership registration.')).toBeVisible();
        await expect(page.getByLabel('Email Address')).toBeVisible();
        await expect(page.getByLabel('Name')).toBeVisible();
        await expect(page.getByLabel('Cellphone')).toBeVisible();
        await expect(page.getByLabel('Company Name')).toBeVisible();
        await expect(page.getByRole('button', { name: 'Complete Registration' })).toBeVisible();
    });

    test('Step 4: Should validate required fields before submission', async ({ page }) => {
        await page.goto('http://127.0.0.1:8080/create');

        await page.getByRole('button', { name: 'Complete Registration' }).click();
        await expect(page.getByText('Please enter your email address.')).toBeVisible();
        await expect(page.getByText('Please enter your full name.')).toBeVisible();
        await expect(page.getByText('Please enter your phone number.')).toBeVisible();
    });

    test('Step 4: Should complete registration successfully', async ({ page }) => {
        await page.goto('http://127.0.0.1:8080/create');

        await page.getByLabel('Email Address').fill('user@example.com');
        await page.getByLabel('Full Name').fill('John Doe');
        await page.getByLabel('Cellphone').fill('+1234567890');
        await page.getByLabel('Corporation Name').fill('Biyard Co');
        await page.getByRole('button', { name: 'Complete' }).click();

        await expect(page.getByText('Account successfully created')).toBeVisible();
        await expect(page).toHaveURL('http://127.0.0.1:8080/');
    });

});
