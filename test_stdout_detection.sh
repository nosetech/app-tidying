#!/bin/bash

# 標準出力判定機能をテストするためのスクリプト
# 様々な実行環境をシミュレートして、判定の正確性を確認する

echo "======================================"
echo "標準出力判定テストスイート"
echo "======================================"
echo ""

SCRIPT_PATH="./detect_stdout.sh"

# テスト1: ターミナルでの通常実行（標準出力が利用可能）
echo "[テスト1] ターミナルでの通常実行"
echo "期待値: 標準出力が利用可能"
echo "実行結果:"
bash "$SCRIPT_PATH" 2>&1 | head -3
echo ""

# テスト2: パイプ経由での実行（標準出力が利用不可をシミュレート）
echo "[テスト2] パイプ経由での実行"
echo "期待値: ダイアログが表示される可能性がある"
echo "実行結果:"
bash "$SCRIPT_PATH" 2>&1 | cat
echo ""

# テスト3: ファイルへのリダイレクト（標準出力が利用不可をシミュレート）
echo "[テスト3] ファイルへのリダイレクト"
echo "期待値: ダイアログが表示される可能性がある"
TEMP_FILE="/tmp/stdout_test_$$"
bash "$SCRIPT_PATH" > "$TEMP_FILE" 2>&1
echo "リダイレクト結果:"
cat "$TEMP_FILE" | head -3
rm -f "$TEMP_FILE"
echo ""

# テスト4: stderr のみへの出力（stdout が閉じられている）
echo "[テスト4] stdout が閉じられている状態"
echo "期待値: stderr に出力される"
bash "$SCRIPT_PATH" > /dev/null 2>&1 || true
echo "（ダイアログが表示されている可能性があります）"
echo ""

# テスト5: 環境変数が空の場合
echo "[テスト5] TERM が未設定の環境"
echo "期待値: 標準出力が利用不可と判定される可能性がある"
TERM="" bash "$SCRIPT_PATH" 2>&1 | head -3
echo ""

echo "======================================"
echo "テスト完了"
echo "======================================"
