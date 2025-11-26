#!/bin/bash

# Macで実行中のアプリケーションのウィンドウ情報を取得するスクリプト
# ウィンドウのあるディスプレイ名を表示

# すべてのディスプレイ情報をJXAで取得
# AppleScript内で直接使用するために、複数行の出力形式にする
DISPLAYS_INFO=$(osascript -l JavaScript << 'JXASCRIPT'
ObjC.import('AppKit')
const screens = $.NSScreen.screens
const result = []

for (let i = 0; i < screens.count; i++) {
    const screen = screens.objectAtIndex(i)
    const frame = screen.frame
    const displayName = ObjC.unwrap(screen.localizedName) || "不明"
    const bounds = {
        left: Math.round(frame.origin.x),
        top: Math.round(frame.origin.y),
        right: Math.round(frame.origin.x + frame.size.width),
        bottom: Math.round(frame.origin.y + frame.size.height)
    }

    result.push(`${bounds.left},${bounds.top},${bounds.right},${bounds.bottom}|${displayName}`)
}

result.join('\n')
JXASCRIPT
)

# Bash側でディスプレイ情報をパース（複数行で処理）
DISPLAY_LIST=""
while IFS='|' read -r bounds display_name; do
    if [[ -n "$bounds" && -n "$display_name" ]]; then
        IFS=',' read -r left top right bottom <<< "$bounds"
        # AppleScript用の配列形式を生成
        DISPLAY_LIST+="{$left, $top, $right, $bottom, \"$display_name\"}, "
    fi
done <<< "$DISPLAYS_INFO"

# 最後のカンマとスペースを削除
DISPLAY_LIST=${DISPLAY_LIST%, }

# AppleScriptテンプレートを生成して実行
osascript << APPLESCRIPT
-- Macで実行中のアプリケーションのウィンドウ情報を取得するスクリプト
-- ディスプレイ名を含めて出力

on findDisplayName(winX, winY, winW, winH, displaysInfo)
    set centerX to winX + (winW / 2)
    set centerY to winY + (winH / 2)

    -- ウィンドウの中心がディスプレイ内に含まれるか確認
    repeat with i from 1 to (count of displaysInfo)
        set displayInfo to item i of displaysInfo
        set displayLeft to item 1 of displayInfo
        set displayTop to item 2 of displayInfo
        set displayRight to item 3 of displayInfo
        set displayBottom to item 4 of displayInfo
        set displayName to item 5 of displayInfo

        if centerX ≥ displayLeft and centerX ≤ displayRight and centerY ≥ displayTop and centerY ≤ displayBottom then
            return displayName
        end if
    end repeat

    -- ウィンドウの境界がディスプレイと重なっているか確認
    repeat with i from 1 to (count of displaysInfo)
        set displayInfo to item i of displaysInfo
        set displayLeft to item 1 of displayInfo
        set displayTop to item 2 of displayInfo
        set displayRight to item 3 of displayInfo
        set displayBottom to item 4 of displayInfo
        set displayName to item 5 of displayInfo

        set winRight to winX + winW
        set winBottom to winY + winH

        -- AABB衝突判定（軸並行境界ボックス）
        if winX ≤ displayRight and winRight ≥ displayLeft and winY ≤ displayBottom and winBottom ≥ displayTop then
            return displayName
        end if
    end repeat

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


