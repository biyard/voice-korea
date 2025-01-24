import { test, expect } from "@playwright/test";
import { chromium } from "playwright";
import {
  latency,
  email,
  password,
  survey_title,
  screenshot_path,
  longlatency,
} from "./constants";
import path from "path";

test.describe("Survey", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("https://voice-korea.dev.biyard.co/");
    await page.waitForTimeout(latency);
  });

  test("[SURVEY-001] create-a-title", async ({ page }) => {
    const timestamp = Math.floor(Date.now() / 1000);
    const survey_title = `Test ${timestamp}`;
    await page.getByPlaceholder("이메일").fill(email);
    await page.waitForTimeout(latency);
    await page.getByPlaceholder("비밀번호").fill(password);
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-title",
        "1-fill-the-login-form"
      ),
      fullPage: true,
    });
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[1]/div[1]/div[2]/div/div[2]/div/div'
      )
      .click();
    await page.waitForLoadState();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-title",
        "2-check-the-login-status"
      ),
      fullPage: true,
    });
    await page.getByText("설문 만들기").click();
    await page.waitForTimeout(latency);
    await page
      .getByPlaceholder("설문지의 제목을 입력해주세요.", { exact: true })
      .fill(survey_title);
    await page.waitForTimeout(latency);
    await page.getByText("임시 저장").click();
    await page.waitForTimeout(latency);
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-title",
        "3-fill-the-survey-title"
      ),
      fullPage: true,
    });
    await page.getByText("취소").click();
    await page.locator(`text=${survey_title}`).waitFor();
    const textToCheck = survey_title;
    const textLocator = page.locator(`text=${textToCheck}`);
    await expect(textLocator).toBeVisible();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-title",
        "4-check-the-survey-title"
      ),
      fullPage: true,
    });
  });

  test("[SURVEY-002] create-a-question", async ({ page }) => {
    const timestamp = Math.floor(Date.now() / 1000);
    const survey_title = `Test ${timestamp}`;
    await page.getByPlaceholder("이메일").fill(email);
    await page.waitForTimeout(latency);
    await page.getByPlaceholder("비밀번호").fill(password);
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-question",
        "1-fill-the-login-form"
      ),
      fullPage: true,
    });
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[1]/div[1]/div[2]/div/div[2]/div/div'
      )
      .click();
    await page.waitForLoadState();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-question",
        "2-check-the-login-status"
      ),
      fullPage: true,
    });
    await page.getByText("설문 만들기").click();
    await page.waitForTimeout(latency);
    await page
      .getByPlaceholder("설문지의 제목을 입력해주세요.", { exact: true })
      .fill(survey_title);
    await page.waitForTimeout(latency);
    await page.getByText("임시 저장").click();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-question",
        "3-fill-the-survey-title"
      ),
      fullPage: true,
    });
    await page
      .getByPlaceholder("설문지의 제목을 입력해주세요.", { exact: true })
      .fill(survey_title);
    await page.waitForTimeout(latency);
    await page.getByText("저장", { exact: true }).click();
    await page.locator(`text=질문 추가하기`).waitFor();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-question",
        "4-question-make-page"
      ),
      fullPage: true,
    });
    await page.getByText("질문 추가하기").click();
    await page.waitForTimeout(latency);
    await page
      .getByPlaceholder("질문을 입력하세요.")
      .fill("Multiple choice test");
    await page.getByPlaceholder("옵션 1").fill("Yes");
    await page.getByPlaceholder("옵션 2").fill("No");
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-question",
        "5-fill-the-question-and-answer"
      ),
      fullPage: true,
    });
    await page.getByText("저장").click();
    await page.waitForTimeout(latency);
    await page.locator(`text=Multiple choice test`).waitFor();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "create-a-question",
        "6-check-the-question"
      ),
      fullPage: true,
    });
    await page.locator(`text=Multiple choice test`).waitFor();
    const textLocator = page.locator(`text=Multiple choice test`);
    await expect(textLocator).toBeVisible();
  });

  test("[SURVEY-003] set-a-response-properties", async ({ page }) => {
    const timestamp = Math.floor(Date.now() / 1000);
    const survey_title = `Test ${timestamp}`;
    await page.getByPlaceholder("이메일").fill(email);
    await page.waitForTimeout(latency);
    await page.getByPlaceholder("비밀번호").fill(password);
    await page.screenshot({
      path: screenshot_path(
        "account",
        "set-a-response-properties",
        "1-fill-the-login-form"
      ),
      fullPage: true,
    });
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[1]/div[1]/div[2]/div/div[2]/div/div'
      )
      .click();
    await page.waitForLoadState();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "set-a-response-properties",
        "2-check-the-login-status"
      ),
      fullPage: true,
    });
    await page.getByText("설문 만들기").click();
    await page.waitForTimeout(latency);
    await page
      .getByPlaceholder("설문지의 제목을 입력해주세요.", { exact: true })
      .fill(survey_title);
    await page.waitForTimeout(latency);
    await page.screenshot({
      path: screenshot_path(
        "account",
        "set-a-response-properties",
        "3-fill-the-survey-title"
      ),
      fullPage: true,
    });
    await page
      .getByPlaceholder("설문지의 제목을 입력해주세요.", { exact: true })
      .fill(survey_title);
    await page.waitForTimeout(latency);
    await page.getByText("저장", { exact: true }).click();
    await page.locator("text=질문 추가하기").waitFor();
    await page.getByText("저장", { exact: true }).click();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "set-a-response-properties",
        "4-add-response-properties-page"
      ),
      fullPage: true,
    });
    await page.getByText("속성 추가하기").click();
    await page.waitForTimeout(latency);
    await page.locator(`text=응답 속성 선택`).waitFor();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "set-a-response-properties",
        "5-set-a-response-properties-page"
      ),
      fullPage: true,
    });
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[2]/div/div/div[2]/div/div[3]/div/div[2]/input'
      )
      .click();
    await page.waitForTimeout(latency);
    await page.getByText("속성 추가하기").click();
    await page.locator(`text=검색 결과`).waitFor();
    await page.getByText("연령").click();
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[2]/div/div[1]/div/div/div[1]/div[4]/div'
      )
      .click();
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[2]/div/div[1]/div/div/div[2]/div/div[2]'
      )
      .click();
    await page.locator(`text=설문 대상자의 속성 선택`).waitFor();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "set-a-response-properties",
        "6-set-a-response-properties"
      ),
      fullPage: true,
    });
    await page.getByText("저장", { exact: true }).click();
    await page.waitForTimeout(latency);
    await page.locator("text=설문 목록").waitFor();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "set-a-response-properties",
        "7-set-a-response-properties-success"
      ),
      fullPage: true,
    });
  });
});
