#!/bin/bash

#
# Created by Nick aka black-dragon74
#
# Compiles the release and bundles into a sharable zip
#

# Get the current directory
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

# Format all the files
echo "Formatting the rust files..."
rustfmt "$DIR"/src/* || exit

# Time to run the build
echo "Building executable..."
cargo build --release || exit

# Make a new directory called release in the current directory
echo "Moving items into places..."
mkdir -p "$DIR/Release"

# Copy the compiled binary inside it
cp "$DIR/target/release/pegasus" "$DIR/Release/"

# Copy the PLIST file inside it
cp "$DIR/misc/com.black-dragon74.Pegasus.plist" "$DIR/Release/"

# Copy the installer CMD inside
cp "$DIR/misc/install.sh" "$DIR/Release/install.command"

# CD inside the Release folder
cd "$DIR/Release" || exit

# Zip all the contents of the directory inside it
echo "Bundling release..."
zip -o Release.zip ./* || exit

# Now remove everything except the zip file
shopt -s extglob
rm -rf -- !(Release.zip)

# We are done
echo "All done. Enjoy..."
exit 0
