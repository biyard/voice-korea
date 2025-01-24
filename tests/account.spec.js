import { test, expect } from "@playwright/test";
import { chromium } from "playwright";
import {
  latency,
  email,
  password,
  google_password,
  screenshot_path,
} from "./constants";
import path from "path";

test.describe.serial("Account", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("https://voice-korea.dev.biyard.co/");
    await page.waitForTimeout(latency);
  });

  test("[ACCOUNT-001] account-sign-up", async ({ page }) => {
    await page.goto("https://voice-korea.dev.biyard.co/");
    await page.waitForTimeout(latency);
    await page.getByText("회원가입").click();
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[2]/div[1]/div/div[2]/div[1]/input'
      )
      .fill(email);
    await page.waitForTimeout(latency);

    //이메일 인증번호 가져와서 입력하는 과정 추가
    const browser = await chromium.launch();
    const page1 = await browser.newPage();
    await page1.goto("https://www.google.co.kr/");
    await page1.waitForTimeout(latency);
    // await page1.getByText('로그인').first().click();
    // await page1.waitForTimeout(latency);
    // await page1.getByLabel('이메일 또는 휴대전화').fill(email);
    // await page1.getByText('다음').click();
    // await page1.waitForTimeout(latency);
    // await page1.getByLabel('비밀번호 입력').fill(google_password);
    // await page1.getByText('다음').click();
    // await page1.waitForTimeout(latency);
    await page1.getByText("Gmail").first().click();
    await page1.waitForTimeout(latency);
    await page1.locator('xpath=//*[@id=":2m"]').click();
    await page1.locator('xpath=//*[@id=":7g"]');
    const str = "";
    const match = str.match(/"([A-Za-z0-9]{6})"/);
    //여기까지가 메일에서 정보 가져오기

    //여기가 중간 지점
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[2]/div[2]/div/div[2]/div[1]/input'
      )
      .fill(match);

    //여기부터 다시 시작
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[2]/div[3]/div/div[2]/div/input'
      )
      .fill(password);
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[2]/div[4]/div/div[2]/div/input'
      )
      .fill(password);
    await page.screenshot({
      path: screenshot_path("account", "account-sign-up", "1-fill-the-form"),
      fullPage: true,
    });
    await page.getByText("회원가입 완료").click();
    await page.waitForNavigation();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "account-sign-up",
        "2-account-sign-up-success"
      ),
      fullPage: true,
    });
    await expect(page).toHaveURL("https://voice-korea.dev.biyard.co");
    console.log("Test passed: User successfully sign up.");
  });

  test("[ACCOUNT-002] account-login", async ({ page }) => {
    await page.goto("https://voice-korea.dev.biyard.co/");
    await page.getByPlaceholder("이메일").fill(email);
    await page.waitForTimeout(latency);
    await page.getByPlaceholder("비밀번호").fill(password);
    await page.screenshot({
      path: screenshot_path(
        "account",
        "account-log-in",
        "1-fill-the-login-form"
      ),
      fullPage: true,
    });
    await page
      .locator(
        'xpath=//*[@id="main"]/div/div[2]/div[1]/div[1]/div[2]/div/div[2]/div/div'
      )
      .click();
    await page.waitForNavigation();
    await page.screenshot({
      path: screenshot_path(
        "account",
        "account-log-in",
        "2-check-the-login-status"
      ),
      fullPage: true,
    });
    await expect(page).toHaveURL(
      "https://voice-korea.dev.biyard.co/ko/dashboard"
    );
    console.log("Test passed: User successfully log-in.");
  });
});
