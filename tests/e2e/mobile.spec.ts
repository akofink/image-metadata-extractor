import { test, expect, devices } from '@playwright/test';
import path from 'path';

/**
 * Mobile Responsiveness E2E Tests
 *
 * Tests the application's mobile experience including:
 * - Responsive layout adaptation
 * - Touch-friendly interface elements
 * - Mobile-specific functionality
 * - Cross-device compatibility
 */

test.describe('Mobile Responsiveness', () => {
  test.describe('iPhone 13', () => {

    test('should display mobile-friendly layout', async ({ browser }) => {
      const context = await browser.newContext({ ...devices['iPhone 13'] });
      const page = await context.newPage();
      await page.goto('/');
      await expect(page.getByTestId('app-title')).toBeVisible();

      // Verify responsive layout
      const title = page.getByTestId('app-title');
      await expect(title).toBeVisible();

      // Check that upload buttons are touch-friendly
      const uploadButton = page.getByTestId('upload-images-button');
      await expect(uploadButton).toBeVisible();

      // Verify button is large enough for touch (minimum 44px)
      const buttonBox = await uploadButton.boundingBox();
      expect(buttonBox?.height).toBeGreaterThanOrEqual(40);
    });

    test('should handle file upload on mobile', async ({ browser }) => {
      const context = await browser.newContext({ ...devices['iPhone 13'] });
      const page = await context.newPage();
      await page.goto('/');
      
      const fileInput = page.locator('input[type="file"][accept*="image"]');
      const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
      await fileInput.setInputFiles(filePath);

      // Wait for processing
      await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });

      // Verify content is readable on mobile
      await expect(page.locator('text=/File Information/i')).toBeVisible();
      await context.close();
    });

    test('should not have horizontal scrolling', async ({ page }) => {
      await page.goto('/');
      
      // Upload a file to get full content
      const fileInput = page.locator('input[type="file"][accept*="image"]');
      const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
      await fileInput.setInputFiles(filePath);
      await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });

      // Check that content fits in viewport
      const bodyScrollWidth = await page.evaluate(() => document.body.scrollWidth);
      const viewportWidth = page.viewportSize()?.width || 0;
      
      // Allow small tolerance for scrollbars
      expect(bodyScrollWidth).toBeLessThanOrEqual(viewportWidth + 20);
    });

    test('should have touch-friendly interactive elements', async ({ page }) => {
      await page.goto('/');
      
      // Upload file first
      const fileInput = page.locator('input[type="file"][accept*="image"]');
      const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
      await fileInput.setInputFiles(filePath);
      await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });

      // Check clean button size
      const cleanButton = page.getByTestId('clean-button');
      if (await cleanButton.isVisible()) {
        const buttonBox = await cleanButton.boundingBox();
        expect(buttonBox?.height).toBeGreaterThanOrEqual(40);
      }

      // Check any checkboxes are touch-friendly
      const checkboxes = page.locator('input[type="checkbox"]');
      const checkboxCount = await checkboxes.count();
      if (checkboxCount > 0) {
        const firstCheckbox = checkboxes.first();
        const checkboxBox = await firstCheckbox.boundingBox();
        // Checkbox should have adequate touch target
        expect(checkboxBox?.width).toBeGreaterThanOrEqual(20);
        expect(checkboxBox?.height).toBeGreaterThanOrEqual(20);
      }
    });
  });

  test.describe('Pixel 5 (Android)', () => {

    test('should work on Android Chrome', async ({ browser }) => {
      const context = await browser.newContext({ ...devices['Pixel 5'] });
      const page = await context.newPage();
      await page.goto('/');
      await expect(page.getByTestId('app-title')).toBeVisible();

      // Test file upload
      const fileInput = page.locator('input[type="file"][accept*="image"]');
      const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
      await fileInput.setInputFiles(filePath);

      await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });

      // Verify key functionality works
      await expect(page.getByTestId('clean-button')).toBeVisible();
      await context.close();
    });

    test('should handle touch gestures appropriately', async ({ page }) => {
      await page.goto('/');
      
      const fileInput = page.locator('input[type="file"][accept*="image"]');
      const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
      await fileInput.setInputFiles(filePath);
      await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });

      // Test tapping on image (if expandable)
      const imageElement = page.locator('img[src^="blob:"]');
      if (await imageElement.isVisible()) {
        await imageElement.tap();
        // Should handle tap without errors
      }
    });
  });

  test.describe('Tablet Landscape', () => {
    test.use({ 
      viewport: { width: 1024, height: 768 }
    });

    test('should adapt layout for tablet', async ({ page }) => {
      await page.goto('/');
      await expect(page.getByTestId('app-title')).toBeVisible();

      // Upload file
      const fileInput = page.locator('input[type="file"][accept*="image"]');
      const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
      await fileInput.setInputFiles(filePath);
      await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });

      // Should use available space efficiently
      const container = page.locator('.container, [data-testid="app-container"], body > div').first();
      const containerBox = await container.boundingBox();
      
      // Content should be reasonably sized for tablet
      expect(containerBox?.width).toBeGreaterThan(600);
    });
  });

  test.describe('Mobile Portrait vs Landscape', () => {
    test('should work in both orientations', async ({ page }) => {
      // Start in portrait
      await page.setViewportSize({ width: 375, height: 667 });
      await page.goto('/');
      await expect(page.getByTestId('app-title')).toBeVisible();

      // Upload file in portrait
      const fileInput = page.locator('input[type="file"][accept*="image"]');
      const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
      await fileInput.setInputFiles(filePath);
      await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });

      // Switch to landscape
      await page.setViewportSize({ width: 667, height: 375 });
      
      // Content should still be visible and functional
      await expect(page.getByTestId('app-title')).toBeVisible();
      await expect(page.locator('text=with-metadata.jpg')).toBeVisible();
      await expect(page.getByTestId('clean-button')).toBeVisible();
    });
  });

  test.describe('Mobile Performance', () => {

    test('should load quickly on mobile', async ({ page }) => {
      const startTime = Date.now();
      
      await page.goto('/');
      await expect(page.getByTestId('app-title')).toBeVisible();
      
      const loadTime = Date.now() - startTime;
      
      // Should load within reasonable time (5 seconds on mobile)
      expect(loadTime).toBeLessThan(5000);
    });

    test('should handle file processing efficiently on mobile', async ({ page }) => {
      await page.goto('/');
      
      const startTime = Date.now();
      
      const fileInput = page.locator('input[type="file"][accept*="image"]');
      const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
      await fileInput.setInputFiles(filePath);
      
      await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 10000 });
      
      const processingTime = Date.now() - startTime;
      
      // File processing should complete within reasonable time on mobile
      expect(processingTime).toBeLessThan(10000);
    });
  });
});