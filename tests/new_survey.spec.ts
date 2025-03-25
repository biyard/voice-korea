import { test, expect } from '@playwright/test';

const timeouts = {
  wait: parseInt(process.env.WAIT_TIMEOUT || "2000", 10),
  visible: parseInt(process.env.VISIBLE_TIMEOUT || "5000", 10),
  url: parseInt(process.env.URL_TIMEOUT || "7000", 10)
};



test.describe('New Survey Page', () => {

  test('[Survey-003] Verify Fields, Errors, and Interactions', async ({ page }) => {
    await page.goto('https://voice-korea.dev.biyard.co/en');
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

    await expect(page).toHaveURL("https://voice-korea.dev.biyard.co/en/surveys", { timeout: timeouts.url });
    await page.screenshot({ path: 'screenshots/Survey-001/05-survey-page.png', fullPage: true });

    const startSurveyButton = page.getByRole('link', { name: "Start Survey" });
    await expect(startSurveyButton).toBeVisible();
    await startSurveyButton.click();
    await page.screenshot({ path: 'screenshots/Survey-001/06-survey-started.png', fullPage: true });

    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(timeouts.wait);

    await expect(page).toHaveURL('https://voice-korea.dev.biyard.co/en/surveys/new', { timeout: timeouts.url });
    await page.screenshot({ path: 'screenshots/Survey-001/07-survey-questions.png', fullPage: true });

    await page.goto('https://voice-korea.dev.biyard.co/en/surveys/new');
    await page.screenshot({ path: 'screenshots/Survey-003/01-new-survey-page.png', fullPage: true });

    const back = page.getByRole('link').filter({ hasText: /^$/ })
    await expect(back).toBeVisible();

    const categoryDropdown = await page.locator('div').filter({ hasText: /^Economy Society City Technology Health Politics Labor$/ }).getByRole('combobox');
    await expect(categoryDropdown).toBeVisible();

    const titleInput = page.getByRole('textbox', { name: 'Please enter a title' });
    await expect(titleInput).toBeVisible();

    const startDatePicker = page.getByRole('button', { name: "/03/24" })
    await expect(startDatePicker).toBeVisible();

    const endDatePicker = page.getByRole('button', { name: "/03/25" })
    await expect(endDatePicker).toBeVisible();

    const descriptionInput = page.getByRole('textbox', { name: 'Please enter a description' });
    await expect(descriptionInput).toBeVisible();

    await page.screenshot({ path: 'screenshots/Survey-003/02-all-fields-visible.png', fullPage: true });


    const errorMessages = [
      'Input introduction is required.',
      'Title is required.',
      'Start date is required.',
      'End date is required.',
      'Description is required.'
    ];

    for (const errorMessage of errorMessages) {
      await expect(page.locator(`text=${errorMessage}`)).toBeVisible();
    }

    await titleInput.fill('Economic Growth Survey');
    await startDatePicker.fill('2025/03/25');
    await endDatePicker.fill('2025/03/26');
    await descriptionInput.fill('A survey to analyze economic trends.');
    await page.screenshot({ path: 'screenshots/Survey-003/04-fields-filled.png', fullPage: true });

    const addQuestionButton = page.getByRole('button', { name: 'Please add a new question.' });
    await expect(addQuestionButton).toBeVisible();
    await addQuestionButton.click();
    await page.screenshot({ path: 'screenshots/Survey-003/05-question-added.png', fullPage: true });

    const questionTypeInput = page.locator('div').filter({ hasText: /^Single ChoiceMultiple ChoiceShort AnswerSubjective$/ }).getByRole('combobox')
    await expect(questionTypeInput).toBeVisible();
    

    const questionTitleInput = page.getByPlaceholder('Please enter a title.');
    await expect(questionTitleInput).toBeVisible();

    const questionDescriptionInput = page.getByPlaceholder('Please enter a description');
    await expect(questionDescriptionInput).toBeVisible();

    await page.waitForLoadState('networkidle');
    await page.screenshot({ path: 'screenshots/Survey-003/06-survey-submitted.png', fullPage: true });

    // await expect(page).toHaveURL(/.*surveys$/, { timeout: timeouts.url });
  });

});
