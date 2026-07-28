#!/bin/bash

# アプリケーション標準メニューの「ウインドウ」メニューにある
# 「移動とサイズ変更」「[ディスプレイ名]に移動」機能をosascriptで
# コントロールできるかの検証（Issue #116）
#
# 検証項目:
# 1. 「ウインドウ」メニューのメニュー構造にアクセス可能か
# 2. 「移動とサイズ変更」サブメニューの項目一覧を取得できるか
# 3. 「移動とサイズ変更」のメニュー項目をクリックして実際にウィンドウが
#    タイル配置されるか（座標変化を検証）
# 4. 「[ディスプレイ名]に移動」メニュー項目を検出できるか
# 5. 「[ディスプレイ名]に移動」をクリックして実際にウィンドウが
#    別ディスプレイへ移動するか（座標変化を検証）
# 6. Finder / Safari / Google Chrome でのメニュー構成の違いを確認
#
# 注意: 「ウインドウ」メニューの表記はアプリごとに揺れがある
#       （Finder/Safari: "ウインドウ"、Chrome: "ウィンドウ" など）ため、
#       各AppleScript内で "ウインドウ"/"ウィンドウ"/"Window" を順に探索する。

# カラー定義
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

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

log_test() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
}

# テスト1: 「ウインドウ」メニューの存在確認とメニュー項目一覧の取得
test_window_menu_structure() {
    local app_name="$1"
    local process_name="$2"

    log_test "テスト1: ${app_name} の「ウインドウ」メニュー構造確認"

    if ! pgrep -q "${process_name}"; then
        log_warning "${app_name} が起動していません。スキップします。"
        return 1
    fi

    local result
    result=$(osascript << APPLESCRIPT
tell application "System Events"
  tell process "${process_name}"
    try
      set menubar to menu bar 1
      set allMenus to (every menu of menubar)
      set winMenu to missing value
      repeat with m in allMenus
        set n to (name of m)
        if n is "ウインドウ" or n is "ウィンドウ" or n is "Window" then
          set winMenu to m
          exit repeat
        end if
      end repeat

      if winMenu is missing value then
        return "Error: ウインドウメニューが見つかりません"
      end if

      set menuItems to (every menu item of winMenu)
      set itemList to ""
      repeat with mi in menuItems
        try
          set n to (name of mi)
          if n is not missing value then
            if itemList is "" then
              set itemList to n
            else
              set itemList to itemList & ", " & n
            end if
          end if
        end try
      end repeat
      return "Success: " & (name of winMenu) & " | " & itemList
    on error err_msg
      return "Error: " & err_msg
    end try
  end tell
end tell
APPLESCRIPT
    )

    if [[ "$result" == "Success:"* ]]; then
        log_success "${app_name}: ${result}"
    else
        log_error "${app_name}: ${result}"
    fi
}

# テスト2: 「移動とサイズ変更」サブメニューをクリックし、実際にウィンドウが変化するか検証
test_move_and_resize() {
    local app_name="$1"
    local process_name="$2"

    log_test "テスト2: ${app_name} の「移動とサイズ変更」検証"

    if ! pgrep -q "${process_name}"; then
        log_warning "${app_name} が起動していません。スキップします。"
        return 1
    fi

    local before
    before=$(osascript -e "tell application \"System Events\" to tell process \"${process_name}\" to return {position of window 1, size of window 1}" 2>&1)

    if [[ "$before" == *"エラー"* || "$before" == *"Error"* || -z "$before" ]]; then
        log_warning "${app_name}: ウィンドウが見つからないためスキップします（${before}）"
        return 1
    fi
    log_info "${app_name} クリック前の位置・サイズ: ${before}"

    local result
    result=$(osascript << APPLESCRIPT
tell application "${app_name}" to activate
delay 0.3
tell application "System Events"
  tell process "${process_name}"
    try
      set menubar to menu bar 1
      set allMenus to (every menu of menubar)
      set winMenu to missing value
      repeat with m in allMenus
        set n to (name of m)
        if n is "ウインドウ" or n is "ウィンドウ" or n is "Window" then
          set winMenu to m
          exit repeat
        end if
      end repeat

      if winMenu is missing value then
        return "Error: ウインドウメニューが見つかりません"
      end if

      set mrItem to missing value
      repeat with mi in (every menu item of winMenu)
        set n to (name of mi)
        if n is "移動とサイズ変更" or n is "Move & Resize" then
          set mrItem to mi
          exit repeat
        end if
      end repeat

      if mrItem is missing value then
        return "Error: 移動とサイズ変更メニューが見つかりません"
      end if

      set subMenu to menu 1 of mrItem
      set leftItem to missing value
      repeat with mi in (every menu item of subMenu)
        set n to (name of mi)
        if n is "左" or n is "Left" then
          set leftItem to mi
          exit repeat
        end if
      end repeat

      if leftItem is missing value then
        return "Error: 左メニュー項目が見つかりません"
      end if

      click leftItem
      return "Success"
    on error err_msg
      return "Error: " & err_msg
    end try
  end tell
end tell
APPLESCRIPT
    )

    if [[ "$result" != "Success"* ]]; then
        log_error "${app_name}: 「移動とサイズ変更」＞「左」のクリックに失敗（${result}）"
        return 1
    fi

    sleep 1
    local after
    after=$(osascript -e "tell application \"System Events\" to tell process \"${process_name}\" to return {position of window 1, size of window 1}" 2>&1)
    log_info "${app_name} クリック後の位置・サイズ: ${after}"

    if [[ "$before" == "$after" ]]; then
        log_error "${app_name}: メニュークリック後もウィンドウの位置・サイズが変化しませんでした"
    else
        log_success "${app_name}: 「移動とサイズ変更」＞「左」で実際にウィンドウが変化しました"
    fi
}

# テスト3: 「[ディスプレイ名]に移動」メニュー項目の検出とクリック検証
test_move_to_display() {
    local app_name="$1"
    local process_name="$2"

    log_test "テスト3: ${app_name} の「[ディスプレイ名]に移動」検証"

    if ! pgrep -q "${process_name}"; then
        log_warning "${app_name} が起動していません。スキップします。"
        return 1
    fi

    local before
    before=$(osascript -e "tell application \"System Events\" to tell process \"${process_name}\" to return {position of window 1, size of window 1}" 2>&1)
    log_info "${app_name} クリック前の位置・サイズ: ${before}"

    local result
    result=$(osascript << APPLESCRIPT
tell application "${app_name}" to activate
delay 0.3
tell application "System Events"
  tell process "${process_name}"
    try
      set menubar to menu bar 1
      set allMenus to (every menu of menubar)
      set winMenu to missing value
      repeat with m in allMenus
        set n to (name of m)
        if n is "ウインドウ" or n is "ウィンドウ" or n is "Window" then
          set winMenu to m
          exit repeat
        end if
      end repeat

      if winMenu is missing value then
        return "Error: ウインドウメニューが見つかりません"
      end if

      -- ディスプレイ名は動的に変わるため「〜に移動」で終わる項目を部分一致検索する
      -- （「タブを新しいウインドウに移動」等の項目は除外する）
      set moveItem to missing value
      set moveItemName to ""
      repeat with mi in (every menu item of winMenu)
        set n to (name of mi)
        if n is not missing value and n ends with "に移動" and n does not contain "タブ" then
          set moveItem to mi
          set moveItemName to n
          exit repeat
        end if
      end repeat

      if moveItem is missing value then
        return "NotFound"
      end if

      click moveItem
      return "Success: " & moveItemName
    on error err_msg
      return "Error: " & err_msg
    end try
  end tell
end tell
APPLESCRIPT
    )

    if [[ "$result" == "NotFound" ]]; then
        log_warning "${app_name}: 「[ディスプレイ名]に移動」メニュー項目が見つかりません（外部ディスプレイ未接続、またはアプリ非対応の可能性）"
        return 1
    fi

    if [[ "$result" != "Success:"* ]]; then
        log_error "${app_name}: 「[ディスプレイ名]に移動」のクリックに失敗（${result}）"
        return 1
    fi

    log_success "${app_name}: メニュー項目「${result#Success: }」を検出・クリックしました"

    sleep 1
    local after
    after=$(osascript -e "tell application \"System Events\" to tell process \"${process_name}\" to return {position of window 1, size of window 1}" 2>&1)
    log_info "${app_name} クリック後の位置・サイズ: ${after}"

    if [[ "$before" == "$after" ]]; then
        log_error "${app_name}: メニュークリック後もウィンドウの位置が変化しませんでした"
    else
        log_success "${app_name}: メニュー操作で実際にウィンドウが別ディスプレイへ移動しました"
    fi
}

# メイン処理
main() {
    echo ""
    log_info "アプリケーション標準メニューのウィンドウ操作（移動・サイズ変更、ディスプレイ移動）の検証を開始します"
    log_warning "このスクリプトは実際に対象アプリのウィンドウを移動・リサイズします"
    echo ""

    # Finder、Safari、Google Chrome を検証対象とする
    for app_pair in "Finder:Finder" "Safari:Safari" "Google Chrome:Google Chrome"; do
        local app_name="${app_pair%%:*}"
        local process_name="${app_pair##*:}"

        test_window_menu_structure "${app_name}" "${process_name}"
        echo ""
        test_move_and_resize "${app_name}" "${process_name}"
        echo ""
        test_move_to_display "${app_name}" "${process_name}"
        echo ""
    done

    log_test "検証結果のまとめ"
    cat << 'SUMMARY'

## 「ウインドウ」メニュー操作の検証結果

詳細は technical-verification/README.md を参照。

SUMMARY
}

main
