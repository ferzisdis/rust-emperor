#!/usr/bin/env python3
"""
Recolor black and white images using the game's main colors.
Converts images to use --parchment-light (#f9f5ea) and --gold-accent (#c59a37).
"""

from PIL import Image
import os
import sys

# Game colors
PARCHMENT_LIGHT = (249, 245, 234)  # #f9f5ea
GOLD_ACCENT = (63, 52, 38)  # #3f3426


def hex_to_rgb(hex_color):
    """Convert hex color to RGB tuple."""
    hex_color = hex_color.lstrip("#")
    return tuple(int(hex_color[i : i + 2], 16) for i in (0, 2, 4))


def recolor_image(input_path, output_path):
    """
    Recolor a black and white image using the game's color scheme.
    Black pixels become gold, white pixels become parchment.
    Gray values are interpolated between the two colors.
    """
    print(f"Processing: {os.path.basename(input_path)}")

    # Open image and convert to RGBA
    img = Image.open(input_path).convert("RGBA")
    pixels = img.load()
    width, height = img.size

    # Create new image
    new_img = Image.new("RGBA", (width, height))
    new_pixels = new_img.load()

    for y in range(height):
        for x in range(width):
            r, g, b, a = pixels[x, y]

            # Calculate grayscale value (0 = black, 255 = white)
            gray = (r + g + b) / 3

            # Normalize to 0-1 range
            ratio = gray / 255.0

            # Interpolate between gold (dark) and parchment (light)
            # ratio 0 (black) -> gold
            # ratio 1 (white) -> parchment
            new_r = int(GOLD_ACCENT[0] + (PARCHMENT_LIGHT[0] - GOLD_ACCENT[0]) * ratio)
            new_g = int(GOLD_ACCENT[1] + (PARCHMENT_LIGHT[1] - GOLD_ACCENT[1]) * ratio)
            new_b = int(GOLD_ACCENT[2] + (PARCHMENT_LIGHT[2] - GOLD_ACCENT[2]) * ratio)

            # Keep original alpha channel
            new_pixels[x, y] = (new_r, new_g, new_b, a)

    # Save the recolored image
    new_img.save(output_path, "PNG")
    print(f"  Saved to: {os.path.basename(output_path)}")


def main():
    # Directory containing images
    images_dir = "static/images"

    if not os.path.exists(images_dir):
        print(f"Error: Directory '{images_dir}' not found!")
        sys.exit(1)

    # Get all PNG files in the directory
    image_files = [f for f in os.listdir(images_dir) if f.endswith(".png")]

    if not image_files:
        print(f"No PNG files found in '{images_dir}'")
        sys.exit(1)

    print(f"Found {len(image_files)} images to recolor")
    print(f"Colors: Parchment Light {PARCHMENT_LIGHT} <-> Gold Accent {GOLD_ACCENT}")
    print("-" * 60)

    # Process each image
    for filename in sorted(image_files):
        input_path = os.path.join(images_dir, filename)
        output_path = os.path.join(images_dir, filename)  # Overwrite original

        try:
            recolor_image(input_path, output_path)
        except Exception as e:
            print(f"  Error processing {filename}: {e}")

    print("-" * 60)
    print(f"✓ Recoloring complete! Processed {len(image_files)} images.")


if __name__ == "__main__":
    main()
