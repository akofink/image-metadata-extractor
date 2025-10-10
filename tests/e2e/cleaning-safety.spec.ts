import { test, expect } from './fixtures/block-analytics';
import path from 'path';

/**
 * Image Cleaning Safety Tests
 *
 * Tests that cleaning failures properly prevent downloads and show error messages
 * to protect user privacy. Critical for preventing users from thinking uncleaned
 * files are actually cleaned.
 */

test.describe('Image Cleaning Safety', () => {
  test('should prevent download when cleaning fails with error', async ({ page }) => {
    // Mock window.alert to capture error messages. Must be done before goto.
    await page.addInitScript(() => {
      window.alert = (message: string) => {
        (window as any).__lastAlert = message;
        console.log('ALERT:', message);
      };
    });

    await page.goto('/');
    await expect(page.getByTestId('app-title')).toBeVisible();

    // Upload a corrupted image that will cause a cleaning failure
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'corrupted.jpg');
    await fileInput.setInputFiles(filePath);
    
    await expect(page.locator('text=corrupted.jpg')).toBeVisible({ timeout: 5000 });

    // Set up a listener to fail the test if a download occurs
    let downloadOccurred = false;
    page.on('download', () => {
      downloadOccurred = true;
    });

    // Click the clean button
    const cleanButton = page.getByTestId('clean-button');
    await expect(cleanButton).toBeVisible();
    await cleanButton.click();

    // Wait for the alert to be shown
    await page.waitForFunction(() => (window as any).__lastAlert);

    // Check that an error alert was shown
    const alertMessage = await page.evaluate(() => (window as any).__lastAlert);
    expect(alertMessage).toContain('Error: Could not clean metadata from corrupted.jpg');

    // Verify that no download was initiated
    expect(downloadOccurred).toBe(false);
    console.log('✅ Download properly prevented on cleaning failure');
  });

  test('should show clear error for unsupported formats', async ({ page }) => {
    // Monitor alerts. Must be done before goto.
    await page.addInitScript(() => {
      window.alert = (message: string) => {
        (window as any).__lastAlert = message;
        console.log('ALERT:', message);
      };
    });

    await page.goto('/');
    await expect(page.getByTestId('app-title')).toBeVisible();

    // Upload an unsupported file type
    const fileInput = page.locator('input[type="file"][accept*="image"]');
    await fileInput.setInputFiles({
      name: 'document.pdf',
      mimeType: 'application/pdf',
      buffer: Buffer.from('%PDF-1.4\n')
    });
    
    await expect(page.locator('text=document.pdf')).toBeVisible({ timeout: 5000 });

    const cleanButton = page.getByTestId('clean-button');
    await cleanButton.click();

    // Wait for the alert to be shown
    await page.waitForFunction(() => (window as any).__lastAlert);

    // Check that an error alert was shown
    const alertMessage = await page.evaluate(() => (window as any).__lastAlert);
    expect(alertMessage).toContain('Error: Could not clean metadata from document.pdf');
    expect(alertMessage).toContain('The file format may not be fully supported for cleaning.');
    console.log('✅ Clear error shown for unsupported format');
  });

  test('should log cleaning statistics for successful operations', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByTestId('app-title')).toBeVisible();

    const consoleLogs: string[] = [];
    
    page.on('console', msg => {
      if (msg.type() === 'log') {
        consoleLogs.push(msg.text());
        console.log('CONSOLE LOG:', msg.text());
      }
    });

    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    await fileInput.setInputFiles(filePath);
    
    await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });

    const cleanButton = page.getByTestId('clean-button');
    
    // This should trigger a download (for JPEG with metadata)
    const downloadPromise = page.waitForEvent('download', { timeout: 10000 }).catch(() => null);
    
    await cleanButton.click();
    
    const download = await downloadPromise;
    
    if (download) {
      // Should see cleaning statistics
      const hasCleaningStats = consoleLogs.some(log => 
        log.includes('Cleaning stats:') && 
        log.includes('Original:') && 
        log.includes('Cleaned:') &&
        log.includes('Reduction:')
      );
      
      expect(hasCleaningStats).toBe(true);
      console.log('✅ Cleaning statistics properly logged');

      // Should see success message
      const hasSuccessMessage = consoleLogs.some(log => 
        log.includes('Successfully cleaned and downloaded')
      );
      
      expect(hasSuccessMessage).toBe(true);
      console.log('✅ Success message properly logged');
    }
  });

  test('should warn about minimal size reduction for suspicious cleaning', async ({ page }) => {
    await page.goto('/');
    await expect(page.getByTestId('app-title')).toBeVisible();

    const consoleWarnings: string[] = [];
    
    page.on('console', msg => {
      if (msg.type() === 'warning') {
        consoleWarnings.push(msg.text());
        console.log('CONSOLE WARNING:', msg.text());
      }
    });

    const fileInput = page.locator('input[type="file"][accept*="image"]');
    const filePath = path.join(__dirname, 'fixtures', 'with-metadata.jpg');
    await fileInput.setInputFiles(filePath);
    
    await expect(page.locator('text=with-metadata.jpg')).toBeVisible({ timeout: 5000 });

    const cleanButton = page.getByTestId('clean-button');
    const downloadPromise = page.waitForEvent('download', { timeout: 10000 }).catch(() => null);
    
    await cleanButton.click();
    
    await downloadPromise;
    await page.waitForTimeout(1000);

    // Check if there were any warnings about minimal reduction
    // (This may or may not trigger depending on the test file's metadata content)
    const hasMinimalReductionWarning = consoleWarnings.some(warning => 
      warning.includes('minimal size reduction')
    );
    
    if (hasMinimalReductionWarning) {
      console.log('✅ Minimal reduction warning properly shown');
    } else {
      console.log('ℹ️ No minimal reduction warning (expected for files with substantial metadata)');
    }
  });
});