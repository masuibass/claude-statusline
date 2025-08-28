#!/bin/bash

# Build optimized release binary
echo "Building claude-statusline..."
cargo build --release

# Create bin directory if it doesn't exist
mkdir -p ~/.claude/bin

# Copy binary to ~/.claude/bin
echo "Installing to ~/.claude/bin/statusline..."
cp target/release/claude-statusline ~/.claude/bin/statusline

# Make it executable (should already be, but just in case)
chmod +x ~/.claude/bin/statusline

echo "Installation complete!"
echo "Update your ~/.claude/settings.json to use:"
echo '  "command": "/Users/'$USER'/.claude/bin/statusline"'