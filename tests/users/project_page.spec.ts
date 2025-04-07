import { test, expect } from "@playwright/test";

test.describe("UI Mobile Responsiveness & Project Page Tests", () => {
  test("Clicking a project navigates to the correct page", async ({ page }) => {
    await page.goto("https://dev.voice-korea.com/en/projects");
    const randomProject = page.locator(
      ".max-w-\\[1300px\\] > div:nth-child(2) > div:nth-child(2) > div:nth-child(2) > div",
    );
    await randomProject.click();

    const url = page.url();
    const match = url.match(/\/projects\/(\d+)$/);

    expect(match).not.toBeNull();
    expect(Number(match?.[1])).not.toBeNaN();
  });

  test("Searching filters projects", async ({ page }) => {
    await page.goto("https://dev.voice-korea.com/en/projects");
    const searchBox = page.getByRole("textbox", { name: "Search" });
    await expect(searchBox).toBeVisible();
    await searchBox.fill("Economy");
    await page.keyboard.press("Enter");
  });

  test("Set Oldest-Newest Filters", async ({ page }) => {
    await page.goto("https://dev.voice-korea.com/en/projects");
    const searchBox = page.locator("summary");
    await searchBox.click();
    const oldestButton = await page.getByRole("button", { name: "Oldest" });
    await expect(oldestButton).toBeVisible();
    await oldestButton.click();
    const newestButton = await page.getByRole("button", { name: "Newest" });
    await expect(newestButton).toBeVisible();
    await newestButton.click();
  });
});
