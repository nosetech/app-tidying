#!/bin/bash

# Macで実行中のアプリケーションのウィンドウ情報を取得するスクリプト

# スクリプトのディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# AppleScriptファイルを実行
osascript "${SCRIPT_DIR}/get_window_info.applescript"