#!/bin/bash

# Rust コードの品質チェック（フォーマット + Lint）

FILE_PATH=$(jq -r '.tool_input.file_path' 2>/dev/null)

# Rust ファイル（.rs）の場合のみ実行
if [[ -n "$FILE_PATH" && "$FILE_PATH" == *.rs ]]; then
  cd "$CLAUDE_PROJECT_DIR" || exit 0

  # cargo fmt を実行
  cargo fmt -- "$FILE_PATH" 2>/dev/null || true

  # cargo clippy を実行（すべてのターゲットに対して警告を エラー扱い）
  if ! cargo clippy --all-targets -- -D warnings 2>&1; then
    echo ""
    echo "ℹ️  clippy の警告が検出されました。自動修正を試行します..."
    echo ""

    # 自動修正可能な警告を修正
    if cargo clippy --all-targets --fix --allow-dirty 2>&1; then
      echo ""
      echo "✅ clippy の自動修正が完了しました。修正内容を確認してください。"

      # 修正後、再度チェック
      if ! cargo clippy --all-targets -- -D warnings 2>&1; then
        echo ""
        echo "⚠️  手動修正が必要な警告が残っています。上記のエラーメッセージを確認して修正してください。"
      fi
    else
      echo ""
      echo "❌ clippy --fix の実行に失敗しました。エラーメッセージを確認してください。"
    fi
  fi
fi

exit 0
