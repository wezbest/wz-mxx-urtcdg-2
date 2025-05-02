#!/usr/bin/bash
# This bash srcript is for installing the KL docker image here
clear

# Colors
export RED='\033[0;31m'
export GREEN='\033[0;32m'
export YELLOW='\033[0;33m'
export BLUE='\033[0;34m'
export PURPLE='\033[0;35m'
export CYAN='\033[0;36m'
export WHITE='\033[0;37m'
export NC='\033[0m' # No Color

# Commands

h1() {
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
    echo -e "${PURPLE}$1${NC}"
    echo -e "${CYAN}~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~${NC}"
}

build_linux() {
    h1 "Building for Linux..."

    # Check if cargo is installed
    if ! command -v cargo &>/dev/null; then
        echo -e "${RED}Error: cargo (Rust) is not installed!${NC}"
        return 1
    fi

    # Build for GNU target (standard Linux)
    h1 "Building for x86_64-unknown-linux-gnu"
    if cargo build --release --target x86_64-unknown-linux-gnu; then
        echo -e "${GREEN}GNU build successful!${NC}"
        echo -e "Binary location: ${YELLOW}target/x86_64-unknown-linux-gnu/release/mw1${NC}"
    else
        echo -e "${RED}GNU build failed!${NC}"
        return 1
    fi

    # Check if musl target is installed
    if ! rustup target list | grep -q "x86_64-unknown-linux-musl (installed)"; then
        h1 "Installing MUSL target..."
        if ! rustup target add x86_64-unknown-linux-musl; then
            echo -e "${RED}Failed to install MUSL target!${NC}"
            return 1
        fi
    fi

    # Build for MUSL target (statically linked)
    h1 "Building for x86_64-unknown-linux-musl (static)"
    if cargo build --release --target x86_64-unknown-linux-musl; then
        echo -e "${GREEN}MUSL build successful!${NC}"
        echo -e "Binary location: ${YELLOW}target/x86_64-unknown-linux-musl/release/mw1${NC}"
    else
        echo -e "${RED}MUSL build failed!${NC}"
        return 1
    fi

    # Optional: Strip binaries to reduce size
    h1 "Stripping binaries..."
    if strip target/x86_64-unknown-linux-gnu/release/mw1 2>/dev/null; then
        echo -e "${GREEN}GNU binary stripped${NC}"
    else
        echo -e "${YELLOW}Warning: Failed to strip GNU binary${NC}"
    fi

    if strip target/x86_64-unknown-linux-musl/release/mw1 2>/dev/null; then
        echo -e "${GREEN}MUSL binary stripped${NC}"
    else
        echo -e "${YELLOW}Warning: Failed to strip MUSL binary${NC}"
    fi

    echo -e "\n${GREEN}All builds completed successfully!${NC}"
    return 0
}

# Excution
build_linux
