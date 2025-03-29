#!/bin/sh
# Define the path to your zsh configuration file.
ZSHRC="$HOME/.zshrc"

# Define the export line to be added.
EXPORT_LINE='export PATH="$PATH:/workspaces/Steve/steve/target/debug"'

# Check if the export line already exists in the file.
if grep -Fxq "$EXPORT_LINE" "$ZSHRC"; then
  echo "The export line is already present in $ZSHRC."
else
  echo "$EXPORT_LINE" >> "$ZSHRC"
  echo "Export line added to $ZSHRC."
fi
