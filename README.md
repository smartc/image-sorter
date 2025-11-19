# Image Sorter

A simple web-based tool to sort images for machine learning datasets.

## Features

- View images one at a time from the `samples/Unsorted` folder
- Categorize images into Clear, Cloudy, or Skip
- Real-time counters for all categories
- Keyboard shortcuts for quick sorting
- Automatically loads the next image after categorization

## Setup

1. Make sure you have Rust installed (https://rustup.rs/)

2. Ensure your directory structure looks like this:
   ```
   samples/
   ├── Unsorted/    (your unsorted images go here)
   ├── Clear/       (will contain clear sky images)
   ├── Cloudy/      (will contain cloudy images)
   └── Skip/        (will contain images to skip)
   ```

3. Run the application:
   ```bash
   cargo run --release
   ```

4. Open your browser to: http://127.0.0.1:3000

## Usage

### Mouse Controls
- Click the **Clear** button to move the current image to `samples/Clear/`
- Click the **Cloudy** button to move the current image to `samples/Cloudy/`
- Click the **Skip** button to move the current image to `samples/Skip/`

### Keyboard Shortcuts
- Press **C** to categorize as Clear
- Press **L** to categorize as Cloudy (L for cLoudy)
- Press **S** to categorize as Skip

### Counters
The header shows real-time counts of:
- Unsorted images (remaining to sort)
- Clear images (total in Clear folder)
- Cloudy images (total in Cloudy folder)
- Skip images (total in Skip folder)

## Supported Image Formats

- JPEG (.jpg, .jpeg)
- PNG (.png)
- GIF (.gif)
- WebP (.webp)

## Notes

- Images are moved (not copied) from Unsorted to the target folder
- The app automatically loads the next image after each categorization
- Existing images in the target folders are counted in the totals
- You can run this in batches - existing categorized images won't be affected
