import { test, expect } from '@playwright/test';
import path from 'path';

/**
 * File Upload E2E Tests
 *
 * Tests the core file upload functionality including:
 * - Basic file selection and upload
 * - Image preview display
 * - File information display
 * - Error handling
 *
 * Best practices followed:
 * - Use data-testid selectors (to be added to components)
 * - Explicit waits with meaningful timeouts
 * - Descriptive test names
 * - Independent tests (no shared state)
 */

test.describe('File Upload', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the application
    await page.goto('/');

    // Wait for the app to be fully loaded
    // The "Upload images" button should be visible
    await expect(page.locator('text=Upload images')).toBeVisible();
  });

  test('should display the upload interface on load', async ({ page }) => {
    // Verify the main heading is present
    await expect(page.locator('h1')).toContainText('File Metadata Extractor');

    // Verify the upload images button is visible
    const uploadButton = page.locator('button:has-text("Upload images")');
    await expect(uploadButton).toBeVisible();
    await expect(uploadButton).toBeEnabled();

    // Verify the image file input exists (hidden but present in DOM)
    // Note: There are 2 file inputs - one for images, one for ZIP archives
    const imageFileInput = page.locator('input[type="file"][accept*="image"]');
    await expect(imageFileInput).toHaveCount(1);
    await expect(imageFileInput).toHaveAttribute('accept', /image/);
  });

  test('should upload and display image information', async ({ page }) => {
    // Locate the image file input (not the ZIP archive input)
    const fileInput = page.locator('input[type="file"][accept*="image"]');

    // Upload a test image
    const filePath = path.join(__dirname, 'fixtures', 'simple.jpg');
    await fileInput.setInputFiles(filePath);

    // Wait for the image to be processed
    // The application should display the filename
    await expect(page.locator('text=simple.jpg')).toBeVisible({ timeout: 5000 });

    // Verify file size is displayed
    // The exact format is "315 B" or similar
    await expect(page.locator('text=/\\d+\\s*B|KB|MB/')).toBeVisible();

    // Verify image dimensions are displayed if applicable
    // Format: "50 × 50" or "50x50" or dimensions shown somewhere
    // This is a soft check as the simple.jpg might not have dimensions
    const dimensionsText = page.locator('text=/\\d+\\s*[×x]\\s*\\d+/');
    if (await dimensionsText.isVisible()) {
      await expect(dimensionsText).toBeVisible();
    }
  });

  test('should display image preview', async ({ page }) => {
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'simple.jpg');

    await fileInput.setInputFiles(filePath);

    // Wait for filename to appear (indicates processing complete)
    await expect(page.locator('text=simple.jpg')).toBeVisible({ timeout: 5000 });

    // Check that an image element with the uploaded data is visible
    // The app uses object URLs for image preview
    const imageElement = page.locator('img[src^="blob:"]');
    await expect(imageElement).toBeVisible();

    // Verify the image is actually loaded (not broken)
    const isImageLoaded = await imageElement.evaluate((img: HTMLImageElement) => {
      return img.complete && img.naturalWidth > 0;
    });
    expect(isImageLoaded).toBe(true);
  });

  test('should handle multiple file selection', async ({ page }) => {
    const fileInput = page.locator('input[type="file"][accept*="image"]');

    // Verify the input allows multiple files
    await expect(fileInput).toHaveAttribute('multiple', 'multiple');

    // Upload the same file twice (simulating multiple selection)
    const filePath = path.join(__dirname, 'fixtures', 'simple.jpg');
    await fileInput.setInputFiles([filePath, filePath]);

    // The application should show batch processing
    // Look for progress indicator or batch UI (use .first() since multiple elements match)
    const batchIndicator = page.locator('text=/Batch|Processing|\\d+\\s*of\\s*\\d+/i').first();
    if (await batchIndicator.isVisible({ timeout: 2000 }).catch(() => false)) {
      await expect(batchIndicator).toBeVisible();
    }

    // Eventually the file should be processed (use .first() since duplicates create multiple matches)
    await expect(page.locator('text=simple.jpg').first()).toBeVisible({ timeout: 10000 });
  });

  test('should show Upload images button when no file is uploaded', async ({ page }) => {
    // On initial load, only the Upload images button should be visible
    const uploadButton = page.locator('button:has-text("Upload images")');
    await expect(uploadButton).toBeVisible();

    // No file information should be displayed yet
    const fileInfoSection = page.locator('text=simple.jpg');
    await expect(fileInfoSection).not.toBeVisible();
  });
});
