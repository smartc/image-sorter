#!/bin/bash

# Create samples directory structure if it doesn't exist
mkdir -p samples/Unsorted
mkdir -p samples/Clear
mkdir -p samples/Cloudy
mkdir -p samples/Skip

echo "Directory structure created:"
echo "  samples/"
echo "  ├── Unsorted/"
echo "  ├── Clear/"
echo "  ├── Cloudy/"
echo "  └── Skip/"
echo ""
echo "Place your images in samples/Unsorted/ and then run:"
echo "  cargo run --release"
