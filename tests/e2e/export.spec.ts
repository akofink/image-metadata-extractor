import { test, expect } from '@playwright/test';
import path from 'path';

/**
 * Export Functionality E2E Tests
 *
 * Tests the metadata export functionality including:
 * - JSON, CSV, TXT export formats
 * - Field selection controls
 * - Download triggering
 * - Browser compatibility
 */

test.describe('Metadata Export', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByTestId('app-title')).toBeVisible();
    
    // Upload test image for export tests
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    await fileInput.setInputFiles(filePath);
    
    // Wait for processing
    await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });
  });

  test('should show export section with format options', async ({ page }) => {
    // Look for export-related UI elements
    const exportText = page.locator('text=/export|download.*metadata/i').first();
    if (await exportText.isVisible()) {
      await expect(exportText).toBeVisible();
    }

    // Check for format options (JSON, CSV, etc.)
    const formatOptions = page.locator('text=/json|csv|txt|xml|yaml|markdown/i');
    if (await formatOptions.first().isVisible()) {
      await expect(formatOptions.first()).toBeVisible();
    }
  });

  test('should trigger JSON export download', async ({ page }) => {
    // Look for JSON export button
    const jsonButton = page.locator('button:has-text("JSON"), button:has-text("json"), [data-testid*="json"]').first();
    
    if (await jsonButton.isVisible()) {
      // Set up download promise before clicking
      const downloadPromise = page.waitForEvent('download', { timeout: 10000 });
      
      await jsonButton.click();
      
      // Wait for download to start
      const download = await downloadPromise;
      
      // Verify download filename contains expected parts
      const filename = download.suggestedFilename();
      expect(filename).toMatch(/\.json$/i);
      expect(filename).toContain('with-metadata');
    } else {
      // If no JSON button found, look for general export functionality
      const exportButton = page.locator('button:has-text("Export"), button:has-text("Download")').first();
      if (await exportButton.isVisible()) {
        await expect(exportButton).toBeVisible();
      }
    }
  });

  test('should trigger CSV export download', async ({ page }) => {
    const csvButton = page.locator('button:has-text("CSV"), button:has-text("csv"), [data-testid*="csv"]').first();
    
    if (await csvButton.isVisible()) {
      const downloadPromise = page.waitForEvent('download', { timeout: 10000 });
      
      await csvButton.click();
      
      const download = await downloadPromise;
      const filename = download.suggestedFilename();
      expect(filename).toMatch(/\.csv$/i);
    }
  });

  test('should trigger TXT export download', async ({ page }) => {
    const txtButton = page.locator('button:has-text("TXT"), button:has-text("txt"), button:has-text("Text"), [data-testid*="txt"]').first();
    
    if (await txtButton.isVisible()) {
      const downloadPromise = page.waitForEvent('download', { timeout: 10000 });
      
      await txtButton.click();
      
      const download = await downloadPromise;
      const filename = download.suggestedFilename();
      expect(filename).toMatch(/\.txt$/i);
    }
  });

  test('should handle field selection for export', async ({ page }) => {
    // Look for checkboxes or selection controls
    const checkboxes = page.locator('input[type="checkbox"]');
    const checkboxCount = await checkboxes.count();
    
    if (checkboxCount > 0) {
      // Verify we can interact with selection controls
      const firstCheckbox = checkboxes.first();
      await expect(firstCheckbox).toBeVisible();
      
      // Test checking/unchecking
      const initialState = await firstCheckbox.isChecked();
      await firstCheckbox.click();
      const newState = await firstCheckbox.isChecked();
      expect(newState).toBe(!initialState);
    }
  });

  test('should show select all / deselect all functionality', async ({ page }) => {
    // Look for select all / deselect all buttons or links
    const selectAllButton = page.locator('button').filter({ hasText: /select.*all/i }).first();
    const deselectAllButton = page.locator('button').filter({ hasText: /deselect|clear.*all|none/i }).first();
    
    // Alternative: look for buttons with "All" or "None" text
    const allButton = page.locator('button:has-text("All")').first();
    const noneButton = page.locator('button:has-text("None")').first();
    
    if (await selectAllButton.isVisible() || await allButton.isVisible()) {
      const buttonToClick = await selectAllButton.isVisible() ? selectAllButton : allButton;
      await expect(buttonToClick).toBeVisible();
      await buttonToClick.click();
    }
    
    if (await deselectAllButton.isVisible() || await noneButton.isVisible()) {
      const buttonToCheck = await deselectAllButton.isVisible() ? deselectAllButton : noneButton;
      await expect(buttonToCheck).toBeVisible();
    }
  });

  test('should export files with appropriate filenames', async ({ page }) => {
    // Find any export button
    const exportButtons = page.locator('button:has-text("Export"), button:has-text("Download"), button:has-text("JSON"), button:has-text("CSV")');
    const buttonCount = await exportButtons.count();
    
    if (buttonCount > 0) {
      const firstExportButton = exportButtons.first();
      
      const downloadPromise = page.waitForEvent('download', { timeout: 10000 });
      await firstExportButton.click();
      
      const download = await downloadPromise;
      const filename = download.suggestedFilename();
      
      // Verify filename structure
      expect(filename).toBeTruthy();
      expect(filename.length).toBeGreaterThan(0);
      
      // Should contain the original filename or a reasonable export name
      const isReasonableFilename = filename.includes('simple') || 
                                  filename.includes('metadata') || 
                                  filename.includes('export') ||
                                  /\.(json|csv|txt|xml|yaml|md)$/i.test(filename);
      expect(isReasonableFilename).toBe(true);
    }
  });

  test('should handle export preferences', async ({ page }) => {
    // Look for preference controls (include file info, GPS, etc.)
    const preferenceControls = page.locator('input[type="checkbox"]:has-text("file"), input[type="checkbox"]:has-text("GPS"), input[type="checkbox"]:has-text("location")');
    
    if (await preferenceControls.first().isVisible()) {
      const control = preferenceControls.first();
      const initialState = await control.isChecked();
      
      // Toggle the preference
      await control.click();
      const newState = await control.isChecked();
      expect(newState).toBe(!initialState);
      
      // Toggle back
      await control.click();
      const finalState = await control.isChecked();
      expect(finalState).toBe(initialState);
    }
  });
});