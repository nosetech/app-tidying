#!/bin/bash

# Rust ファイルの自動フォーマット

FILE_PATH=$(jq -r '.tool_input.file_path' 2>/dev/null)

# Rust ファイル（.rs）の場合のみ実行
if [[ -n "$FILE_PATH" && "$FILE_PATH" == *.rs ]]; then
  cd "$CLAUDE_PROJECT_DIR" || exit 0

  # cargo fmt を実行
  cargo fmt -- "$FILE_PATH" 2>/dev/null || true
fi

exit 0
