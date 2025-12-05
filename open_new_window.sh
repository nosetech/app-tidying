#!/bin/bash

# 指定したアプリケーションの新規ウィンドウを開くスクリプト
# 使用方法: ./open_new_window.sh Finder|Safari|Chrome

set -e

# カラー定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# ログ関数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# 使用方法表示
usage() {
    cat << 'EOF'
使用方法: ./open_new_window.sh <APP_NAME>

APP_NAME:
  Finder    - Finder の新規ウィンドウを開く
  Safari    - Safari の新規ウィンドウを開く
  Chrome    - Google Chrome の新規ウィンドウを開く

例:
  ./open_new_window.sh Finder
  ./open_new_window.sh Safari
  ./open_new_window.sh Chrome
EOF
}

# 新規ウィンドウを開く関数
open_new_window() {
    local app_name=$1

    # アプリケーション名の正規化
    case "$app_name" in
        Finder)
            local display_name="Finder"
            local process_name="Finder"
            ;;
        Safari)
            local display_name="Safari"
            local process_name="Safari"
            ;;
        Chrome|Google\ Chrome)
            local display_name="Google Chrome"
            local process_name="Google Chrome"
            ;;
        *)
            log_error "サポートされていないアプリケーション: $app_name"
            usage
            return 1
            ;;
    esac

    log_info "$display_name の新規ウィンドウを開きます..."

    # アプリケーションが起動しているか確認
    if ! pgrep -q "$process_name"; then
        log_warning "$display_name が起動していません。起動します..."
        open -a "$display_name"
        sleep 2
    fi

    # AppleScript で新規ウィンドウを開く
    local result
    result=$(osascript << APPLESCRIPT
tell application "System Events"
  tell process "$process_name"
    try
      -- メニューバーを取得
      set menubar to menu bar 1
      set allMenus to (every menu of menubar)
      set fileMenu to {}

      -- File または ファイル メニューを検索
      repeat with m in allMenus
        if name of m is "File" or name of m is "ファイル" then
          set fileMenu to m
          exit repeat
        end if
      end repeat

      if fileMenu is {} then
        return "Error: File menu not found"
      end if

      -- メニューアイテムを取得
      set menuItems to (every menu item of fileMenu)

      -- 「New Window」「新規ウインドウ」を検索してクリック
      repeat with mi in menuItems
        try
          set itemName to name of mi
          -- 英語と日本語の両方に対応
          if (itemName contains "New Window") or (itemName contains "新規ウインドウ") then
            click mi
            return "Success: New window opened"
          end if
        end try
      end repeat

      return "Error: New window menu item not found"
    on error err_msg
      return "Error: " & err_msg
    end try
  end tell
end tell
APPLESCRIPT
    )

    # 結果処理
    if [[ "$result" == "Success"* ]]; then
        log_success "$display_name の新規ウィンドウを開きました"
        return 0
    else
        log_error "$result"
        return 1
    fi
}

# メイン処理
main() {
    if [[ $# -eq 0 ]]; then
        log_error "アプリケーション名を指定してください"
        usage
        return 1
    fi

    local app_name=$1
    open_new_window "$app_name"
}

# スクリプト実行
main "$@"
