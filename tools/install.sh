#!/bin/bash

# Colors for terminal output
YELLOW="\033[1;33m"
GREEN="\033[1;32m"
RED="\033[1;31m"
RESET="\033[0m"

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${RESET}"
    echo "This tool requires Rust to be installed."
    echo "Please install Rust using the following command:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build the CLI tool
echo -e "${YELLOW}Building write CLI...${RESET}"
cargo build --release

# Create ~/.local/bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Copy the binary to ~/.local/bin
echo -e "${YELLOW}Installing write CLI to ~/.local/bin...${RESET}"
cp target/release/write ~/.local/bin/write

# Make it executable
chmod +x ~/.local/bin/write

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo -e "${YELLOW}Adding ~/.local/bin to your PATH...${RESET}"
    
    # Determine shell
    SHELL_NAME=$(basename "$SHELL")
    
    if [ "$SHELL_NAME" = "bash" ]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
        echo -e "${GREEN}Added to ~/.bashrc. Please run 'source ~/.bashrc' to update your PATH.${RESET}"
    elif [ "$SHELL_NAME" = "zsh" ]; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
        echo -e "${GREEN}Added to ~/.zshrc. Please run 'source ~/.zshrc' to update your PATH.${RESET}"
    else
        echo -e "${YELLOW}Please add ~/.local/bin to your PATH manually.${RESET}"
    fi
fi

echo -e "${GREEN}Installation complete!${RESET}"
echo -e "You can now use the write CLI by running: ${YELLOW}write${RESET}"
echo -e "For help, run: ${YELLOW}write --help${RESET}"
echo -e "To launch the interactive CLI, run: ${YELLOW}write interactive${RESET}" 