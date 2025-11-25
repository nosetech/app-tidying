#!/bin/bash

# ディスプレイの解像度情報を取得するスクリプト

echo "=== ディスプレイ情報 ==="

# JXAを使用してディスプレイ情報を取得
# ディスプレイ名、物理解像度、UI解像度（論理解像度）、スケーリング係数を取得
osascript -l JavaScript << 'JXASCRIPT'
ObjC.import('AppKit')

const screens = $.NSScreen.screens
const result = []

for (let i = 0; i < screens.count; i++) {
    const screen = screens.objectAtIndex(i)
    const frame = screen.frame
    const scaleFactor = screen.backingScaleFactor

    // ディスプレイ名を取得（NSScreen.localizedNameを使用）
    const displayName = ObjC.unwrap(screen.localizedName) || "不明"

    // 物理解像度
    const physicalWidth = Math.round(frame.size.width)
    const physicalHeight = Math.round(frame.size.height)

    // UI解像度（論理解像度）
    const uiWidth = Math.round(physicalWidth / scaleFactor)
    const uiHeight = Math.round(physicalHeight / scaleFactor)

    // メインディスプレイの判定
    const isMainDisplay = screen.isEqual($.NSScreen.mainScreen)
    const displayType = isMainDisplay ? "（メイン）" : ""

    result.push(`ディスプレイ ${i}: ${displayName} ${displayType}`)
    result.push(`  物理解像度: ${physicalWidth}x${physicalHeight}`)
    result.push(`  UI解像度: ${uiWidth}x${uiHeight}`)
    result.push(`  スケール係数: ${scaleFactor}x`)
}

result.join('\n')
JXASCRIPT

echo ""
echo "=== 画面境界情報 ==="

# AppleScriptを使って画面境界を取得
osascript -e '
tell application "Finder"
    set screenBounds to bounds of window of desktop
    set leftX to item 1 of screenBounds
    set topY to item 2 of screenBounds
    set rightX to item 3 of screenBounds
    set bottomY to item 4 of screenBounds
    return "全体の境界: " & leftX & ", " & topY & ", " & rightX & ", " & bottomY
end tell'