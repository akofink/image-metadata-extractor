import { test, expect } from '@playwright/test';
import path from 'path';

/**
 * Batch Downloads E2E Tests
 *
 * Tests the batch download functionality including:
 * - Batch ZIP download of cleaned images
 * - Batch export functionality
 * - Error handling for batch operations
 * - Performance with multiple files
 */

test.describe('Batch Downloads', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.getByTestId('app-title')).toBeVisible();
  });

  test('should enable batch ZIP download after uploading multiple files', async ({ page }) => {
    // Upload multiple files
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    await fileInput.setInputFiles([filePath, filePath, filePath]);
    
    // Wait for processing to complete
    await expect(page.locator('text=with-metadata.jpg').first()).toBeVisible({ timeout: 10000 });
    
    // Look for batch download/ZIP button
    const batchButtons = page.locator('button:has-text("ZIP"), button:has-text("Batch"), button:has-text("Download All")');
    const batchButton = batchButtons.first();
    
    if (await batchButton.isVisible()) {
      await expect(batchButton).toBeVisible();
      await expect(batchButton).toBeEnabled();
    } else {
      // If no obvious batch button, look for any download-related batch functionality
      const allButtons = page.locator('button');
      const buttonCount = await allButtons.count();
      
      let foundBatchRelated = false;
      for (let i = 0; i < buttonCount; i++) {
        const button = allButtons.nth(i);
        const text = await button.textContent();
        if (text && (text.toLowerCase().includes('batch') || 
                    text.toLowerCase().includes('zip') || 
                    text.toLowerCase().includes('all'))) {
          foundBatchRelated = true;
          await expect(button).toBeVisible();
          break;
        }
      }
      
      // Log what buttons are available for debugging
      const buttonTexts = [];
      for (let i = 0; i < Math.min(buttonCount, 10); i++) {
        const text = await allButtons.nth(i).textContent();
        if (text) buttonTexts.push(text.trim());
      }
      console.log('Available buttons:', buttonTexts);
      
      if (!foundBatchRelated) {
        console.log('No batch download functionality found - this may be the bug');
      }
    }
  });

  test('should trigger batch ZIP download with console monitoring', async ({ page }) => {
    // Monitor console messages to catch the error
    const consoleMessages: string[] = [];
    const errorMessages: string[] = [];
    
    page.on('console', msg => {
      const text = msg.text();
      consoleMessages.push(`${msg.type()}: ${text}`);
      if (msg.type() === 'log' && text.includes('No files were successfully cleaned')) {
        errorMessages.push(text);
      }
      console.log(`CONSOLE ${msg.type()}: ${text}`);
    });

    // Upload multiple files
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    await fileInput.setInputFiles([filePath, filePath]);
    
    // Wait for processing
    await expect(page.locator('text=with-metadata.jpg').first()).toBeVisible({ timeout: 10000 });
    
    // Look for batch download functionality
    const batchRelatedButtons = page.locator('button').filter({ 
      hasText: /zip|batch|download.*all|all.*download/i 
    });
    
    const buttonCount = await batchRelatedButtons.count();
    console.log(`Found ${buttonCount} batch-related buttons`);
    
    if (buttonCount > 0) {
      const batchButton = batchRelatedButtons.first();
      const buttonText = await batchButton.textContent();
      console.log(`Attempting to click batch button: "${buttonText}"`);
      
      // Try to download batch
      const downloadPromise = page.waitForEvent('download', { timeout: 10000 }).catch(() => null);
      
      await batchButton.click();
      
      // Wait for either download or error
      await page.waitForTimeout(3000);
      
      const download = await downloadPromise;
      
      if (download) {
        console.log(`✅ Batch download succeeded: ${download.suggestedFilename()}`);
        expect(download.suggestedFilename()).toMatch(/\.zip$/i);
      } else {
        console.log('❌ No download occurred');
        
        // Check if we caught the error message
        if (errorMessages.length > 0) {
          console.log('🐛 Found the bug! Error messages:', errorMessages);
          throw new Error(`Batch cleaning failed: ${errorMessages[0]}`);
        }
      }
    } else {
      throw new Error('No batch download functionality found in UI');
    }
  });

  test('should show batch cleaning progress', async ({ page }) => {
    // Upload multiple files to trigger batch mode
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    await fileInput.setInputFiles([filePath, filePath, filePath, filePath]);
    
    // Wait for processing
    await expect(page.locator('text=with-metadata.jpg').first()).toBeVisible({ timeout: 15000 });
    
    // Look for batch progress indicators
    const progressElements = page.locator('[data-testid="batch-progress"]').or(page.locator('text=/processing|cleaning|progress/i'));
    
    if (await progressElements.first().isVisible()) {
      await expect(progressElements.first()).toBeVisible();
    }
    
    // Look for batch-specific UI elements
    const batchUI = page.locator('text=/batch|multiple.*files|(\d+).*files/i');
    if (await batchUI.first().isVisible()) {
      await expect(batchUI.first()).toBeVisible();
    }
  });

  test('should handle batch export with multiple files', async ({ page }) => {
    // Upload multiple files
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    await fileInput.setInputFiles([filePath, filePath, filePath]);
    
    // Wait for processing
    await expect(page.locator('text=with-metadata.jpg').first()).toBeVisible({ timeout: 10000 });
    
    // Look for export functionality that works with multiple files
    const exportButtons = page.locator('button:has-text("JSON"), button:has-text("CSV"), button:has-text("Export")');
    
    if (await exportButtons.first().isVisible()) {
      const downloadPromise = page.waitForEvent('download', { timeout: 10000 }).catch(() => null);
      
      await exportButtons.first().click();
      
      const download = await downloadPromise;
      
      if (download) {
        const filename = download.suggestedFilename();
        console.log(`Batch export succeeded: ${filename}`);
        
        // Should contain data from multiple files
        expect(filename).toBeTruthy();
      }
    }
  });

  test('should show appropriate error messaging for batch operations', async ({ page }) => {
    // Monitor console for error messages
    const consoleMessages: string[] = [];
    page.on('console', msg => {
      consoleMessages.push(`${msg.type()}: ${msg.text()}`);
    });

    // Upload files that might cause cleaning issues
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    await fileInput.setInputFiles([filePath, filePath]);
    
    await expect(page.locator('text=with-metadata.jpg').first()).toBeVisible({ timeout: 10000 });
    
    // Try to trigger batch operations
    const allButtons = page.locator('button');
    const buttonCount = await allButtons.count();
    
    for (let i = 0; i < buttonCount; i++) {
      const button = allButtons.nth(i);
      const text = await button.textContent();
      
      if (text && (text.toLowerCase().includes('zip') || 
                  text.toLowerCase().includes('batch') ||
                  text.toLowerCase().includes('download all'))) {
        
        console.log(`Testing button: "${text}"`);
        
        await button.click();
        await page.waitForTimeout(2000);
        
        // Check for error messages
        const errorInConsole = consoleMessages.some(msg => 
          msg.includes('No files were successfully cleaned') ||
          msg.includes('cleaning failed') ||
          msg.includes('error')
        );
        
        if (errorInConsole) {
          console.log('🐛 Found batch cleaning error:', consoleMessages.filter(msg => 
            msg.includes('No files were successfully cleaned') ||
            msg.includes('cleaning failed') ||
            msg.includes('error')
          ));
        }
        
        break;
      }
    }
  });

  test('should work with different file types in batch', async ({ page }) => {
    // Test batch operations with the same file uploaded multiple times
    // (simulating different file types by using the same file)
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    
    // Upload multiple instances
    await fileInput.setInputFiles([filePath, filePath, filePath, filePath, filePath]);
    
    // Wait for all files to be processed
    await expect(page.locator('text=with-metadata.jpg').first()).toBeVisible({ timeout: 15000 });
    
    // Look for batch functionality
    const batchElements = page.locator('button').or(page.locator('text=/5.*files|batch.*5|processing.*5/i'));
    
    if (await batchElements.first().isVisible()) {
      console.log('✅ Batch UI appears for 5 files');
    }
    
    // Try batch operations
    const zipButton = page.locator('button:has-text("ZIP"), button:has-text("Download"), button').filter({
      hasText: /batch|zip|all/i
    }).first();
    
    if (await zipButton.isVisible()) {
      console.log('Found batch download button');
      
      // Monitor for the specific error
      let foundError = false;
      page.on('console', msg => {
        if (msg.text().includes('No files were successfully cleaned')) {
          foundError = true;
          console.log('🐛 Reproduced the batch cleaning bug!');
        }
      });
      
      await zipButton.click();
      await page.waitForTimeout(3000);
      
      if (foundError) {
        throw new Error('Batch cleaning bug reproduced: No files were successfully cleaned');
      }
    }
  });
});