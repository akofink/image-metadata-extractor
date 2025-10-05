# Add Playwright E2E Tests

## Goal

Add comprehensive end-to-end tests using Playwright to prevent feature regressions and ensure the application works correctly across different browsers.

## Context

This is a Rust + WebAssembly image metadata extraction tool built with Yew. The application runs entirely client-side in the browser with no backend. We need E2E tests to catch regressions in:

- File upload and processing
- Metadata extraction and display
- Export functionality (JSON, CSV, TXT, XML, YAML, Markdown)
- Image cleaning and download
- Batch processing
- Mobile responsiveness
- Browser compatibility (Chrome, Firefox, Safari)

## Current Testing Setup

**Existing Tests**:
- Unit tests in `tests/` directory (197 tests, 80% coverage)
- Tests cover core logic (EXIF parsing, exports, GPS, privacy, etc.)
- No E2E tests for UI or integration flows

**Development Setup**:
- Build: `make build` or `wasm-pack build --target web`
- Dev server: `make serve` (uses npx serve, python, or basic-http-server)
- Local URL: `http://localhost:8000`

## Requirements

### 1. Playwright Setup

**Installation**:
```bash
npm init playwright@latest
# or
pnpm create playwright
```

**Configuration** (`playwright.config.ts`):
- Test against Chromium, Firefox, WebKit (Safari)
- Base URL: `http://localhost:8000`
- Headless mode for CI
- Screenshots on failure
- Video recording on failure
- Mobile viewport tests (iPhone, Android)

**Project Structure**:
```
tests/e2e/
  ├── fixtures/
  │   ├── sample.jpg        # Image with EXIF data
  │   ├── no-exif.png       # Image without EXIF
  │   └── gps-photo.jpg     # Image with GPS coordinates
  ├── upload.spec.ts        # File upload tests
  ├── metadata.spec.ts      # Metadata display tests
  ├── export.spec.ts        # Export functionality tests
  ├── cleaning.spec.ts      # Image cleaning tests
  ├── batch.spec.ts         # Batch processing tests
  └── mobile.spec.ts        # Mobile responsiveness tests
```

### 2. Test Coverage

#### A. File Upload Tests (`upload.spec.ts`)

**Happy Path**:
- [x] Single file upload shows metadata
- [x] Multiple file upload shows batch progress
- [x] Drag-and-drop file upload works
- [x] "Choose Image" button triggers file input

**Edge Cases**:
- [x] Unsupported file type shows error
- [x] Empty file shows error
- [x] Very large file (10MB+) processes successfully
- [x] Cancel file selection (no crash)

**Assertions**:
- File name displayed correctly
- File size formatted (KB, MB)
- Image preview renders
- Processing completes within reasonable time (<5s for typical image)

#### B. Metadata Display Tests (`metadata.spec.ts`)

**Core Functionality**:
- [x] EXIF data displayed in categories
- [x] GPS coordinates shown with map links
- [x] Metadata sorted alphabetically within categories
- [x] Field explanations toggle on/off
- [x] Privacy risk analysis shown
- [x] Metadata consistency warnings displayed

**Categories to Test**:
- Camera Settings
- Image Properties
- Location (GPS)
- Timestamps
- Software/Processing
- Other Metadata

**Interactive Elements**:
- [x] Explanation tooltips work
- [x] Map links open correctly
- [x] Categories expand/collapse
- [x] Select/deselect all checkboxes

#### C. Export Tests (`export.spec.ts`)

**Export Formats**:
- [x] JSON export with selected fields
- [x] CSV export with headers
- [x] TXT export human-readable
- [x] XML export valid structure
- [x] YAML export valid syntax
- [x] Markdown export formatted correctly

**Export Features**:
- [x] Select/deselect individual fields
- [x] Select/deselect all fields (global + per-category)
- [x] Include/exclude file info
- [x] Include/exclude GPS data
- [x] Export buttons disabled when no fields selected
- [x] Download triggers with correct filename
- [x] Empty values omitted from JSON export

**Browser Support**:
- [x] Chrome/Edge: `showSaveFilePicker` native dialog
- [x] Firefox/Safari: Fallback to traditional download
- [x] Mobile: Downloads work correctly

#### D. Image Cleaning Tests (`cleaning.spec.ts`)

**Functionality**:
- [x] Clean single image removes metadata
- [x] Downloaded image has no EXIF
- [x] Original format preserved (JPEG → JPEG)
- [x] Image quality preserved
- [x] Filename includes "-cleaned" suffix
- [x] Format conversion works (JPEG → PNG)
- [x] Quality slider affects file size

**Supported Formats**:
- [x] JPEG cleaning
- [x] PNG cleaning
- [x] WebP cleaning
- [x] TIFF cleaning
- [x] Other formats show appropriate messaging

#### E. Batch Processing Tests (`batch.spec.ts`)

**Multi-file Operations**:
- [x] Upload 10 images shows progress bar
- [x] Progress updates correctly (1/10, 2/10, etc.)
- [x] All files processed successfully
- [x] Can view each file's metadata
- [x] Batch export to JSON array
- [x] Batch ZIP download with cleaned images
- [x] Cancel batch operation (if implemented)

**Performance**:
- [x] UI stays responsive during batch processing
- [x] No crashes with 20+ images
- [x] Memory doesn't spike excessively

#### F. Mobile Tests (`mobile.spec.ts`)

**Responsive Design**:
- [x] Layout adapts to mobile viewport
- [x] Touch-friendly button sizes (44px minimum)
- [x] No horizontal scrolling
- [x] Metadata cards stack vertically
- [x] Export dropdowns work on touch
- [x] Image modal works on mobile

**Mobile Browsers**:
- [x] iOS Safari (iPhone 13)
- [x] Chrome Android (Pixel 5)
- [x] Touch gestures work correctly

### 3. CI Integration

**GitHub Actions** (`.github/workflows/e2e.yml`):
```yaml
name: E2E Tests

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: jetli/wasm-pack-action@v0.4.0

      - name: Build WASM
        run: make build-release

      - name: Install dependencies
        run: npm ci

      - name: Install Playwright browsers
        run: npx playwright install --with-deps

      - name: Run E2E tests
        run: npm run test:e2e

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: playwright-report
          path: playwright-report/
```

### 4. Test Utilities and Helpers

**Common Helpers** (`tests/e2e/helpers.ts`):

```typescript
import { Page } from '@playwright/test';

export async function uploadFile(page: Page, filePath: string) {
  await page.setInputFiles('input[type="file"]', filePath);
}

export async function waitForMetadata(page: Page) {
  await page.waitForSelector('[data-testid="metadata-display"]');
}

export async function selectExportFields(page: Page, fields: string[]) {
  for (const field of fields) {
    await page.check(`[data-testid="export-field-${field}"]`);
  }
}

export async function downloadFile(page: Page, buttonSelector: string): Promise<string> {
  const [download] = await Promise.all([
    page.waitForEvent('download'),
    page.click(buttonSelector),
  ]);
  return await download.path();
}
```

**Fixture Generation** (`scripts/generate_test_fixtures.sh`):
- Create sample images with known EXIF data
- Use `exiftool` to embed test metadata
- Document expected values for assertions

### 5. Data Test IDs

Add `data-testid` attributes to key UI elements for reliable selection:

**File Upload**:
- `data-testid="choose-image-button"`
- `data-testid="file-input"`
- `data-testid="drop-zone"`

**Metadata Display**:
- `data-testid="metadata-display"`
- `data-testid="metadata-category-{category}"`
- `data-testid="metadata-field-{field}"`
- `data-testid="explanation-toggle"`
- `data-testid="privacy-risk"`

**Export**:
- `data-testid="export-dropdown"`
- `data-testid="export-json-button"`
- `data-testid="export-csv-button"`
- `data-testid="export-field-{field}"`
- `data-testid="select-all-checkbox"`

**Image Cleaning**:
- `data-testid="clean-button"`
- `data-testid="format-select"`
- `data-testid="quality-slider"`

**Batch**:
- `data-testid="batch-progress"`
- `data-testid="batch-zip-button"`

### 6. Example Test

```typescript
import { test, expect } from '@playwright/test';
import { uploadFile, waitForMetadata } from './helpers';

test.describe('Metadata Extraction', () => {
  test('should extract and display EXIF data from JPEG', async ({ page }) => {
    await page.goto('/');

    // Upload file
    await uploadFile(page, 'tests/e2e/fixtures/sample.jpg');
    await waitForMetadata(page);

    // Verify file info
    await expect(page.getByTestId('file-name')).toContainText('sample.jpg');
    await expect(page.getByTestId('file-size')).toContainText('KB');

    // Verify metadata categories
    await expect(page.getByTestId('metadata-category-camera')).toBeVisible();
    await expect(page.getByTestId('metadata-category-location')).toBeVisible();

    // Verify specific fields
    await expect(page.getByTestId('metadata-field-Make')).toContainText('Canon');
    await expect(page.getByTestId('metadata-field-Model')).toContainText('EOS');

    // Verify GPS coordinates
    const gpsField = page.getByTestId('metadata-field-gps');
    await expect(gpsField).toContainText('°');

    // Verify map link
    const mapLink = page.getByTestId('map-link');
    await expect(mapLink).toHaveAttribute('href', /google\.com\/maps/);
  });

  test('should handle images without EXIF gracefully', async ({ page }) => {
    await page.goto('/');
    await uploadFile(page, 'tests/e2e/fixtures/no-exif.png');
    await waitForMetadata(page);

    await expect(page.getByText('No metadata found')).toBeVisible();
  });
});
```

## Implementation Steps

1. **Setup** (30 min)
   - Install Playwright and dependencies
   - Create `playwright.config.ts`
   - Create test directory structure
   - Generate test fixtures

2. **Add Data Test IDs** (1-2 hours)
   - Update components with `data-testid` attributes
   - Ensure consistent naming convention
   - Document test IDs in component comments

3. **Write Core Tests** (4-6 hours)
   - File upload tests
   - Metadata display tests
   - Export tests (all formats)
   - Image cleaning tests

4. **Write Advanced Tests** (2-3 hours)
   - Batch processing tests
   - Mobile responsiveness tests
   - Cross-browser compatibility tests

5. **CI Integration** (1 hour)
   - Create GitHub Actions workflow
   - Configure artifacts upload
   - Test workflow on PR

6. **Documentation** (30 min)
   - Update README with test instructions
   - Document test fixtures and data
   - Add troubleshooting guide

## Success Criteria

- [ ] All core user flows covered by E2E tests
- [ ] Tests pass on Chromium, Firefox, WebKit
- [ ] Tests pass on mobile viewports
- [ ] CI pipeline runs tests automatically
- [ ] <5 minute test suite execution time
- [ ] Clear test failure messages
- [ ] Test coverage report generated
- [ ] Zero flaky tests (retry if needed)

## Notes

- Use `test.slow()` for tests involving large file uploads
- Mock network requests if any (should be none for this app)
- Use `screenshot: 'only-on-failure'` to debug issues
- Consider visual regression testing with Percy/Chromatic (optional)
- Test both happy paths and error cases
- Ensure tests are deterministic and don't depend on timing

## References

- Playwright Docs: https://playwright.dev/
- Playwright Best Practices: https://playwright.dev/docs/best-practices
- WebAssembly Testing: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/testing.html
- GitHub Actions for Playwright: https://playwright.dev/docs/ci-intro
