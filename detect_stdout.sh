#!/bin/bash

# 標準出力が利用可能かどうかを判定するスクリプト
# ターミナルで実行時は標準出力に出力、Automatorなどで実行時はダイアログで表示

# 標準出力が利用可能かを判定する関数
is_stdout_available() {
    # TTYが接続されている場合は標準出力が利用可能
    if [ -t 1 ]; then
        return 0  # 標準出力が利用可能
    fi

    # $TERM が設定されていれば、ターミナル環境
    if [ -n "$TERM" ] && [ "$TERM" != "dumb" ]; then
        return 0  # 標準出力が利用可能
    fi

    # その他の場合は標準出力が利用できない
    return 1  # 標準出力が利用不可
}

# メッセージを出力する関数
output_message() {
    local message="$1"

    if is_stdout_available; then
        # 標準出力が利用可能な場合
        echo "$message"
    else
        # 標準出力が利用できない場合、ダイアログで表示
        osascript << EOF
tell application "System Events"
    display dialog "$message" with title "アプリケーション情報" buttons {"OK"} default button 1
end tell
EOF
    fi
}

# テスト実行
echo "=== 標準出力判定テスト ===" >&2

if is_stdout_available; then
    echo "判定結果: 標準出力が利用可能です"
    echo "実行環境: ターミナル/シェル"
else
    echo "判定結果: 標準出力が利用できません" >&2
    echo "実行環境: Automator/GUI アプリケーション" >&2
fi

# メッセージ出力のテスト
output_message "このメッセージは標準出力またはダイアログで表示されます"
