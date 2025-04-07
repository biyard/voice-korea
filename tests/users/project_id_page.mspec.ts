import { test, expect } from "@playwright/test";
import path from "path";

test.describe("UI Mobile Responsiveness & Individual Project Page Tests", () => {
  test(`[Project-ID-001] Check responsiveness on `, async ({
    page,
  }, testInfo) => {
    const projectName = testInfo.project.name;
    const screenshotBase = path.join(
      "screenshots",
      "users",
      projectName,
      "project-id-page",
    );

    const randomProjectNumber = Math.floor(Math.random() * 100) + 1;
    await page.goto(`/projects/${randomProjectNumber}`);
    await page.screenshot({
      path: `${screenshotBase}/01-page-entered.png`,
      fullPage: true,
    });

    const bodyOverflowX = await page.evaluate(
      () =>
        document.documentElement.scrollWidth >
        document.documentElement.clientWidth,
    );
    expect(bodyOverflowX).toBeFalsy();

    await page.screenshot({
      path: `${screenshotBase}/02-ui.png`,
      fullPage: true,
    });
  });
});
