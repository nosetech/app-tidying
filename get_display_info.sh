#!/bin/bash

# ディスプレイの解像度情報を取得するスクリプト

echo "=== ディスプレイ情報 ==="

# system_profilerを使用してディスプレイ情報を取得し、解像度部分を抽出
system_profiler SPDisplaysDataType | grep -E "(Display Type|Resolution|UI Looks like)" | while read line; do
    if [[ $line == *"Display Type"* ]]; then
        display_name=$(echo "$line" | sed 's/.*Display Type: //')
        echo "ディスプレイ: $display_name"
    elif [[ $line == *"Resolution"* ]]; then
        resolution=$(echo "$line" | sed 's/.*Resolution: //' | sed 's/ (.*//')
        echo "  解像度: $resolution"
    elif [[ $line == *"UI Looks like"* ]]; then
        ui_resolution=$(echo "$line" | sed 's/.*UI Looks like: //' | sed 's/ @.*//')
        echo "  UI解像度: $ui_resolution"
    fi
done

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