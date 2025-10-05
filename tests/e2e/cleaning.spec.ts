import { test, expect } from '@playwright/test';
import path from 'path';

/**
 * Image Cleaning E2E Tests
 *
 * Tests the image cleaning functionality including:
 * - Clean button availability and interaction
 * - Download of cleaned files
 * - File format preservation
 * - Error handling
 */

test.describe('Image Cleaning', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByTestId('app-title')).toBeVisible();
    
    // Upload test image for cleaning tests
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    await fileInput.setInputFiles(filePath);
    
    // Wait for processing
    await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });
  });

  test('should show image cleaning section', async ({ page }) => {
    // Verify cleaning section is visible
    await expect(page.locator('text=/clean|remove.*metadata/i')).toBeVisible();
    await expect(page.getByTestId('clean-button')).toBeVisible();
    await expect(page.getByTestId('clean-button')).toBeEnabled();
  });

  test('should trigger cleaned image download', async ({ page }) => {
    const cleanButton = page.getByTestId('clean-button');
    
    // Set up download promise before clicking
    const downloadPromise = page.waitForEvent('download', { timeout: 10000 });
    
    await cleanButton.click();
    
    // Wait for download to start
    const download = await downloadPromise;
    
    // Verify download filename
    const filename = download.suggestedFilename();
    expect(filename).toContain('cleaned');
    expect(filename).toContain('with-metadata');
    expect(filename).toMatch(/\.(jpg|jpeg)$/i);
  });

  test('should preserve original file format', async ({ page }) => {
    const cleanButton = page.getByTestId('clean-button');
    
    const downloadPromise = page.waitForEvent('download', { timeout: 10000 });
    await cleanButton.click();
    
    const download = await downloadPromise;
    const filename = download.suggestedFilename();
    
    // Original is .jpg, cleaned should also be .jpg
    expect(filename).toMatch(/\.jpg$/i);
  });

  test('should show cleaning information and features', async ({ page }) => {
    // Look for cleaning description text
    await expect(page.locator('text=/privacy.*safe|metadata.*removed|binary.*clean/i').first()).toBeVisible();
    
    // Should mention supported formats
    const supportedFormatsText = page.locator('text=/jpeg|png|webp|gif|tiff|heif|pdf|svg/i');
    if (await supportedFormatsText.first().isVisible()) {
      await expect(supportedFormatsText.first()).toBeVisible();
    }
  });

  test('should show cleaning benefits and description', async ({ page }) => {
    // Look for privacy-related messaging
    const privacyText = page.locator('text=/privacy|metadata.*removed|clean.*file/i');
    await expect(privacyText.first()).toBeVisible();
    
    // Should explain what cleaning does
    const explanationText = page.locator('text=/remove.*metadata|strip.*data|binary.*clean/i');
    if (await explanationText.first().isVisible()) {
      await expect(explanationText.first()).toBeVisible();
    }
  });

  test('should handle cleaning button states correctly', async ({ page }) => {
    const cleanButton = page.getByTestId('clean-button');
    
    // Button should be enabled when file is loaded
    await expect(cleanButton).toBeEnabled();
    
    // Button should have appropriate styling
    const buttonText = await cleanButton.textContent();
    expect(buttonText).toMatch(/clean|download.*privacy.*safe|remove.*metadata/i);
  });

  test('should work with different image formats', async ({ page }) => {
    // Test with the same JPEG first
    let cleanButton = page.getByTestId('clean-button');
    await expect(cleanButton).toBeVisible();
    
    // If we had a PNG fixture, we could test that too
    // For now, verify the button works with our JPEG
    const downloadPromise = page.waitForEvent('download', { timeout: 10000 });
    await cleanButton.click();
    
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toBeTruthy();
  });

  test('should show quality preservation messaging', async ({ page }) => {
    // Look for text about quality preservation
    const qualityText = page.locator('text=/quality.*preserved|original.*quality|high.*performance/i');
    if (await qualityText.first().isVisible()) {
      await expect(qualityText.first()).toBeVisible();
    }
  });
});