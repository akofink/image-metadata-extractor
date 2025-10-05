# E2E Test Fixtures

This directory contains test image files for end-to-end testing.

## Current Fixtures

Fixtures will be generated automatically when you run the test suite for the first time.

## Manual Fixture Generation

If you want to generate fixtures manually:

### Option 1: Using ImageMagick
```bash
# Create a simple 50x50 red JPEG
convert -size 50x50 xc:red simple.jpg

# Create a simple 50x50 blue PNG
convert -size 50x50 xc:blue simple.png
```

### Option 2: Using Python + Pillow
```bash
pip install Pillow
python3 generate_fixtures.py
```

### Option 3: Use Real Images
Copy real camera images with EXIF data to this directory for comprehensive testing:
- `with-exif.jpg` - Image with full EXIF data
- `with-gps.jpg` - Image with GPS coordinates
- `no-exif.png` - Image without metadata

## Test Requirements

The E2E tests expect at least:
- One image file for basic upload testing (any format)

Additional fixtures improve test coverage but are not required for the first test to pass.
