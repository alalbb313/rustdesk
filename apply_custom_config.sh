#!/usr/bin/env bash
set -e

# ------------------------------------------------------------
# RustDesk Custom Configuration Auto‑apply Script (Linux/macOS)
# ------------------------------------------------------------
# This script modifies the source files to apply the custom
# configuration described in CUSTOM_CONFIG.md.
# Run it from the repository root after cloning.

# 1. Dart – default options
sed -i '' "s/get defaultOptionAccessMode => .*/get defaultOptionAccessMode => isCustomClient ? 'full' : '';/" flutter/lib/common.dart
sed -i '' "s/get defaultOptionDirectServer => .*/get defaultOptionDirectServer => isCustomClient ? 'Y' : '';/" flutter/lib/common.dart
sed -i '' "s/get defaultOptionCollapseToolbar => .*/get defaultOptionCollapseToolbar => isCustomClient ? 'Y' : '';/" flutter/lib/common.dart

# 2. ServerConfig default values (four lines after the constructor)
# Replace the empty defaults with the custom ones
sed -i '' "/ServerConfig.fromOptions/,+5 s/?? \"\"/?? \"rustdesk.alalbb.top\"/" flutter/lib/common.dart
sed -i '' "/ServerConfig.fromOptions/,+5 s/?? \"\"/?? \"rustdesk.alalbb.top\"/" flutter/lib/common.dart
sed -i '' "/ServerConfig.fromOptions/,+5 s/?? \"\"/?? \"https://rustdesk.alalbb.top:8443\"/" flutter/lib/common.dart
sed -i '' "/ServerConfig.fromOptions/,+5 s/?? \"\"/?? \"rB3CwJAIDVga6SrfrnUgIDfFcAAiX2+V4xBZXMAKsjU=\"/" flutter/lib/common.dart

# 3. Rust – default permanent password
sed -i '' "s/password = .*/password = \"Jerry@313\".to_string();/" libs/hbb_common/src/config.rs

echo "Custom configuration applied successfully."
