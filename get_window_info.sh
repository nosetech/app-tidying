#!/bin/bash

# Macで実行中のアプリケーションのウィンドウ情報を取得するスクリプト

# スクリプトのディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# メインディスプレイの境界をJXAで動的に取得
MAIN_DISPLAY_BOUNDS=$(osascript -l JavaScript << 'JXASCRIPT'
ObjC.import('AppKit')
const mainScreen = $.NSScreen.mainScreen
const frame = mainScreen.frame
const bounds = [
  Math.round(frame.origin.x),
  Math.round(frame.origin.y),
  Math.round(frame.origin.x + frame.size.width),
  Math.round(frame.origin.y + frame.size.height)
]
bounds.join(',')
JXASCRIPT
)

# AppleScriptで使用するメインディスプレイ境界を生成
IFS=',' read -r LEFT TOP RIGHT BOTTOM <<< "$MAIN_DISPLAY_BOUNDS"

# AppleScriptテンプレートを生成して実行
osascript << APPLESCRIPT
-- Macで実行中のアプリケーションのウィンドウ情報を取得するスクリプト
-- ディスプレイ情報も含めて出力

-- メインディスプレイの境界定数
on getMainDisplayBounds()
    return {$LEFT, $TOP, $RIGHT, $BOTTOM}
end getMainDisplayBounds

-- ウィンドウがメインディスプレイに属するかを判定する関数
on isWindowOnMainDisplay(winX, winY, winW, winH, mainBounds)
    set centerX to winX + (winW / 2)
    set centerY to winY + (winH / 2)

    set mainLeft to item 1 of mainBounds
    set mainTop to item 2 of mainBounds
    set mainRight to item 3 of mainBounds
    set mainBottom to item 4 of mainBounds

    if centerX ≥ mainLeft and centerX ≤ mainRight and centerY ≥ mainTop and centerY ≤ mainBottom then
        return true
    else
        return false
    end if
end isWindowOnMainDisplay

tell application "System Events"
    set mainBounds to my getMainDisplayBounds()

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
                        if my isWindowOnMainDisplay(winX, winY, winW, winH, mainBounds) then
                            set displayInfo to "Main"
                        else
                            set displayInfo to "External"
                        end if

                        set end of appList to procName & " | " & winTitle & " | " & winX & "," & winY & " | " & winW & "x" & winH & " | " & displayInfo
                    end try
                end repeat
            end if
        end try
    end repeat
    return appList
end tell
APPLESCRIPT
