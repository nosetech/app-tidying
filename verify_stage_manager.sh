#!/bin/bash

# macOS ステージマネージャ状態の検証スクリプト
# macOS 13 Ventura 以降で導入されたステージマネージャの状態取得を検証

# System Information を取得する関数
get_system_info() {
    echo "=== macOS System Information ==="
    system_profiler SPSoftwareDataType 2>/dev/null | grep -E "System Version|Kernel Version" || echo "Not available"
    echo ""
}

# ステージマネージャの状態を取得する関数（AppleScript）
check_stage_manager_applescript() {
    echo "=== Stage Manager Status (AppleScript) ==="

    osascript << 'APPLESCRIPT'
tell application "System Events"
    try
        -- Mission Control ペイン内のステージマネージャの設定を確認
        -- ただし、直接的に取得できるプロパティは限定的
        set stageManagerEnabled to "Unknown"

        -- サポートされているか確認（macOS 13以降）
        -- ProcessSerializationのようなプロパティでステージマネージャの有効化確認を試みる
        tell (first process whose name is "Finder")
            try
                -- 新しいウィンドウプロパティをチェック
                -- アクティブなウィンドウを取得
                set activeWindow to (front window)
                set windowCount to (count of windows)
                log "Active window count: " & windowCount
            end try
        end tell

        return stageManagerEnabled
    on error errMsg
        return "Error: " & errMsg
    end try
end tell
APPLESCRIPT
    echo ""
}

# defaults コマンドを使ってステージマネージャの設定を確認
check_stage_manager_defaults() {
    echo "=== Stage Manager Status (defaults read) ==="

    # Desktop ドメインでステージマネージャ関連のキーを検索
    echo "Checking com.apple.WindowManager..."
    defaults read com.apple.WindowManager 2>/dev/null | grep -i "stage" || echo "No stage-related keys found"

    echo ""
    echo "Checking com.apple.dock for stage manager settings..."
    defaults read com.apple.dock 2>/dev/null | grep -i -E "stage|expose" || echo "No stage-related keys found"

    echo ""
}

# ステージマネージャ関連の設定キーをすべて探索
search_stage_manager_settings() {
    echo "=== Searching for Stage Manager Settings ==="

    # ホームディレクトリの全てのプリファレンスを探索
    echo "Searching in ~/Library/Preferences/..."

    local found_keys=0

    # defaults を使って全ドメインを列挙（slow）
    # 代わりに既知のドメインをチェック
    for domain in com.apple.dock com.apple.WindowManager com.apple.Expose com.apple.spaces; do
        echo ""
        echo "Domain: $domain"
        if defaults read "$domain" 2>/dev/null | head -20; then
            found_keys=$((found_keys + 1))
        else
            echo "  (not found)"
        fi
    done

    echo ""
}

# System Preferences から直接読み込み
check_system_settings() {
    echo "=== Stage Manager from System Settings ==="

    # macOS 13+ のシステム設定で Desktop & Dock セクションをチェック
    defaults read com.apple.desktop 2>/dev/null | grep -i stage || echo "No Desktop setting found"

    echo ""
}

# ウィンドウ動作の検証
verify_window_behavior() {
    echo "=== Window Behavior Verification ==="

    # ステージマネージャが有効な場合とそうでない場合のウィンドウ取得の違いを検証
    osascript << 'APPLESCRIPT'
tell application "System Events"
    set processCount to count of processes
    log "Total processes: " & processCount

    set visibleProcesses to 0
    set hiddenProcesses to 0

    repeat with proc in processes
        try
            if background only of proc then
                set hiddenProcesses to hiddenProcesses + 1
            else
                set visibleProcesses to visibleProcesses + 1
                set procName to name of proc
                set windowCount to count of windows of proc
                if windowCount > 0 then
                    log procName & ": " & windowCount & " windows"
                end if
            end if
        end try
    end repeat

    log "Visible processes: " & visibleProcesses
    log "Background processes: " & hiddenProcesses
end tell
APPLESCRIPT

    echo ""
}

# JXA (JavaScript for Automation) を使用した確認
check_with_jxa() {
    echo "=== Stage Manager Status (JXA) ==="

    osascript -l JavaScript << 'JXASCRIPT'
ObjC.import('AppKit')
ObjC.import('Foundation')

const fm = $.NSFileManager.defaultManager
const userDefaults = $.NSUserDefaults.standardUserDefaults

// Stage Manager 関連のキーを探索
const stageManagerKeys = [
    'com.apple.WindowManager',
    'com.apple.desktop',
    'com.apple.Expose',
    'com.apple.spaces',
    'AppleWindowTabbingMode'
]

try {
    // NSScreen 情報から仮想デスクトップの情報を取得
    const screens = $.NSScreen.screens
    console.log("Number of screens: " + screens.count)

    // Desktop & Dock プリファレンスを読み込む
    const plistPath = $.NSHomeDirectory() + "/Library/Preferences/com.apple.WindowManager.plist"
    const plistExists = fm.fileExistsAtPath(plistPath)
    console.log("WindowManager plist exists: " + plistExists)

    if (plistExists) {
        const plist = $.NSMutableDictionary.dictionaryWithContentsOfFile(plistPath)
        if (plist) {
            const keys = plist.allKeys
            for (let i = 0; i < keys.count; i++) {
                const key = $.NSString(keys.objectAtIndex(i))
                if (ObjC.unwrap(key).toLowerCase().includes('stage')) {
                    console.log("Found Stage Manager key: " + key + " = " + plist.objectForKey(key))
                }
            }
        }
    }

} catch (e) {
    console.log("Error: " + e)
}
JXASCRIPT

    echo ""
}

# ステージマネージャが有効な場合のウィンドウへの影響を検証
verify_stage_manager_impact() {
    echo "=== Potential Stage Manager Impact on Windows ==="

    echo "When Stage Manager is enabled:"
    echo "1. Only windows in the active group are fully visible"
    echo "2. Window positions may be affected by group management"
    echo "3. Multiple windows may be managed as groups"
    echo "4. AppleScript window position/size queries may return different results"
    echo ""

    echo "Testing window accessibility..."
    osascript << 'APPLESCRIPT'
tell application "System Events"
    set appCount to 0
    set windowCount to 0
    set inaccessibleCount to 0

    repeat with proc in processes whose background only is false
        try
            set procName to name of proc
            set windowList to windows of proc
            set appCount to appCount + 1

            repeat with win in windowList
                set windowCount to windowCount + 1
                try
                    set winPos to position of win
                    set winSize to size of win
                on error
                    set inaccessibleCount to inaccessibleCount + 1
                end try
            end repeat
        on error
            -- Skip processes that don't have windows
        end try
    end repeat

    log "Applications with windows: " & appCount
    log "Total windows accessible: " & windowCount
    log "Windows with inaccessible position/size: " & inaccessibleCount
end tell
APPLESCRIPT

    echo ""
}

# メイン処理
main() {
    echo "========================================="
    echo "Stage Manager Verification Script"
    echo "========================================="
    echo ""

    get_system_info
    check_stage_manager_applescript
    check_stage_manager_defaults
    check_system_settings
    check_with_jxa
    verify_window_behavior
    verify_stage_manager_impact
    search_stage_manager_settings

    echo "========================================="
    echo "Verification Complete"
    echo "========================================="
}

main
