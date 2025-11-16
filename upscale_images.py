#!/usr/bin/env python3
"""
Upscale pixel art images using nearest-neighbor interpolation for sharp results.
"""

from PIL import Image
import os
from pathlib import Path

# Configuration
SOURCE_DIR = "RustEmperorOriginal/images"
TARGET_DIR = "static/images"
SCALE_FACTOR = 4

def upscale_image(input_path, output_path, scale):
    """Upscale an image using nearest-neighbor interpolation."""
    with Image.open(input_path) as img:
        # Get original size
        original_size = img.size
        new_size = (original_size[0] * scale, original_size[1] * scale)

        # Use NEAREST for sharp pixel art scaling
        upscaled = img.resize(new_size, Image.NEAREST)

        # Save with no compression to maintain quality
        upscaled.save(output_path, 'PNG', optimize=False)

        print(f"✓ {os.path.basename(input_path)}: {original_size[0]}x{original_size[1]} → {new_size[0]}x{new_size[1]}")

def main():
    # Ensure target directory exists
    Path(TARGET_DIR).mkdir(parents=True, exist_ok=True)

    # Get all PNG files from source directory
    source_path = Path(SOURCE_DIR)
    image_files = sorted(source_path.glob("*.png"))

    if not image_files:
        print(f"No PNG files found in {SOURCE_DIR}")
        return

    print(f"Upscaling {len(image_files)} images with {SCALE_FACTOR}x nearest-neighbor scaling...")
    print()

    for img_file in image_files:
        output_file = Path(TARGET_DIR) / img_file.name
        try:
            upscale_image(img_file, output_file, SCALE_FACTOR)
        except Exception as e:
            print(f"✗ Error processing {img_file.name}: {e}")

    print()
    print(f"Done! All images saved to {TARGET_DIR}")

if __name__ == "__main__":
    main()