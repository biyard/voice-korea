import { test, expect } from '@playwright/test';

const devMobileViewports = [
    { device: 'iPhone 12', width: 390, height: 844 },
    { device: 'Samsung Galaxy S20', width: 412, height: 915 },
    { device: 'Pixel 5', width: 393, height: 851 },
    { device: 'iPad Mini', width: 768, height: 1024 },
];

test.describe('UI Mobile Responsiveness & Individual Project Page Tests', () => {
    for (const viewport of devMobileViewports) {
        test(`[Project-ID-001] Check responsiveness on ${viewport.device} (${viewport.width}x${viewport.height})`, async ({ page }) => {
            await page.setViewportSize({ width: viewport.width, height: viewport.height });
            const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
            await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);

            const bodyOverflowX = await page.evaluate(() => document.documentElement.scrollWidth > document.documentElement.clientWidth);
            expect(bodyOverflowX).toBeFalsy();

            await page.screenshot({ path: `screenshots/ui-${viewport.device}.png`, fullPage: true });
        });
    }

    test('[Project-ID-002] Confirm it has project details', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-002/01-page-entered.png`, fullPage: true });

        const projectImage = page.getByRole('img', { name: 'Header Section Image' });
        await expect(projectImage).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-002/02-project-image.png`, fullPage: true });

        const projectDuration = page.getByText('월 26일 2025년 ~ 4월 24일 2025년');
        await expect(projectDuration).toBeVisible();

        const projectTitle = page.getByText('full test survey').first();
        await expect(projectTitle).toBeVisible();

        const projectCategory = page.locator('div').filter({ hasText: /^Economy$/ }).nth(1);
        await expect(projectCategory).toBeVisible();

        const projectParticipants = await page.getByText('participant', { exact: true });
        await expect(projectParticipants).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-002/03-project-participants.png`, fullPage: true });

        const projectVotes = page.getByText('Vote');
        await expect(projectVotes).toBeVisible();

        const input = page.getByRole('textbox', { name: 'Leave a reply...' });
        await expect(input).toBeVisible();
        await input.click();
        await input.fill('Test comment');
        await page.keyboard.press('Enter');
        await page.screenshot({ path: 'screenshots/users/project-id-002/04-project-input.png' });
    });

    test('[Project-ID-003] Confirm All Tabs Exist', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-003/01-page-entered.png`, fullPage: true });

        const basicInfo = page.locator('div').filter({ hasText: /^Basic Info$/ }).nth(1);
        await expect(basicInfo).toBeVisible();

        const sampleSurvey = page.getByRole('paragraph').filter({ hasText: 'Sample Survey' });
        await expect(sampleSurvey).toBeVisible();

        const deliberation = page.getByText('Deliberation', { exact: true });
        await expect(deliberation).toBeVisible();

        const discussion = page.locator('div').filter({ hasText: /^Discussion$/ }).nth(1);
        await expect(discussion).toBeVisible();

        const finalSurvey = page.getByText('Final Survey');
        await expect(finalSurvey).toBeVisible();

        const finalDraft = page.getByText('Final Draft');
        await expect(finalDraft).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-003/02-project-tabs.png`, fullPage: true });
    });

    test('[Project-ID-004] Test Forms And Their Labels, Section-01', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-004/01-page-entered.png`, fullPage: true });

        const sectionOne = page.getByRole('paragraph').filter({ hasText: 'Basic Info' });
        await expect(sectionOne).toBeVisible();
        await sectionOne.click()
        await page.screenshot({ path: `screenshots/users/project-id-004/02-basic-info.png`, fullPage: true });

        const header = page.locator('#basic-info').getByText('Basic Info')
        await expect(header).toBeVisible();

        const innerDropdown = page.getByText('Introduction');
        await expect(innerDropdown).toBeVisible();

        const relatedMaterials = page.getByText('Related materials')
        await expect(relatedMaterials).toBeVisible();

        const pdfMaterial = page.locator('div').filter({ hasText: /\.pdf$/ }).nth(1);
        await expect(pdfMaterial).toBeVisible();
        await pdfMaterial.click();
        const downloadPromise = page.waitForEvent('download');
        const download = await downloadPromise;
        expect(download.suggestedFilename()).toMatch(/\.pdf$/);
        await page.screenshot({ path: `screenshots/users/project-id-004/03-pdf-material.png`, fullPage: true });
    });

    test('[Project-ID-005] Test Forms And Their Labels, Section-02', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-005/01-page-entered.png`, fullPage: true });


        const sampleSurvey = page.locator('div').filter({ hasText: /^Sample Survey$/ }).nth(1)
        await expect(sampleSurvey).toBeVisible()
        await sampleSurvey.click()
        await page.screenshot({ path: `screenshots/users/project-id-005/02-sample-survey.png`, fullPage: true });

        const sampleSurveyHeader = page.locator('#sample-survey').getByText('Sample Survey', { exact: true })
        await expect(sampleSurveyHeader).toBeVisible();

        const sampleSurveyDescription = page.locator('div').filter({ hasText: /^Sample Survey Title$/ }).nth(3)
        await expect(sampleSurveyDescription).toBeVisible();

        const sampleSurveyButton = page.getByText('Take part in the survey')
        await expect(sampleSurveyButton).toBeVisible();
        await sampleSurveyButton.click();

        const testDiv = page.locator('#sample-survey > div > div').first()
        await expect(testDiv).toBeVisible();
        const testDivText = await testDiv.textContent()
        expect(testDivText).toContain('test1')

        const surveyInputOne = page.getByRole('textbox', { name: 'Please Enter Details' }).first()
        await expect(surveyInputOne).toBeVisible();
        await surveyInputOne.click()
        await surveyInputOne.fill('Test input 1')
        await page.screenshot({ path: `screenshots/users/project-id-005/03-survey-input.png`, fullPage: true });

        const surveyInputTwo = page.getByRole('textbox', { name: 'Please Enter Details' }).nth(1)
        await expect(surveyInputTwo).toBeVisible();
        await surveyInputTwo.click()
        await surveyInputTwo.fill('Test input 2')
        await page.screenshot({ path: `screenshots/users/project-id-005/04-survey-input.png`, fullPage: true });

        const surveyCheckBoxOne = page.locator('label > .w-\\[24px\\]').first()
        await expect(surveyCheckBoxOne).toBeVisible();
        await surveyCheckBoxOne.click()

        const surveyCheckBoxTwo = page.locator('div:nth-child(2) > label > .w-\\[24px\\]').first()
        await expect(surveyCheckBoxTwo).toBeVisible();
        await surveyCheckBoxTwo.click()

        const surveyCheckBoxThree = page.locator('div:nth-child(5) > div:nth-child(3) > div > label > .w-\\[24px\\]').first()
        await expect(surveyCheckBoxThree).toBeVisible();
        await surveyCheckBoxThree.click()

        const surveyCheckBoxFour =  page.locator('div:nth-child(5) > div:nth-child(3) > div:nth-child(2) > label > .w-\\[24px\\]')
        await expect(surveyCheckBoxFour).toBeVisible();
        await surveyCheckBoxFour.click()
        await page.screenshot({ path: `screenshots/users/project-id-005/05-survey-checkbox.png`, fullPage: true });

        const submitButton = page.getByText('Submit')
        await expect(submitButton).toBeVisible();
        await submitButton.click()
        await page.screenshot({ path: `screenshots/users/project-id-005/06-survey-submit.png`, fullPage: true });

    });

    test('[Project-ID-006] Validations, Section-02', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-006/01-page-entered.png`, fullPage: true });


        const sampleSurvey = page.getByText('Sample Survey')
        await expect(sampleSurvey).toBeVisible()
        await sampleSurvey.click()
        await page.screenshot({ path: `screenshots/users/project-id-006/02-sample-survey.png`, fullPage: true });

        const sampleSurveyHeader = page.locator('#sample-survey').getByText('Sample Survey', { exact: true })
        await expect(sampleSurveyHeader).toBeVisible();

        const sampleSurveyButton = page.getByText('Take part in the survey')
        await expect(sampleSurveyButton).toBeVisible();
        await sampleSurveyButton.click();

        const submitButton = page.getByText('Submit')
        await expect(submitButton).toBeVisible();
        await submitButton.click()
        await page.screenshot({ path: `screenshots/users/project-id-006/03-survey-submit.png`, fullPage: true });

        const checkBoxErrorMessage = page.getByText('Please select at least one option')
        await expect(checkBoxErrorMessage).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-006/04-survey-checkbox-error.png`, fullPage: true });

        const inputErrorMessage = page.getByText('Please fill out this field')
        await expect(inputErrorMessage).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-006/05-survey-input-error.png`, fullPage: true });
    })

    test('[Project-ID-007] Test Forms And Their Labels, Section-03', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-007/01-page-entered.png`, fullPage: true });

        const deliberation = page.getByRole('paragraph').filter({ hasText: 'Deliberation' })
        await expect(deliberation).toBeVisible();
        await deliberation.click()
        await page.screenshot({ path: `screenshots/users/project-id-007/02-deliberation.png`, fullPage: true });

        const deliberationHeader = page.locator('#deliberation').getByText('Deliberation')
        await expect(deliberationHeader).toBeVisible();

        const deliberationDescription = page.locator('div').filter({ hasText: /^Deliberation Title$/ }).nth(3)
        await expect(deliberationDescription).toBeVisible();

        const deliberationMaterials = page.getByText('Deliberation materials')
        await expect(deliberationMaterials).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-007/03-deliberation-materials.png`, fullPage: true });
    })

    test('[Project-ID-008] Test Forms And Their Labels, Section-04', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-008/01-page-entered.png`, fullPage: true });

        const discussion = page.getByRole('paragraph').filter({ hasText: 'Discussion' })
        await expect(discussion).toBeVisible();
        await discussion.click()
        await page.screenshot({ path: `screenshots/users/project-id-008/02-discussion.png`, fullPage: true });

        const discussionHeader = page.locator('#discussion').getByText('Discussion', { exact: true })
        await expect(discussionHeader).toBeVisible();

        const firstDiscussionTab = page.locator('div').filter({ hasText: /^Highlights$/ }).nth(2)
        await expect(firstDiscussionTab).toBeVisible();
        await firstDiscussionTab.click()
        await page.screenshot({ path: `screenshots/users/project-id-008/03-discussion-highlights.png`, fullPage: true });

        const secondDiscussionTab = page.locator('div').filter({ hasText: /^Video Discussion$/ }).nth(2)
        await expect(secondDiscussionTab).toBeVisible();
        await secondDiscussionTab.click()
        await page.screenshot({ path: `screenshots/users/project-id-008/04-discussion-video.png`, fullPage: true });

        const relatedMaterials = page.getByText('Related materials')
        await expect(relatedMaterials).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-008/05-discussion-materials.png`, fullPage: true });
    })

    test('[Project-ID-009] Test Forms And Their Labels, Section-05', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-009/01-page-entered.png`, fullPage: true });

        const finalSurvey = page.locator('#final-survey').getByText('Final Survey')
        await expect(finalSurvey).toBeVisible();
        await finalSurvey.click()
        await page.screenshot({ path: `screenshots/users/project-id-009/02-final-survey.png`, fullPage: true });

        const finalSurveyHeader = page.getByText('Final Survey4월 5일 2025년 ~ 4월')
        await expect(finalSurveyHeader).toBeVisible();

        const finalVoteTopic = page.getByText('Final Vote Topic')
        await expect(finalVoteTopic).toBeVisible();

        const finalSurveyButton = page.getByText('Take part in the vote.')
        await expect(finalSurveyButton).toBeVisible();
        await finalSurveyButton.click();
        await page.screenshot({ path: `screenshots/users/project-id-009/03-final-survey-button.png`, fullPage: true });

        const finalSurveyInput = page.getByRole('textbox', { name: 'Please Enter Details' }).first()
        await expect(finalSurveyInput).toBeVisible();
        await finalSurveyInput.click()
        await finalSurveyInput.fill('Test input 1')
        await page.screenshot({ path: `screenshots/users/project-id-009/04-final-survey-input.png`, fullPage: true });

        const secondFinalSurveyInput = page.getByRole('textbox', { name: 'Please Enter Details' }).nth(1)
        await expect(secondFinalSurveyInput).toBeVisible();
        await secondFinalSurveyInput.click()
        await secondFinalSurveyInput.fill('Test input 2')

        const finalSurveyCheckBoxOne = page.locator('.w-\\[24px\\]').first()
        await expect(finalSurveyCheckBoxOne).toBeVisible();
        await finalSurveyCheckBoxOne.click()

        const finalSurveyCheckBoxTwo =  page.locator('div:nth-child(2) > label > .w-\\[24px\\]').first()
        await expect(finalSurveyCheckBoxTwo).toBeVisible();
        await finalSurveyCheckBoxTwo.click()

        const finalSurveyCheckBoxThree =  page.locator('div:nth-child(5) > div:nth-child(3) > div > label > .w-\\[24px\\]').first()
        await expect(finalSurveyCheckBoxThree).toBeVisible();
        await finalSurveyCheckBoxThree.click()

        const finalSurveyCheckBoxFour = page.locator('div:nth-child(5) > div:nth-child(3) > div:nth-child(2) > label > .w-\\[24px\\]')
        await expect(finalSurveyCheckBoxFour).toBeVisible();
        await finalSurveyCheckBoxFour.click()
        await page.screenshot({ path: `screenshots/users/project-id-009/05-final-survey-checkbox.png`, fullPage: true });

        const finalSurveySubmitButton = page.getByText('Submit')
        await expect(finalSurveySubmitButton).toBeVisible();
        await finalSurveySubmitButton.click()
        await page.screenshot({ path: `screenshots/users/project-id-009/06-final-survey-submit.png`, fullPage: true });

        const finalSurveyPopup = page.locator('div').filter({ hasText: 'Please check again before voting.Voting is anonymous and once a vote is' }).nth(2);
        await expect(finalSurveyPopup).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-009/07-final-survey-popup.png`, fullPage: true });

        const cancelPopup = page.getByText('Cancel')
        await expect(cancelPopup).toBeVisible();
        await cancelPopup.click()
        await page.screenshot({ path: `screenshots/users/project-id-009/08-final-survey-cancel.png`, fullPage: true });

        await finalSurveySubmitButton.click()
        await page.screenshot({ path: `screenshots/users/project-id-009/09-final-survey-submit-again.png`, fullPage: true });

        const finalSurveyConfirmButton = page.getByText('Complete Voting')
        await expect(finalSurveyConfirmButton).toBeVisible();
        await finalSurveyConfirmButton.click()
        await page.screenshot({ path: `screenshots/users/project-id-009/10-final-survey-confirm.png`, fullPage: true });

    })

    test('[Project-ID-010] Validations, Section-05', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-010/01-page-entered.png`, fullPage: true });


        const finalSurvey = page.getByText('Final Survey')
        await expect(finalSurvey).toBeVisible()
        await finalSurvey.click()
        await page.screenshot({ path: `screenshots/users/project-id-010/02-final-survey.png`, fullPage: true });

        const finalSurveyHeader = page.locator('#sample-survey').getByText('Sample Survey', { exact: true })
        await expect(finalSurveyHeader).toBeVisible();

        const finalSurveyButton = page.getByText('Take part in the vote.')
        await expect(finalSurveyButton).toBeVisible();
        await finalSurveyButton.click();
        await page.screenshot({ path: `screenshots/users/project-id-010/03-final-survey-button.png`, fullPage: true });

        const submitButton = page.getByText('Submit')
        await expect(submitButton).toBeVisible();
        await submitButton.click()
        await page.screenshot({ path: `screenshots/users/project-id-010/04-final-survey-submit.png`, fullPage: true });

        const checkBoxErrorMessage = page.getByText('Please select at least one option')
        await expect(checkBoxErrorMessage).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-010/05-final-survey-checkbox-error.png`, fullPage: true });

        const inputErrorMessage = page.getByText('Please fill out this field')
        await expect(inputErrorMessage).toBeVisible();
        await page.screenshot({ path: `screenshots/users/project-id-010/06-final-survey-input-error.png`, fullPage: true });
    })

    test('[Project-ID-011] Test Forms And Their Labels, Section-06', async ({ page }) => {
        const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
        await page.goto(`https://dev.voice-korea.com/en/projects/${randomProjectNumber}`);
        await page.screenshot({ path: `screenshots/users/project-id-011/01-page-entered.png`, fullPage: true });

        const finalDraft = page.getByRole('paragraph').filter({ hasText: 'Final Draft' })
        await expect(finalDraft).toBeVisible();
        await finalDraft.click()
        await page.screenshot({ path: `screenshots/users/project-id-011/02-final-draft.png`, fullPage: true });
    })
});
