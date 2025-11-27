#!/bin/bash

# macOS ステージマネージャ詳細検証スクリプト
# ステージマネージャの状態と、その有効/無効がウィンドウ操作に与える影響を検証

# ステージマネージャが有効か確認する関数
is_stage_manager_enabled() {
    local enabled=$(defaults read com.apple.WindowManager GloballyEnabled 2>/dev/null)
    if [[ "$enabled" == "1" ]]; then
        echo "true"
    else
        echo "false"
    fi
}

# ステージマネージャの有効状態を表示
echo "========================================="
echo "Stage Manager Status Check"
echo "========================================="
echo ""

# macOS バージョンを確認
os_version=$(sw_vers -productVersion)
echo "macOS Version: $os_version"
echo ""

# ステージマネージャが有効か確認
stage_manager_enabled=$(is_stage_manager_enabled)
echo "Stage Manager Enabled: $stage_manager_enabled"

# com.apple.WindowManager の全キーを表示
echo ""
echo "com.apple.WindowManager Settings:"
defaults read com.apple.WindowManager 2>/dev/null

echo ""
echo "========================================="
echo "Stage Manager Window Behavior Test"
echo "========================================="
echo ""

# ウィンドウ取得テスト（ステージマネージャ有効時の動作検証）
osascript << 'APPLESCRIPT'
tell application "System Events"
    set windowInfoList to {}

    repeat with proc in processes whose background only is false
        try
            set procName to name of proc

            -- ウィンドウを取得
            set windowList to windows of proc
            if (count of windowList) > 0 then
                repeat with win in windowList
                    try
                        set winTitle to title of win
                        set winPos to position of win
                        set winSize to size of win
                        set winX to item 1 of winPos
                        set winY to item 2 of winPos
                        set winW to item 1 of winSize
                        set winH to item 2 of winSize

                        -- ウィンドウ情報を記録
                        set windowInfo to procName & " | " & winTitle & " | Position: (" & winX & ", " & winY & ") | Size: " & winW & "x" & winH
                        set end of windowInfoList to windowInfo
                    on error errMsg
                        log "Error getting window info: " & errMsg
                    end try
                end repeat
            end if
        on error
            -- Skip
        end try
    end repeat

    -- ウィンドウ情報を出力
    repeat with info in windowInfoList
        log info
    end repeat

    log "Total windows found: " & (count of windowInfoList)
end tell
APPLESCRIPT

echo ""
echo "========================================="
echo "Stage Manager Group Information"
echo "========================================="
echo ""

# Mission Control の設定（仮想デスクトップとステージマネージャのグループ化）
osascript << 'APPLESCRIPT'
tell application "System Events"
    -- スペース情報を取得（仮想デスクトップ）
    try
        -- Dock.app から Mission Control の状態を確認
        set missionControlActive to exists (process "Dock")
        log "Mission Control check: " & missionControlActive

        -- ステージマネージャのグループ管理は AppleScript では直接アクセス不可
        -- グループ化されたウィンドウの情報は Window Server の内部データ構造に存在
        log "Note: Stage Manager groups are managed by WindowServer and not directly accessible via AppleScript"

    on error errMsg
        log "Error: " & errMsg
    end try
end tell
APPLESCRIPT

echo ""
echo "========================================="
echo "Potential Limitations with Stage Manager"
echo "========================================="
echo ""

cat << 'EOF'
When Stage Manager is enabled:

1. Window Access Limitations:
   - Only windows in the active group are fully accessible via AppleScript
   - Background group windows may return partial or incorrect position/size data
   - Some window operations may fail on non-active group windows

2. Window Position/Size Impact:
   - Window positions may be relative to the group rather than screen coordinates
   - Window sizes may not reflect the actual visual size due to grouping

3. Multi-App Groups:
   - Multiple applications can be grouped together in a single Stage Manager group
   - Grouped windows move together as a unit
   - Ungrouping operations may not be supported via AppleScript

4. Implementation Recommendations:
   - Check Stage Manager status before performing window operations
   - Consider adding a flag to disable app when Stage Manager is enabled
   - Document that window layout restoration may not work properly with Stage Manager
   - Offer user warning when attempting to manage windows with Stage Manager active

5. workarounds:
   - Toggle Stage Manager off temporarily for window management
   - Use AppleScript to activate the target application first
   - Implement retry logic if window operations fail
EOF

echo ""
echo "========================================="
echo "Verification Complete"
echo "========================================="
