import { Page, expect } from '@playwright/test';

/**
 * Helper functions for E2E tests
 * 
 * Common utilities to reduce code duplication and improve test readability.
 */

/**
 * Upload a file using the file input
 */
export async function uploadFile(page: Page, filePath: string): Promise<void> {
  const fileInput = page.locator('input[type="file"][accept*="image"]');
  await fileInput.setInputFiles(filePath);
}

/**
 * Wait for file to be processed and metadata to be available
 */
export async function waitForFileProcessing(page: Page, filename: string): Promise<void> {
  await expect(page.locator(`text=${filename}`)).toBeVisible({ timeout: 10000 });
}

/**
 * Wait for metadata display to be ready
 */
export async function waitForMetadata(page: Page): Promise<void> {
  // Wait for either file information section or metadata to appear
  await expect(page.locator('text=/File Information|metadata/i').first()).toBeVisible({ timeout: 5000 });
}

/**
 * Select export fields by checking checkboxes
 */
export async function selectExportFields(page: Page, fieldNames: string[]): Promise<void> {
  for (const field of fieldNames) {
    const checkbox = page.locator(`input[type="checkbox"][data-testid*="${field}"], input[type="checkbox"]:near(text="${field}")`);
    if (await checkbox.isVisible()) {
      await checkbox.check();
    }
  }
}

/**
 * Download a file and return the download object
 */
export async function downloadFile(page: Page, buttonSelector: string): Promise<any> {
  const downloadPromise = page.waitForEvent('download', { timeout: 10000 });
  await page.locator(buttonSelector).click();
  return await downloadPromise;
}

/**
 * Download a file by test ID and return the download object
 */
export async function downloadFileByTestId(page: Page, testId: string): Promise<any> {
  const downloadPromise = page.waitForEvent('download', { timeout: 10000 });
  await page.getByTestId(testId).click();
  return await downloadPromise;
}

/**
 * Check if an element is in the viewport
 */
export async function isInViewport(page: Page, selector: string): Promise<boolean> {
  return await page.locator(selector).evaluate((element) => {
    const rect = element.getBoundingClientRect();
    return (
      rect.top >= 0 &&
      rect.left >= 0 &&
      rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
      rect.right <= (window.innerWidth || document.documentElement.clientWidth)
    );
  });
}

/**
 * Get the size of an element (useful for touch target testing)
 */
export async function getElementSize(page: Page, selector: string): Promise<{ width: number; height: number } | null> {
  const element = page.locator(selector);
  if (await element.isVisible()) {
    const box = await element.boundingBox();
    return box ? { width: box.width, height: box.height } : null;
  }
  return null;
}

/**
 * Check if page has horizontal scrolling
 */
export async function hasHorizontalScroll(page: Page): Promise<boolean> {
  const scrollWidth = await page.evaluate(() => document.body.scrollWidth);
  const viewportWidth = page.viewportSize()?.width || 0;
  return scrollWidth > viewportWidth + 10; // Allow small tolerance
}

/**
 * Wait for batch processing to complete
 */
export async function waitForBatchProcessing(page: Page, expectedFileCount: number): Promise<void> {
  // Wait for batch progress to appear if multiple files
  if (expectedFileCount > 1) {
    const batchProgress = page.getByTestId('batch-progress');
    if (await batchProgress.isVisible({ timeout: 3000 })) {
      // Wait for batch to complete (progress disappears or shows 100%)
      await expect(batchProgress).not.toBeVisible({ timeout: 15000 }).or(
        expect(page.getByTestId('batch-status')).toContainText(/100%|completed/i)
      );
    }
  }
  
  // Wait for at least one file to be processed
  await expect(page.locator('text=/File Information|simple\.jpg/i').first()).toBeVisible({ timeout: 15000 });
}

/**
 * Upload multiple files for batch testing
 */
export async function uploadMultipleFiles(page: Page, filePaths: string[]): Promise<void> {
  const fileInput = page.locator('input[type="file"][accept*="image"]');
  await fileInput.setInputFiles(filePaths);
}

/**
 * Check if metadata export functionality is available
 */
export async function hasExportFunctionality(page: Page): Promise<boolean> {
  const exportButtons = page.locator('button:has-text("Export"), button:has-text("JSON"), button:has-text("CSV"), button:has-text("Download")');
  return await exportButtons.first().isVisible();
}

/**
 * Navigate to next item in batch (if available)
 */
export async function navigateBatchNext(page: Page): Promise<boolean> {
  const nextButton = page.locator('button:has-text("Next"), button:has-text("➡")').first();
  if (await nextButton.isVisible() && await nextButton.isEnabled()) {
    await nextButton.click();
    return true;
  }
  return false;
}

/**
 * Navigate to previous item in batch (if available)
 */
export async function navigateBatchPrevious(page: Page): Promise<boolean> {
  const prevButton = page.locator('button:has-text("Previous"), button:has-text("⬅")').first();
  if (await prevButton.isVisible() && await prevButton.isEnabled()) {
    await prevButton.click();
    return true;
  }
  return false;
}

/**
 * Get current batch position info
 */
export async function getBatchPosition(page: Page): Promise<{ current: number; total: number } | null> {
  const positionText = page.locator('text=/image.*of.*|file.*of.*/i');
  if (await positionText.isVisible()) {
    const text = await positionText.textContent();
    const match = text?.match(/(\d+).*of.*(\d+)/i);
    if (match) {
      return {
        current: parseInt(match[1]),
        total: parseInt(match[2])
      };
    }
  }
  return null;
}

/**
 * Check if element is touch-friendly (meets minimum size requirements)
 */
export async function isTouchFriendly(page: Page, selector: string, minSize: number = 44): Promise<boolean> {
  const size = await getElementSize(page, selector);
  if (!size) return false;
  return size.width >= minSize && size.height >= minSize;
}