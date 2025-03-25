// import { test, expect } from '@playwright/test';

// const timeouts = {
//     wait: parseInt(process.env.WAIT_TIMEOUT || "5000", 10),
//     visible: parseInt(process.env.VISIBLE_TIMEOUT || "10000", 10),
//     url: parseInt(process.env.URL_TIMEOUT || "15000", 10)
// };

// test('Login and extract actual elements using getByRole & getByText', async ({ page }) => {
//     await page.goto('https://voice-korea.dev.biyard.co/en/');
//     await page.screenshot({ path: 'screenshots/Survey-001/01-login-page.png', fullPage: true });

//     await page.waitForLoadState('domcontentloaded');

//     const emailInput = page.getByRole('textbox', { name: "Email" });
//     await expect(emailInput).toBeVisible({ timeout: timeouts.visible });
//     await emailInput.fill('jesuswrites20043@gmail.com');
//     await page.screenshot({ path: 'screenshots/Survey-001/02-email-filled.png', fullPage: true });

//     const passwordInput = page.getByRole('textbox', { name: "Password" }); 
//     await expect(passwordInput).toBeVisible({ timeout: timeouts.visible });
//     await passwordInput.fill('12345678a#');
//     await page.screenshot({ path: 'screenshots/Survey-001/03-password-filled.png', fullPage: true });

//     const loginButton = page.getByRole('button', { name: "Login" });
//     await expect(loginButton).toBeVisible({ timeout: timeouts.visible });
//     await loginButton.click();
//     await page.screenshot({ path: 'screenshots/Survey-001/04-login-clicked.png', fullPage: true });

//     await page.waitForLoadState('networkidle');
//     await page.waitForTimeout(timeouts.wait);

//     await page.goto('https://voice-korea.dev.biyard.co/en/surveys/new');

//     const elements = await page.evaluate(() => {
//         const roles = ["button", "link", "textbox", "combobox", "checkbox", "radio"];
//         return roles.map(role => ({
//             role,
//             elements: Array.from(document.querySelectorAll('*'))
//                 .filter(el => el.getAttribute('role') === role)
//                 .map(el => el.lookupPrefix || el.getAttribute('aria-label') || 'Unnamed')
//         }));
//     });

//     const set_elements = {
//         buttons: await page.locator('button').allInnerTexts(),
//         links: await page.locator('a').allInnerTexts(),
//         textboxes: await page.locator('textbox').allInnerTexts(),
//         checkboxes: await page.locator('link"]').allInnerTexts(),
//         radios: await page.locator('combobox"]').allInnerTexts(),
//         combos: await page.locator('select').allInnerTexts(),
//     };

//     console.log('✅ Extracted Elements:', JSON.stringify(elements, null, 2));

//     console.log('✅ Extracted Elements:', JSON.stringify(set_elements, null, 2));

//     expect(elements.length).toBeGreaterThan(0);
// });
