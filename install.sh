#!/bin/bash

# Check for debug flag
if [[ "$1" == "--debug" || "$1" == "-d" ]]; then
    echo "Building claude-statusline with debug_json feature..."
    cargo build --release --features debug_json
    echo "Debug mode enabled: JSON will be logged to /tmp/claude-code-statusline-debug-*.log"
else
    # Build optimized release binary
    echo "Building claude-statusline..."
    cargo build --release
fi

# Create bin directory if it doesn't exist
mkdir -p ~/.claude/bin

# Copy binary to ~/.claude/bin
echo "Installing to ~/.claude/bin/statusline..."
cp target/release/claude-statusline ~/.claude/bin/statusline

# Make it executable (should already be, but just in case)
chmod +x ~/.claude/bin/statusline

echo "Installation complete!"

if [[ "$1" == "--debug" || "$1" == "-d" ]]; then
    echo ""
    echo "Debug logs will be written to:"
    echo "  - Status: /tmp/claude-code-statusline-debug-status.log"
    echo "  - Transcript: /tmp/claude-code-statusline-debug-transcript.log"
fi

echo ""
echo "Update your ~/.claude/settings.json to use:"
echo '  "statusLine": {'
echo '    "type": "command",'
echo '    "command": "/Users/'$USER'/.claude/bin/statusline"'
echo '  }'