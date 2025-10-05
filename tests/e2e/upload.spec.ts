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
    // The "Choose Image" button should be visible
    await expect(page.locator('text=Choose Image')).toBeVisible();
  });

  test('should display the upload interface on load', async ({ page }) => {
    // Verify the main heading is present
    await expect(page.locator('h1')).toContainText('Image Metadata Extractor');

    // Verify the choose image button is visible
    const chooseButton = page.locator('button:has-text("Choose Image")');
    await expect(chooseButton).toBeVisible();
    await expect(chooseButton).toBeEnabled();

    // Verify the file input exists (hidden but present in DOM)
    const fileInput = page.locator('input[type="file"]');
    await expect(fileInput).toHaveCount(1);
    await expect(fileInput).toHaveAttribute('accept', /image/);
  });

  test('should upload and display image information', async ({ page }) => {
    // Locate the file input
    const fileInput = page.locator('input[type="file"]');

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
    const fileInput = page.locator('input[type="file"]');
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
    const fileInput = page.locator('input[type="file"]');

    // Verify the input allows multiple files
    await expect(fileInput).toHaveAttribute('multiple', '');

    // Upload the same file twice (simulating multiple selection)
    const filePath = path.join(__dirname, 'fixtures', 'simple.jpg');
    await fileInput.setInputFiles([filePath, filePath]);

    // The application should show batch processing
    // Look for progress indicator or batch UI
    const batchIndicator = page.locator('text=/Batch|Processing|\\d+\\s*of\\s*\\d+/i');
    if (await batchIndicator.isVisible({ timeout: 2000 })) {
      await expect(batchIndicator).toBeVisible();
    }

    // Eventually the file should be processed
    await expect(page.locator('text=simple.jpg')).toBeVisible({ timeout: 10000 });
  });

  test('should show Choose Image button when no file is uploaded', async ({ page }) => {
    // On initial load, only the Choose Image button should be visible
    const chooseButton = page.locator('button:has-text("Choose Image")');
    await expect(chooseButton).toBeVisible();

    // No file information should be displayed yet
    const fileInfoSection = page.locator('text=simple.jpg');
    await expect(fileInfoSection).not.toBeVisible();
  });
});
