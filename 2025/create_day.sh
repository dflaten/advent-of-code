#!/bin/bash

# Usage: ./create_package.sh <number>

if [ -z "$1" ]; then
    echo "Usage: $0 <number>"
    exit 1
fi

NUMBER=$1
TEMPLATE_DIR="./template"
BASE_NAME="day"
NEW_PACKAGE="advent_of_code_day_${NUMBER}"
DIR_NAME="day${NUMBER}"


# Copy template directory
cp -r "$TEMPLATE_DIR" "$DIR_NAME"

# Update package name in Cargo.toml
sed -i '' "s/name = \".*\"/name = \"$NEW_PACKAGE\"/" "$DIR_NAME/Cargo.toml"

echo "Created package: $NEW_PACKAGE"
