#!/bin/bash

# 引数で指定したアプリケーションを外部ディスプレイの中央に表示させるスクリプト

if [ $# -eq 0 ]; then
    echo "使用方法: $0 <アプリケーション名>"
    echo "例: $0 \"Google Chrome\""
    exit 1
fi

APP_NAME="$1"

echo "アプリケーション '$APP_NAME' を外部ディスプレイの中央に移動します..."

# アプリケーションの起動状態を確認
IS_RUNNING=$(osascript -e "tell application \"System Events\" to exists (process \"$APP_NAME\")")

if [ "$IS_RUNNING" = "false" ]; then
    echo "アプリケーション '$APP_NAME' が起動していません。起動します..."
    osascript -e "tell application \"$APP_NAME\" to activate"
    
    # アプリケーションの起動とウィンドウ表示を待機
    echo "アプリケーションの起動を待機中..."
    sleep 3
    
    # ウィンドウが表示されるまで最大10秒待機
    for i in {1..10}; do
        WINDOW_COUNT=$(osascript -e "tell application \"System Events\" to try to get count of windows of process \"$APP_NAME\"" 2>/dev/null || echo "0")
        echo "現在のウィンドウ数: $WINDOW_COUNT"
        if [ "$WINDOW_COUNT" -gt 0 ]; then
            echo "ウィンドウが表示されました"
            break
        fi
        echo "ウィンドウ表示待機中... ($i/10)"
        sleep 1
    done
    
    if [ "$WINDOW_COUNT" -eq 0 ]; then
        echo "警告: ウィンドウが表示されませんでした。位置変更を試行します..."
    fi
else
    echo "アプリケーション '$APP_NAME' は既に起動しています"
fi

# 画面境界情報を取得
BOUNDS=$(osascript -e 'tell application "Finder" to return bounds of window of desktop')

# 境界情報を解析
LEFT_X=$(echo $BOUNDS | cut -d',' -f1 | tr -d ' ')
TOP_Y=$(echo $BOUNDS | cut -d',' -f2 | tr -d ' ')
RIGHT_X=$(echo $BOUNDS | cut -d',' -f3 | tr -d ' ')
BOTTOM_Y=$(echo $BOUNDS | cut -d',' -f4 | tr -d ' ')

# 外部ディスプレイの座標とサイズを計算
# 外部ディスプレイ: 左上が (-560, -1440), サイズ 2560x1440
EXT_LEFT=-560
EXT_TOP=-1440
EXT_WIDTH=2560
EXT_HEIGHT=1440

# デフォルトウィンドウサイズ
WINDOW_WIDTH=1200
WINDOW_HEIGHT=800

# 外部ディスプレイの中央座標を計算
CENTER_X=$((EXT_LEFT + (EXT_WIDTH - WINDOW_WIDTH) / 2))
CENTER_Y=$((EXT_TOP + (EXT_HEIGHT - WINDOW_HEIGHT) / 2))

echo "外部ディスプレイ領域: ${EXT_LEFT}, ${EXT_TOP}, ${EXT_WIDTH}x${EXT_HEIGHT}"
echo "ウィンドウを移動する座標: ${CENTER_X}, ${CENTER_Y}"
echo "ウィンドウサイズ: ${WINDOW_WIDTH}x${WINDOW_HEIGHT}"

# AppleScriptでウィンドウを移動
osascript << EOF
tell application "System Events"
    try
        tell process "$APP_NAME"
            tell first window
                set position to {$CENTER_X, $CENTER_Y}
                set size to {$WINDOW_WIDTH, $WINDOW_HEIGHT}
            end tell
        end tell
        return "成功: '$APP_NAME' のウィンドウを外部ディスプレイの中央に移動しました"
    on error errMsg
        return "エラー: " & errMsg
    end try
end tell
EOF
