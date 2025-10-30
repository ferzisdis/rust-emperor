# Image Upscaling Documentation

## Overview

The original Dark Emperor game includes beautiful black-and-white pixel art images. These images have been upscaled 4x using nearest-neighbor interpolation to make them larger and more visible in the modern web version while maintaining their crisp, sharp pixel art aesthetic.

## Technical Details

### Upscaling Method
- **Tool**: Python 3 with Pillow (PIL) library
- **Interpolation**: `Image.NEAREST` (nearest-neighbor)
- **Scale Factor**: 4x
- **Color Preservation**: Original 1-bit/2-bit colormaps maintained

### Why Nearest-Neighbor?
Nearest-neighbor interpolation is essential for pixel art because:
- ✅ Preserves hard edges and sharp pixels
- ✅ No blurring or smoothing artifacts
- ✅ Maintains the authentic retro pixel art look
- ✅ Each original pixel becomes a clean 4x4 block

### Browser Rendering
The CSS includes specific rules to prevent browsers from smoothing the images:
```css
img {
    image-rendering: -moz-crisp-edges;     /* Firefox */
    image-rendering: -webkit-crisp-edges;  /* Safari (older) */
    image-rendering: pixelated;            /* Chrome, Edge, Safari */
    image-rendering: crisp-edges;          /* Standard */
}
```

## File Locations

- **Original Images**: `RustEmperorOriginal/images/*.png`
- **Upscaled Images**: `static/images/*.png`
- **Upscaling Script**: `upscale_images.py`

## Image Inventory

### Castles (6 levels)
- `deg_castle0.png` - 11×9 → 44×36 pixels
- `deg_castle1.png` - 11×9 → 44×36 pixels
- `deg_castle2.png` - 8×15 → 32×60 pixels
- `deg_castle3.png` - 11×8 → 44×32 pixels
- `deg_castle4.png` - 11×18 → 44×72 pixels
- `deg_castle5.png` - 9×17 → 36×68 pixels

### Resource Icons
- `deg_man.png` - 7×9 → 28×36 pixels (People)
- `deg_food.png` - 7×9 → 28×36 pixels (Food)
- `deg_gold.png` - 7×9 → 28×36 pixels (Gold)
- `deg_iron.png` - 7×9 → 28×36 pixels (Iron)
- `deg_weapons.png` - 7×9 → 28×36 pixels (Weapons)
- `deg_soldier.png` - 7×9 → 28×36 pixels (Soldiers)

### Buildings
- `deg_farm.png` - 10×9 → 40×36 pixels (Farm)
- `deg_market.png` - 10×9 → 40×36 pixels (Market)
- `deg_mine.png` - 10×9 → 40×36 pixels (Mine)
- `deg_smithy.png` - 10×9 → 40×36 pixels (Smithy)

### Scene Images
- `de1.png` - 93×54 → 372×216 pixels (Title screen)
- `de2.png` - 23×18 → 92×72 pixels (Game icon)
- `deg_land.png` - 96×64 → 384×256 pixels (Landscape)
- `deg_win.png` - 96×43 → 384×172 pixels (Victory screen)
- `deg_lost.png` - 96×43 → 384×172 pixels (Defeat screen)

## Re-running the Upscaling

If you need to re-upscale the images or change the scale factor:

```bash
# Install Pillow if needed
pip3 install --user Pillow

# Run the upscaling script
python3 upscale_images.py
```

### Changing Scale Factor
Edit `upscale_images.py` and modify the `SCALE_FACTOR` variable:
```python
SCALE_FACTOR = 4  # Change to 3, 5, 6, etc.
```

## Usage in Templates

Reference images in your HTML templates using:
```html
<img src="/static/images/deg_castle0.png" alt="Castle Level 0" class="pixel-art">
<img src="/static/images/deg_food.png" alt="Food Icon">
```

The `pixel-art` class is optional as all `<img>` tags have crisp rendering by default.

## Testing

View all upscaled images in the browser:
```
http://localhost:8080/static/test_images.html
```

This test page displays all images organized by category with their dimensions.

## File Sizes

The upscaled images remain very small:
- Icon images: ~120-900 bytes each
- Building images: ~800-1000 bytes each
- Scene images: ~9-18 KB each

Total size of all 21 images: ~50 KB

## Quality Comparison

### ❌ Before (blurry scaling)
- Used macOS `sips` default interpolation
- Images appeared smooth/blurry
- Lost pixel art aesthetic

### ✅ After (sharp scaling)
- Python Pillow with NEAREST interpolation
- CSS `image-rendering: pixelated`
- Perfect crisp pixel art appearance
- Each pixel is a clean 4×4 square

## Credits

Original artwork from the Dark Emperor mobile game by Vladimir Galochkin.