import { test, expect, chromium } from "@playwright/test";
import fs from "fs";

const credentials = {
  email: process.env.GOOGLE_EMAIL || "testemail@gmail.com",
  pass: process.env.GOOGLE_PASS || "thepassword",
};

const timeouts = {
  wait: parseInt(process.env.WAIT_TIMEOUT || "5000", 10),
  visible: parseInt(process.env.VISIBLE_TIMEOUT || "5000", 10),
  url: parseInt(process.env.URL_TIMEOUT || "7000", 10),
};

let browserInstance: any = null;

async function getBrowserInstance() {
  if (!browserInstance) {
    browserInstance = await chromium.launch({
      headless: false,
      args: [
        "--disable-blink-features=AutomationControlled",
        "--no-sandbox",
        "--disable-web-security",
        "--disable-infobars",
        "--disable-extensions",
        "--start-maximized",
        "--window-size=1300, 1280",
      ],
    });
  }
  return browserInstance;
}

test.describe.configure({ mode: "serial" });

test("Google OAuth Login and Save Session", async ({ page, context }) => {
  // const browser = await getBrowserInstance();
  // const context = await browser.newContext();
  // const page = await context.newPage();

  await page.goto("https://dev.voice-korea.com/en/");
  await page.screenshot({
    path: "screenshots/users/google-001/01-load-page.png",
  });

  // const hamburger = page.getByRole('button').filter({ hasText: /^$/ })
  // await expect(hamburger).toBeVisible();
  // await hamburger.click();

  const googleSignInButton = page.getByText("Login");
  await expect(googleSignInButton).toBeVisible();
  await googleSignInButton.click();
  await page.screenshot({
    path: "screenshots/users/google-001/02-click-login.png",
  });

  const continueWithGoogle = page.getByText("Continue with Google");
  await expect(continueWithGoogle).toBeVisible();

  const [popup] = await Promise.all([
    page.waitForEvent("popup"),
    continueWithGoogle.click(),
  ]);
  await popup.screenshot({
    path: "screenshots/users/google-001/03-google-popup.png",
  });

  await popup.waitForLoadState("domcontentloaded");
  await popup.waitForURL(/accounts.google.com/);

  if (await popup.isVisible('input[type="email"]')) {
    await popup.fill('input[type="email"]', credentials.email);
    await popup.click('button:has-text("Next")');
    await popup.screenshot({
      path: "screenshots/users/google-001/04-enter-email.png",
    });

    if (await popup.isVisible("text=Couldn’t find your Google Account")) {
      console.error("Invalid email entered!");
      return;
    }

    await popup.waitForSelector('input[type="password"]', {
      timeout: timeouts.wait,
    });
    await popup.fill('input[type="password"]', credentials.pass);
    await popup.click('button:has-text("Next")');
    await popup.screenshot({
      path: "screenshots/users/google-001/05-enter-password.png",
    });

    if (await popup.isVisible("text=Wrong password. Try again")) {
      console.error("Incorrect password!");
      return;
    }

    await popup.waitForLoadState("networkidle");
  }

  if (await popup.isVisible("text=This app isn’t verified")) {
    await popup.click('button:has-text("Advanced")');
    await popup.click('a:has-text("Go to ratel.foundation (unsafe)")');
    await popup.screenshot({
      path: "screenshots/users/google-001/06-unverified-app.png",
    });
  }

  // await page.waitForLoadState('domcontentloaded');

  await page.screenshot({
    path: "screenshots/users/google-001/07-login-success.png",
  });

  await context.storageState({ path: "storage/auth.json" });

  console.log("Google OAuth login session saved!");

  await context.close();
  // await browser.close();
});

// test.afterAll(async () => {
//     if (browserInstance) {
//         await browserInstance.close();
//         browserInstance = null;
//     }
// });
