import { test, expect } from "@playwright/test";
import { wrap } from "../util";

const screenshotPages = [
  { name: "page-1", path: "Screenshot 2025-04-04 at 9.55.07 PM.png" },
  { name: "page-2", path: "Screenshot 2025-04-04 at 9.55.15 PM.png" },
  { name: "page-3", path: "Screenshot 2025-04-04 at 9.55.20 PM.png" },
];

test.describe("[UI-Tests] Checking pages using screenshots", () => {
  for (const pageInfo of screenshotPages) {
    test(`[${pageInfo.name}] Validate page structure`, async ({ page }, testInfo) => {
      const p = wrap(page, testInfo.project.name, `screenshots/${pageInfo.name}`);

      await p.goto(`file://${__dirname}/${pageInfo.path}`, { waitUntil: "load" });
      await p.fullCapture("full");
      await p.capture("top");

      const viewport = page.viewportSize();
      if (viewport && viewport.width < 768) {
        await p.clickXpathAndCapture("//button[contains(@class, 'menu')]", "Hamburger");
      }

      await p.clickXpathAndCapture("//nav/a[1]", "Navigation Link 1");
      await p.clickXpathAndCapture("//nav/a[2]", "Navigation Link 2");
      await p.clickXpathAndCapture("//nav/a[3]", "Navigation Link 3");

      const button = p.getByRole("button", { name: "Login" });
      await expect(button).toBeVisible();
    });
  }
});
