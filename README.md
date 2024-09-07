# RUST-EE2E

## Status
### Core tasks

1. End-to-End Encryption (E2EE)
    - [x] Key generation ✅
    - [x] Key import & export
    - [x] Encryption & Decryption ✅
    - [x] CLI for demonstration - check How to

2. SDK Development
    - [x] Provide bindings for mobile platforms
    - [x] SDK integration guides for mobile
    - [ ] Provide bindings for desktio platforms
    - [ ] SDK integration guides for desktop
    - [ ] Write tests to verify the SDK functionality
3. Deployment
    - [ ] Desktop: Instructions for building and deploying the SDK on a desktop platform.
    - [x] Mobile: Configuration for integrating the SDK with Android and iOS projects.

## Quick start

### Rust

Open up the project in your favorite editor and poke around the Cargo workspace.
All important codes are under `crates/`!

#### Stuff to look at 

* All of the code related to key functionalities is in `crates/crypto`, including several unit tests to demonstrate the Rust API.

### Command-line Interface

Before using the CLI, you need to build the Rust core.

```shell
$ cargo build -p cli
```

For all supported CLI's commands, run

```shell
$ ./target/debug/cli -h

RSA encryption/decryption CLI

Usage: cli <COMMAND>

Commands:
  generate  Generate RSA key pair in pem format
  encrypt   Encrypt a given plaintext message using the public key and encoded in base64
  decrypt   Decrypt the encrypted message in base64 using the private key
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
Example
```shell
$ ./target/debug/cli generate --filename test

Your private key has been saved in test
Your public key has been saved in test.pub

$ ls | grep test
test # Private key
test.pub # Public key
```

```shell
$ ./target/debug/cli encrypt --public-key test.pub --message hello
Ciphertext:
 "output_base64_encoded_ciphertext"

$ ./target/debug/cli decrypt --private-key test \
  --ciphertext "output_base64_encoded_ciphertext"
Plaintext:
 hello
```

### iOS

Before opening up the Swift package in Xcode, you need to build the Rust core.

```shell
cd rust/
./build-ios.sh
```

This generates an XCFramework and generates Swift bindings to the Rust core.
Check the script if you're interested in the gritty details.

**You need to do this every time you make Rust changes that you want reflected in the Swift Package!**

#### Stuff to look at

* `Package.swift` documents the UniFFI setup (which is... special thanks to SPM quirks).
* `SafeCalculator` and `SafeMultiply` in `Sources/Foobar` contain the Swift-y calculator wrapper class and multiplication operator.

### Android

You need to install [`cargo-ndk`](https://github.com/bbqsrc/cargo-ndk).

```shell
cargo install cargo-ndk
```
#### Stuff to look at

* The interesting stuff lives under the `SafeCalculator` and `SafeMultiply` classes.
* Also check out the tests for an example of usage.
* You can also check out the Android tests.
* The gradle files are mostly boilerplate, but there are a few things in there needed for building the Rust library. That took a while to figure out, and I currently believe this is the easiest approach.

## References that are used to build this repo

* [clap](https://github.com/clap-rs/clap) - a crate that gives simple declarative codes to build `CLI`
* [cargo-ndk](https://github.com/bbqsrc/cargo-ndk) - cargo subcommand to build Android's Artifacts
* [UniFFI](https://github.com/mozilla/uniffi-rs) - helps to build Rust binding for mobile platforms
* [uniffi-starter](https://github.com/ianthetechie/uniffi-starter) - most of the idea and build scripts from this repo - **check it out!**