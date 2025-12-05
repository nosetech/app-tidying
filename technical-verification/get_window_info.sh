#!/bin/bash

# Macで実行中のアプリケーションのウィンドウ情報を取得するスクリプト
# システム設定ファイルからディスプレイ配置情報を取得して正確に判定

# システム設定ファイルからディスプレイ情報を取得する関数
get_system_display_config() {
    local plist_file=""
    # ByHost ディレクトリから windowserver.displays plist を探す
    plist_file=$(find ~/Library/Preferences/ByHost -name "com.apple.windowserver.displays.*.plist" 2>/dev/null | head -1)

    if [[ -z "$plist_file" ]]; then
        return 1
    fi

    # Python を使用してディスプレイ設定をパース
    python3 << PYTHON
import plistlib
import sys

with open("$plist_file", 'rb') as f:
    plist = plistlib.load(f)

# DisplaySets の最初の設定（ConfigVersion=1）を取得
if 'DisplaySets' in plist and 'Configs' in plist['DisplaySets']:
    configs = plist['DisplaySets']['Configs']
    for config in configs:
        if config.get('ConfigVersion') == 1:
            # debug: ConfigVersion=1が見つかったことを出力
            # print("DEBUG: Found ConfigVersion=1", file=sys.stderr)
            for display_config in config.get('DisplayConfig', []):
                current_info = display_config.get('CurrentInfo', {})
                uuid = display_config.get('UUID', '')
                origin_x = int(current_info.get('OriginX', 0))
                origin_y = int(current_info.get('OriginY', 0))
                wide = int(current_info.get('Wide', 0))
                high = int(current_info.get('High', 0))

                # debug: 各ディスプレイの情報
                # print(f"DEBUG: UUID={uuid}, OriginX={origin_x}, OriginY={origin_y}, Wide={wide}, High={high}", file=sys.stderr)
                print(f"{uuid}|{origin_x}|{origin_y}|{wide}|{high}")
            break
PYTHON
}

# JXAでディスプレイ名とUUIDの対応を取得
get_display_name_mapping() {
    osascript -l JavaScript << 'JXASCRIPT'
ObjC.import('AppKit')
const screens = $.NSScreen.screens
const result = []

for (let i = 0; i < screens.count; i++) {
    const screen = screens.objectAtIndex(i)
    const displayName = ObjC.unwrap(screen.localizedName) || "不明"

    result.push(displayName)
}

result.join('\n')
JXASCRIPT
}

# システム設定からディスプレイ設定を取得
SYSTEM_DISPLAYS=$(get_system_display_config)

# JXAでディスプレイ名を取得
DISPLAY_NAMES=$(get_display_name_mapping)

# ディスプレイ情報をパースして AppleScript 用の配列を生成
DISPLAY_LIST=""
DISPLAY_NAME_ARRAY=()

# ディスプレイ名を配列に格納
while IFS= read -r name; do
    DISPLAY_NAME_ARRAY+=("$name")
done <<< "$DISPLAY_NAMES"

# システム設定から取得したディスプレイ配置を処理
display_index=0
while IFS='|' read -r uuid origin_x origin_y wide high; do
    if [[ -n "$uuid" ]]; then
        # ディスプレイ名を取得（インデックスに対応）
        display_name="${DISPLAY_NAME_ARRAY[$display_index]:-不明}"

        # 境界座標を計算
        left=$origin_x
        top=$origin_y
        right=$((origin_x + wide))
        bottom=$((origin_y + high))

        # AppleScript用の配列形式を生成
        DISPLAY_LIST+="{$left, $top, $right, $bottom, \"$display_name\"}, "

        ((display_index++))
    fi
done <<< "$SYSTEM_DISPLAYS"

# 最後のカンマとスペースを削除
DISPLAY_LIST=${DISPLAY_LIST%, }

# AppleScriptテンプレートを生成して実行
osascript << APPLESCRIPT
-- Macで実行中のアプリケーションのウィンドウ情報を取得するスクリプト
-- ディスプレイ名を含めて出力

on findDisplayName(winX, winY, winW, winH, displaysInfo)
    set centerX to winX + (winW / 2)
    set centerY to winY + (winH / 2)

    -- debug: ウィンドウ中心座標
    -- log "DEBUG: Window center: (" & centerX & ", " & centerY & ")"

    -- ウィンドウの中心がディスプレイ内に含まれるか確認
    repeat with i from 1 to (count of displaysInfo)
        set displayInfo to item i of displaysInfo
        set displayLeft to item 1 of displayInfo
        set displayTop to item 2 of displayInfo
        set displayRight to item 3 of displayInfo
        set displayBottom to item 4 of displayInfo
        set displayName to item 5 of displayInfo

        -- debug: 各ディスプレイの境界
        -- log "DEBUG: Display " & i & " (" & displayName & "): L=" & displayLeft & " T=" & displayTop & " R=" & displayRight & " B=" & displayBottom

        if centerX ≥ displayLeft and centerX ≤ displayRight and centerY ≥ displayTop and centerY ≤ displayBottom then
            -- log "DEBUG: Matched center in display " & displayName
            return displayName
        end if
    end repeat

    -- ウィンドウが複数のディスプレイと重なっている場合、最大の重なり面積を持つディスプレイを返す
    set maxOverlapArea to 0
    set displayNameWithMaxOverlap to ""

    set winRight to winX + winW
    set winBottom to winY + winH

    repeat with i from 1 to (count of displaysInfo)
        set displayInfo to item i of displaysInfo
        set displayLeft to item 1 of displayInfo
        set displayTop to item 2 of displayInfo
        set displayRight to item 3 of displayInfo
        set displayBottom to item 4 of displayInfo
        set displayName to item 5 of displayInfo

        -- AABB衝突判定（軸並行境界ボックス）
        if winX < displayRight and winRight > displayLeft and winY < displayBottom and winBottom > displayTop then
            -- 重なり領域を計算
            if displayLeft > winX then
                set overlapLeft to displayLeft
            else
                set overlapLeft to winX
            end if

            if displayTop > winY then
                set overlapTop to displayTop
            else
                set overlapTop to winY
            end if

            if displayRight < winRight then
                set overlapRight to displayRight
            else
                set overlapRight to winRight
            end if

            if displayBottom < winBottom then
                set overlapBottom to displayBottom
            else
                set overlapBottom to winBottom
            end if

            set overlapWidth to overlapRight - overlapLeft
            set overlapHeight to overlapBottom - overlapTop

            if overlapWidth > 0 and overlapHeight > 0 then
                set overlapArea to overlapWidth * overlapHeight

                if overlapArea > maxOverlapArea then
                    set maxOverlapArea to overlapArea
                    set displayNameWithMaxOverlap to displayName
                end if
            end if
        end if
    end repeat

    if displayNameWithMaxOverlap ≠ "" then
        return displayNameWithMaxOverlap
    end if

    -- どのディスプレイとも重ならない場合は、最も近いディスプレイを返す
    set closestDisplayIndex to 1
    set closestDistance to 999999999

    repeat with i from 1 to (count of displaysInfo)
        set displayInfo to item i of displaysInfo
        set displayLeft to item 1 of displayInfo
        set displayTop to item 2 of displayInfo
        set displayRight to item 3 of displayInfo
        set displayBottom to item 4 of displayInfo
        set displayName to item 5 of displayInfo

        set distanceX to 0
        set distanceY to 0

        if centerX < displayLeft then
            set distanceX to displayLeft - centerX
        else if centerX > displayRight then
            set distanceX to centerX - displayRight
        end if

        if centerY < displayTop then
            set distanceY to displayTop - centerY
        else if centerY > displayBottom then
            set distanceY to centerY - displayBottom
        end if

        set distance to distanceX * distanceX + distanceY * distanceY

        if distance < closestDistance then
            set closestDistance to distance
            set closestDisplayIndex to i
        end if
    end repeat

    set closestDisplay to item closestDisplayIndex of displaysInfo
    return item 5 of closestDisplay
end findDisplayName

set displaysInfo to {$DISPLAY_LIST}

tell application "System Events"
    set appList to {}
    repeat with proc in (processes whose background only is false)
        try
            set procName to name of proc
            set windowList to windows of proc
            if (count of windowList) > 0 then
                repeat with win in windowList
                    try
                        set winPos to position of win
                        set winSize to size of win
                        set winTitle to title of win
                        set winX to item 1 of winPos
                        set winY to item 2 of winPos
                        set winW to item 1 of winSize
                        set winH to item 2 of winSize

                        -- ウィンドウのディスプレイを判定
                        set displayName to my findDisplayName(winX, winY, winW, winH, displaysInfo)

                        set end of appList to procName & " | " & winTitle & " | " & winX & "," & winY & " | " & winW & "x" & winH & " | " & displayName
                    end try
                end repeat
            end if
        end try
    end repeat
end tell

return appList
APPLESCRIPT


