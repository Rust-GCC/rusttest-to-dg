#!/usr/bin/env bash

set -e

# if GCCRS_PATH is set
if [ -z "$GCCRS_PATH" ]; then
  echo "Error: GCCRS_PATH environment variable is not set."
  echo -e "Please set it using the command:\n\texport GCCRS_PATH=/path/to/gccrs"
  exit 1
fi

# if RUST_PATH is set
if [ -z "$RUST_PATH" ]; then
  echo "Error: GCCRS_PATH environment variable is not set."
  echo -e "Please set it using the command:\n\texport GCCRS_PATH=/path/to/gccrs"
  exit 1
fi

# check for rusttest-to-dg path
if [ -z "$RUSTTEST_TO_DG_PATH" ]; then
  echo "Error: RUSTTEST_TO_DG_PATH environment variable is not set."
  echo -e "Please set it using the command:\n\texport RUSTTEST_TO_DG_PATH=/path/to/rusttest-to-dg"
  exit 1
fi

echo "GCCRS_PATH: $GCCRS_PATH"
echo "RUST_PATH: $RUST_PATH"
echo "RUSTTEST_TO_DG_PATH: $RUSTTEST_TO_DG_PATH"

# Installing rusttest-to-dg
cd $RUSTTEST_TO_DG_PATH
echo -e "\nInstalling rusttest-to-dg\n"
cargo install --path .
echo -e "\nInstalled rusttest-to-dg\n"


# Check if the ui directory exists and remove it if it does
if [ -d "$GCCRS_PATH/gcc/testsuite/rust/rustc/ui" ]; then
  echo "Removing existing ui directory at $GCCRS_PATH/gcc/testsuite/rust/rustc/ui"
  rm -rf "$GCCRS_PATH/gcc/testsuite/rust/rustc/ui"
fi



# Copying the RUST_PATH/tests/ui to GCCRS_PATH/gcc/testsuite/rust/rustc
echo -e "Copying tests from $RUST_PATH/tests/ui to $GCCRS_PATH/gcc/testsuite/rust/rustc"
cp -r $RUST_PATH/tests/ui $GCCRS_PATH/gcc/testsuite/rust/rustc
echo -e "Copied $RUST_PATH/tests/ui tests to $GCCRS_PATH/gcc/testsuite/rust/rustc/ui"
cd $GCCRS_PATH/gcc/testsuite/rust/rustc/ui

process_files() {
    # Recursively process rust files
    for file in "$1"/*.rs; do
        if [[ -f "$file" ]]; then
            base_name="${file%.rs}"
            stderr_file="${base_name}.stderr"
            output_file="${base_name}_dg.rs"

            # if we have `.stderr` file
            if [[ -f "$stderr_file" ]]; then
                rusttest-to-dg --file "$file" --stderr "$stderr_file" > "$output_file"
            else
                rusttest-to-dg --file "$file" > "$output_file"
            fi
            mv $output_file $file
        fi
    done

    # Recursively process subdirectories
    for dir in "$1"/*/; do
        if [[ -d "$dir" ]]; then
            process_files "$dir"
        fi
    done
}

echo -e "Converting rustc source files to DejaGnu format..."
process_files "$GCCRS_PATH/gcc/testsuite/rust/rustc/ui/"

# Remove all files that don't end with .rs extension in the ui directory
echo "Removing non-.rs files in $GCCRS_PATH/gcc/testsuite/rust/rustc/ui"
find "$GCCRS_PATH/gcc/testsuite/rust/rustc/ui" -type f ! -name '*.rs' -exec rm -f {} +
echo "Removed non-.rs files in $GCCRS_PATH/gcc/testsuite/rust/rustc/ui"

echo -e "Processing complete."