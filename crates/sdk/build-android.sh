#!/usr/bin/env zsh

set -e
set -u

# NOTE: You MUST run this every time you make changes to the core. Unfortunately, calling this from Xcode directly
# does not work so well.

# In release mode, we create a ZIP archive of the xcframework and update Package.swift with the computed checksum.
# This is only needed when cutting a new release, not for local development.
release=false

for arg in "$@"
do
    case $arg in
        --release)
            release=true
            shift # Remove --release from processing
            ;;
        *)
            shift # Ignore other argument from processing
            ;;
    esac
done

generate_ffi() {
  echo "Generating framework module mapping and FFI bindings"
  # Todo: not architect specific yet
  cargo run -p uniffi-bindgen generate --library target/release/lib$1.dylib --language kotlin --out-dir target/android/uniffi-staging
}

# Build .so file for each architects
cargo ndk -t armeabi-v7a -t arm64-v8a -o target/android/jniLibs build --release
# Build lib rust file
cargo build -p sdk

basename=e2ee_sdk
generate_ffi $basename
