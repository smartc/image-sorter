# Sky Image Sorter

A web-based utility for sorting sky images to build machine learning datasets. Designed for classifying all-sky camera images into clear, cloudy, and skip categories, making it easy to prepare training data for weather and cloud detection models.

## Features

- View and sort images one at a time from the `samples/Unsorted` folder
- Categorize images into Clear Sky, Cloudy, or Skip
- **Review mode**: Review and recategorize images from any folder (Unsorted, Clear, Cloudy, or Skip)
- Real-time counters for all categories
- Keyboard shortcuts for quick sorting
- Automatically loads the next image after categorization
- Move images between folders to fix mistakes or refine classifications

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

### Folder Selection
- Use the folder selector dropdown to choose which folder to review
- **Unsorted** (default): Sort new images into categories
- **Clear**, **Cloudy**, **Skip**: Review and recategorize previously sorted images
- Switch between folders anytime to fix mistakes or refine your classifications

### Mouse Controls
- Click the **Clear** button to move the current image to `samples/Clear/`
- Click the **Cloudy** button to move the current image to `samples/Cloudy/`
- Click the **Skip** button to move the current image to `samples/Skip/`
- Click the **Unsorted** button (when reviewing sorted images) to move back to `samples/Unsorted/`

### Keyboard Shortcuts
- Press **C** to categorize as Clear
- Press **L** to categorize as Cloudy (L for cLoudy)
- Press **S** to categorize as Skip
- Press **U** to move back to Unsorted (when reviewing sorted images)

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
