#!/bin/bash

# macOS ステージマネージャ状態の検証スクリプト
# macOS 13 Ventura 以降で導入されたステージマネージャの状態取得を検証

# System Information を取得する関数
get_system_info() {
    echo "=== macOS システム情報 ==="
    system_profiler SPSoftwareDataType 2>/dev/null | grep -E "System Version|Kernel Version" || echo "取得できません"
    echo ""
}

# ステージマネージャの状態を取得する関数（AppleScript）
check_stage_manager_applescript() {
    echo "=== ステージマネージャの状態 (AppleScript) ==="

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
    echo "=== ステージマネージャの状態 (defaults read) ==="

    # Desktop ドメインでステージマネージャ関連のキーを検索
    echo "com.apple.WindowManager を確認中..."
    defaults read com.apple.WindowManager 2>/dev/null | grep -i "stage" || echo "ステージマネージャ関連のキーが見つかりません"

    echo ""
    echo "com.apple.dock のステージマネージャ設定を確認中..."
    defaults read com.apple.dock 2>/dev/null | grep -i -E "stage|expose" || echo "ステージマネージャ関連のキーが見つかりません"

    echo ""
}

# ステージマネージャ関連の設定キーをすべて探索
search_stage_manager_settings() {
    echo "=== ステージマネージャ設定の検索 ==="

    # ホームディレクトリの全てのプリファレンスを探索
    echo "~/Library/Preferences/ で検索中..."

    local found_keys=0

    # defaults を使って全ドメインを列挙（slow）
    # 代わりに既知のドメインをチェック
    for domain in com.apple.dock com.apple.WindowManager com.apple.Expose com.apple.spaces; do
        echo ""
        echo "ドメイン: $domain"
        if defaults read "$domain" 2>/dev/null | head -20; then
            found_keys=$((found_keys + 1))
        else
            echo "  (見つかりません)"
        fi
    done

    echo ""
}

# System Preferences から直接読み込み
check_system_settings() {
    echo "=== システム設定からのステージマネージャ ==="

    # macOS 13+ のシステム設定で Desktop & Dock セクションをチェック
    defaults read com.apple.desktop 2>/dev/null | grep -i stage || echo "デスクトップ設定が見つかりません"

    echo ""
}

# ウィンドウ動作の検証
verify_window_behavior() {
    echo "=== ウィンドウの動作検証 ==="

    # ステージマネージャが有効な場合とそうでない場合のウィンドウ取得の違いを検証
    osascript << 'APPLESCRIPT'
tell application "System Events"
    set processCount to count of processes
    log "総プロセス数: " & processCount

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
                    log procName & ": " & windowCount & " ウィンドウ"
                end if
            end if
        end try
    end repeat

    log "表示中のプロセス: " & visibleProcesses
    log "バックグラウンドプロセス: " & hiddenProcesses
end tell
APPLESCRIPT

    echo ""
}

# JXA (JavaScript for Automation) を使用した確認
check_with_jxa() {
    echo "=== ステージマネージャの状態 (JXA) ==="

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
    console.log("スクリーン数: " + screens.count)

    // Desktop & Dock プリファレンスを読み込む
    const plistPath = $.NSHomeDirectory() + "/Library/Preferences/com.apple.WindowManager.plist"
    const plistExists = fm.fileExistsAtPath(plistPath)
    console.log("WindowManager plist が存在: " + plistExists)

    if (plistExists) {
        const plist = $.NSMutableDictionary.dictionaryWithContentsOfFile(plistPath)
        if (plist) {
            const keys = plist.allKeys
            for (let i = 0; i < keys.count; i++) {
                const key = $.NSString(keys.objectAtIndex(i))
                if (ObjC.unwrap(key).toLowerCase().includes('stage')) {
                    console.log("ステージマネージャキーを検出: " + key + " = " + plist.objectForKey(key))
                }
            }
        }
    }

} catch (e) {
    console.log("エラー: " + e)
}
JXASCRIPT

    echo ""
}

# ステージマネージャが有効な場合のウィンドウへの影響を検証
verify_stage_manager_impact() {
    echo "=== ステージマネージャがウィンドウに与える潜在的な影響 ==="

    echo "ステージマネージャが有効な場合:"
    echo "1. アクティブなグループ内のウィンドウのみが完全に表示される"
    echo "2. ウィンドウの位置がグループ管理の影響を受ける可能性がある"
    echo "3. 複数のウィンドウはグループとして管理される可能性がある"
    echo "4. AppleScript でウィンドウの位置・サイズの取得が異なる結果を返す可能性"
    echo ""

    echo "ウィンドウアクセスをテスト中..."
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
            -- ウィンドウのないプロセスはスキップ
        end try
    end repeat

    log "ウィンドウを持つアプリケーション数: " & appCount
    log "アクセス可能なウィンドウの合計: " & windowCount
    log "位置・サイズ取得が不可能なウィンドウ: " & inaccessibleCount
end tell
APPLESCRIPT

    echo ""
}

# メイン処理
main() {
    echo "========================================="
    echo "ステージマネージャ検証スクリプト"
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
    echo "検証完了"
    echo "========================================="
}

main
