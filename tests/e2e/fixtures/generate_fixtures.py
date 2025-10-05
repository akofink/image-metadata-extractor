#!/usr/bin/env python3
"""
Generate test image fixtures with known EXIF data.

This script creates minimal test images for E2E testing.
For more comprehensive EXIF data, use a real camera image or exiftool.
"""

from PIL import Image
import os

# Get the directory where this script is located
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))

def create_simple_jpeg():
    """Create a simple JPEG image (50x50 red square)."""
    img = Image.new('RGB', (50, 50), color='red')
    output_path = os.path.join(SCRIPT_DIR, 'simple.jpg')
    img.save(output_path, 'JPEG', quality=95)
    print(f"Created: {output_path}")

def create_simple_png():
    """Create a simple PNG image (50x50 blue square)."""
    img = Image.new('RGB', (50, 50), color='blue')
    output_path = os.path.join(SCRIPT_DIR, 'simple.png')
    img.save(output_path, 'PNG')
    print(f"Created: {output_path}")

if __name__ == '__main__':
    print("Generating test fixtures...")
    create_simple_jpeg()
    create_simple_png()
    print("\nNote: These are minimal images without EXIF data.")
    print("For comprehensive EXIF testing, add real camera images to this directory.")
