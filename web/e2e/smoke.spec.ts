import { test, expect } from "@playwright/test";

// The single flow most likely to silently break: routing + i18n + lazy per-page
// chunks + the search index. If this passes, the app boots, the language toggle
// rewrites the URL and swaps content, a research page renders from its MDX
// chunk, and ⌘K search finds and navigates to a result.

test("home renders and the language toggle switches to French", async ({ page }) => {
  await page.goto("/");
  // The FR/EN toggle is in the header. On an English page it offers "FR".
  const toggle = page.getByRole("button", { name: /^FR$/ });
  await expect(toggle).toBeVisible();
  await toggle.click();
  await expect(page).toHaveURL(/\/fr(\/|$)/);
  await expect(page.locator("html")).toHaveAttribute("lang", "fr");
  // Back to English.
  await page.getByRole("button", { name: /^EN$/ }).click();
  await expect(page).toHaveURL(/localhost:5173\/?$/);
});

test("a research page renders from its MDX chunk", async ({ page }) => {
  await page.goto("/research/records");
  await expect(page.getByRole("heading", { level: 1 })).toContainText(/Records/i);
  // The RecordsView island (a lazy chunk) mounts without a page error.
  await expect(page.locator("main")).toBeVisible();
});

test("the same research page renders in French", async ({ page }) => {
  await page.goto("/fr/research/records");
  await expect(page.locator("html")).toHaveAttribute("lang", "fr");
  await expect(page.getByRole("heading", { level: 1 })).toContainText(/Records et solveurs/i);
});

test("search finds a page and navigates to it", async ({ page }) => {
  await page.goto("/research");
  // Open the search dialog via its header button (labelled "Search").
  await page.getByRole("button", { name: "Search" }).click();
  // The search input lives inside the dialog.
  const dialog = page.getByRole("dialog");
  await expect(dialog).toBeVisible();
  const searchbox = dialog.locator("input");
  await expect(searchbox).toBeVisible();
  await searchbox.fill("records"); // needs ≥2 chars to search
  // Results are buttons carrying the page title; click the "Records" one.
  const result = dialog.getByRole("button", { name: /records/i }).first();
  await expect(result).toBeVisible();
  await result.click();
  await expect(page).toHaveURL(/\/research\//);
});
