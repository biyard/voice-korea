import { test, expect } from "@playwright/test";
import path from "path";

test.describe("UI Mobile Responsiveness", () => {
  test(`Check responsiveness`, async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "users",
      projectName,
      "home-overflow",
    );

    await page.goto("/");

    const bodyOverflowX = await page.evaluate(
      () =>
        document.documentElement.scrollWidth >
        document.documentElement.clientWidth,
    );
    expect(bodyOverflowX).toBeFalsy();

    await page.screenshot({
      path: `${screenshotBase}/01-home.png`,
      fullPage: true,
    });
  });
});
