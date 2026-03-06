#!/bin/bash

# Create samples directory structure if it doesn't exist
for tod in day night; do
    mkdir -p samples/$tod/Unsorted
    mkdir -p samples/$tod/Clear
    mkdir -p samples/$tod/Cloudy
    mkdir -p samples/$tod/Skip
done

echo "Directory structure created:"
echo "  samples/"
echo "  ├── day/"
echo "  │   ├── Unsorted/"
echo "  │   ├── Clear/"
echo "  │   ├── Cloudy/"
echo "  │   └── Skip/"
echo "  └── night/"
echo "      ├── Unsorted/"
echo "      ├── Clear/"
echo "      ├── Cloudy/"
echo "      └── Skip/"
echo ""
echo "Place your daytime images in samples/day/Unsorted/"
echo "Place your nighttime images in samples/night/Unsorted/"
echo "Then run:"
echo "  cargo run --release"
