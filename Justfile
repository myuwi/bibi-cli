TARGET_DIR := "/usr/local/bin"

# List available commands
@list:
  just --list

# Build Bibi in release mode
@build:
    cargo build --release

# Clean build artifacts
@clean:
    cargo clean

# Build and Install Bibi
@install: build
    sudo install -Dsm755 target/release/bibi -t {{TARGET_DIR}}

# Uninstall Bibi
@uninstall:
    sudo rm "{{TARGET_DIR}}/bibi"
