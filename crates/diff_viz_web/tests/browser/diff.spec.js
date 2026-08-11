import { expect, test } from "@playwright/test";

test("returns stable results across the real WASM boundary", async ({ page }) => {
  await page.goto("/");

  const result = await page.evaluate(async () => {
    const wasm = await import("/pkg/diff_viz_web.js");
    await wasm.default();
    const original = "alpha\nbeta\n";
    const modified = "alpha\ngamma\n";
    const lines = wasm.wasm_compute_diff(original, modified, "lines");
    const patch = wasm.wasm_generate_unified_diff(original, modified, "original", "modified", 3);
    const json = wasm.wasm_json_diff('{"enabled":false}', '{"enabled":true}');

    return {
      linesAreArray: Array.isArray(lines),
      kinds: lines.map((line) => line.kind),
      patchApplies: wasm.wasm_apply_patch(original, patch) === modified,
      jsonIsArray: Array.isArray(json),
      jsonKinds: json.map((line) => line.kind)
    };
  });

  expect(result).toEqual({
    linesAreArray: true,
    kinds: ["unchanged", "modified"],
    patchApplies: true,
    jsonIsArray: true,
    jsonKinds: ["modified"]
  });
});

test("compares text and switches between review views", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("#wasm-status")).toHaveClass(/ready/);

  await page.getByRole("textbox", { name: "Original text" }).fill("one\ntwo\nthree");
  await page.getByRole("textbox", { name: "Modified text" }).fill("one\nchanged\nthree");
  await page.getByRole("button", { name: "Compare" }).click();

  await expect(page.getByRole("table", { name: "Side-by-side diff" })).toBeVisible();
  await expect(page.getByText("1 changed", { exact: true })).toBeVisible();

  await page.getByRole("button", { name: "Unified" }).click();
  await expect(page.locator(".unified-list")).toContainText("changed");
  await expect(page.locator("#review-mode")).toHaveText("Lines / Unified");
});

test("supports JSON errors, keyboard comparison, and language switching", async ({ page }) => {
  await page.goto("/");
  const original = page.getByRole("textbox", { name: "Original text" });
  const modified = page.getByRole("textbox", { name: "Modified text" });

  await original.fill("left");
  await modified.fill("right");
  await original.press(process.platform === "darwin" ? "Meta+Enter" : "Control+Enter");
  await expect(page.getByRole("table", { name: "Side-by-side diff" })).toBeVisible();

  await page.getByRole("button", { name: "JSON" }).click();
  await expect(page.locator(".output-message.error")).toContainText("Operation failed");

  await page.getByRole("button", { name: "Switch language" }).click();
  await expect(page.locator("html")).toHaveAttribute("lang", "zh-CN");
  await expect(page.getByRole("button", { name: "对比" })).toBeVisible();
});

test("loads locally without console failures or page overflow", async ({ baseURL, page }) => {
  const problems = [];
  const externalRequests = [];
  const failedResponses = [];
  const expectedOrigin = new URL(baseURL).origin;

  page.on("console", (message) => {
    if (["error", "warning"].includes(message.type())) {
      problems.push(`${message.type()}: ${message.text()}`);
    }
  });
  page.on("pageerror", (error) => problems.push(`pageerror: ${error.message}`));
  page.on("request", (request) => {
    if (new URL(request.url()).origin !== expectedOrigin) {
      externalRequests.push(request.url());
    }
  });
  page.on("response", (response) => {
    if (response.status() >= 400) {
      failedResponses.push(`${response.status()} ${response.url()}`);
    }
  });

  await page.goto("/");
  await expect(page.locator("#wasm-status")).toHaveClass(/ready/);
  await page.waitForLoadState("networkidle");

  const layout = await page.evaluate(() => ({
    clientWidth: document.documentElement.clientWidth,
    scrollWidth: document.documentElement.scrollWidth
  }));
  expect(layout.scrollWidth).toBe(layout.clientWidth);
  expect(externalRequests).toEqual([]);
  expect(failedResponses).toEqual([]);
  expect(problems).toEqual([]);
});

test("exposes keyboard focus and respects reduced motion", async ({ page }) => {
  await page.emulateMedia({ reducedMotion: "reduce" });
  await page.goto("/");

  await page.keyboard.press("Tab");
  const skipLink = page.getByRole("link", { name: "Skip to workspace" });
  await expect(skipLink).toBeFocused();
  await expect(skipLink).toHaveCSS("outline-style", "auto");

  const transitionSeconds = await page.getByRole("button", { name: "Compare" }).evaluate(
    (button) => Number.parseFloat(getComputedStyle(button).transitionDuration)
  );
  expect(transitionSeconds).toBeLessThan(0.001);
});
