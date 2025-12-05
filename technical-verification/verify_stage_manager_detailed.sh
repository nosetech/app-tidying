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
echo "ステージマネージャの状態確認"
echo "========================================="
echo ""

# macOS バージョンを確認
os_version=$(sw_vers -productVersion)
echo "macOS バージョン: $os_version"
echo ""

# ステージマネージャが有効か確認
stage_manager_enabled=$(is_stage_manager_enabled)
echo "ステージマネージャ有効: $stage_manager_enabled"

# com.apple.WindowManager の全キーを表示
echo ""
echo "com.apple.WindowManager 設定:"
defaults read com.apple.WindowManager 2>/dev/null

echo ""
echo "========================================="
echo "ステージマネージャのウィンドウ動作テスト"
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
                        set windowInfo to procName & " | " & winTitle & " | 位置: (" & winX & ", " & winY & ") | サイズ: " & winW & "x" & winH
                        set end of windowInfoList to windowInfo
                    on error errMsg
                        log "ウィンドウ情報の取得エラー: " & errMsg
                    end try
                end repeat
            end if
        on error
            -- スキップ
        end try
    end repeat

    -- ウィンドウ情報を出力
    repeat with info in windowInfoList
        log info
    end repeat

    log "見つかったウィンドウの合計: " & (count of windowInfoList)
end tell
APPLESCRIPT

echo ""
echo "========================================="
echo "ステージマネージャグループ情報"
echo "========================================="
echo ""

# Mission Control の設定（仮想デスクトップとステージマネージャのグループ化）
osascript << 'APPLESCRIPT'
tell application "System Events"
    -- スペース情報を取得（仮想デスクトップ）
    try
        -- Dock.app から Mission Control の状態を確認
        set missionControlActive to exists (process "Dock")
        log "Mission Control の確認: " & missionControlActive

        -- ステージマネージャのグループ管理は AppleScript では直接アクセス不可
        -- グループ化されたウィンドウの情報は Window Server の内部データ構造に存在
        log "注: ステージマネージャのグループは WindowServer によって管理されており、AppleScript では直接アクセスできません"

    on error errMsg
        log "エラー: " & errMsg
    end try
end tell
APPLESCRIPT

echo ""
echo "========================================="
echo "ステージマネージャの制限事項"
echo "========================================="
echo ""

cat << 'EOF'
ステージマネージャが有効な場合:

1. ウィンドウアクセスの制限:
   - アクティブなグループ内のウィンドウのみが AppleScript で完全にアクセス可能
   - バックグラウンドグループのウィンドウは不完全または不正な位置・サイズデータを返す可能性
   - 非アクティブなグループのウィンドウでは一部の操作が失敗する可能性

2. ウィンドウの位置・サイズへの影響:
   - ウィンドウの位置がグループに相対的であり、画面座標では表現されない可能性
   - グループ化によりウィンドウサイズが実際の表示サイズを反映していない可能性

3. 複数アプリのグループ化:
   - 複数のアプリケーションが1つのステージマネージャグループにグループ化される可能性
   - グループ化されたウィンドウは1つのユニットとして移動
   - AppleScript ではグループ化解除操作はサポートされない可能性

4. 実装時の推奨事項:
   - ウィンドウ操作前にステージマネージャの状態をチェック
   - ステージマネージャが有効な場合はアプリを無効化するフラグの追加を検討
   - ステージマネージャ有効時のウィンドウレイアウト復元は正常に機能しないことを文書化
   - ステージマネージャがアクティブな場合、ウィンドウ管理時にユーザーに警告を表示

5. 回避策:
   - ウィンドウ管理の際、ステージマネージャを一時的に無効化
   - AppleScript を使用してターゲットアプリケーションを事前にアクティベート
   - ウィンドウ操作失敗時のリトライロジックを実装
EOF

echo ""
echo "========================================="
echo "検証完了"
echo "========================================="
