import { test, expect } from "@playwright/test";
import path from "path";

test.describe("UI Mobile Responsiveness & Project Page Tests", () => {
  test(`Check responsiveness`, async ({ page }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "users",
      projectName,
      "project-page",
    );

    await page.goto("/projects");

    const bodyOverflowX = await page.evaluate(
      () =>
        document.documentElement.scrollWidth >
        document.documentElement.clientWidth,
    );
    expect(bodyOverflowX).toBeFalsy();

    await page.screenshot({
      path: `${screenshotBase}/01-ui.png`,
      fullPage: true,
    });
  });
});
