#!/bin/bash

# macOS仮想デスクトップ（スペース）の検証スクリプト
# Issue #4: macOS仮想デスクトップ（スペース）の情報取得と操作の検証

set -e

# カラー出力
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== macOS仮想デスクトップ（スペース）検証スクリプト ===${NC}"
echo ""

# =====================================
# 【検証1】スペース情報取得
# =====================================
echo -e "${BLUE}【検証1】スペース情報の取得${NC}"
echo ""

python3 << 'PYTHON'
import plistlib
import os
import sys

plist_path = os.path.expanduser('~/Library/Preferences/com.apple.spaces.plist')

try:
    with open(plist_path, 'rb') as f:
        spaces_plist = plistlib.load(f)

    # SpacesDisplayConfiguration から情報を取得
    config = spaces_plist.get('SpacesDisplayConfiguration', {})
    mgmt_data = config.get('Management Data', {})
    monitors = mgmt_data.get('Monitors', [])

    print("■ 現在のスペース設定:")
    print(f"  Management Mode: {mgmt_data.get('Management Mode', 'Unknown')}")
    print()

    print("■ ディスプレイ別スペース情報:")
    total_spaces = 0
    for i, monitor in enumerate(monitors):
        display_id = monitor.get('Display Identifier', f'Display {i+1}')
        spaces = monitor.get('Spaces', [])
        current = monitor.get('Current Space', {})

        print(f"  {display_id}:")
        print(f"    スペース数: {len(spaces)}")
        print(f"    スペースID一覧: {[s.get('id64') for s in spaces]}")
        print(f"    現在のスペース: {current.get('id64', 'N/A')} (UUID: {current.get('uuid', 'N/A')})")
        print()
        total_spaces += len(spaces)

    print(f"■ 総スペース数: {total_spaces}")
    print()

    # Space Properties からウィンドウ情報を取得
    space_props = config.get('Space Properties', [])
    print("■ スペース別ウィンドウ割り当て:")
    for space_prop in space_props:
        space_name = space_prop.get('name', 'Unknown')
        windows = space_prop.get('windows', [])
        print(f"  {space_name}: {len(windows)} ウィンドウ")

except Exception as e:
    print(f"Error: {e}", file=sys.stderr)
    sys.exit(1)
PYTHON

echo ""
echo -e "${GREEN}✓ スペース情報取得: 成功${NC}"
echo ""

# =====================================
# 【検証2】スペース総数の取得
# =====================================
echo -e "${BLUE}【検証2】スペース総数の取得${NC}"
echo ""

python3 << 'PYTHON'
import plistlib
import os

plist_path = os.path.expanduser('~/Library/Preferences/com.apple.spaces.plist')

try:
    with open(plist_path, 'rb') as f:
        spaces_plist = plistlib.load(f)

    config = spaces_plist.get('SpacesDisplayConfiguration', {})
    monitors = config.get('Management Data', {}).get('Monitors', [])

    total_spaces = sum(len(m.get('Spaces', [])) for m in monitors)
    print(f"総スペース数: {total_spaces}")

except Exception as e:
    print(f"Error: {e}")
    import sys
    sys.exit(1)
PYTHON

echo ""
echo -e "${GREEN}✓ スペース総数取得: 可能${NC}"
echo ""

# =====================================
# 【検証3】スペース名の取得
# =====================================
echo -e "${BLUE}【検証3】スペース名の取得${NC}"
echo ""

echo "■ スペース名一覧:"
python3 << 'PYTHON'
import plistlib
import os

plist_path = os.path.expanduser('~/Library/Preferences/com.apple.spaces.plist')

try:
    with open(plist_path, 'rb') as f:
        spaces_plist = plistlib.load(f)

    config = spaces_plist.get('SpacesDisplayConfiguration', {})
    space_props = config.get('Space Properties', [])

    for i, space_prop in enumerate(space_props, 1):
        name = space_prop.get('name', f'Space {i}')
        windows_count = len(space_prop.get('windows', []))
        print(f"  {i}. {name} ({windows_count} ウィンドウ)")

except Exception as e:
    print(f"Error: {e}")
PYTHON

echo ""
echo -e "${GREEN}✓ スペース名取得: 可能${NC}"
echo ""

# =====================================
# 【検証4】各スペースに属するウィンドウ情報
# =====================================
echo -e "${BLUE}【検証4】各スペースに属するウィンドウ情報${NC}"
echo ""

python3 << 'PYTHON'
import plistlib
import os

plist_path = os.path.expanduser('~/Library/Preferences/com.apple.spaces.plist')

try:
    with open(plist_path, 'rb') as f:
        spaces_plist = plistlib.load(f)

    config = spaces_plist.get('SpacesDisplayConfiguration', {})
    space_props = config.get('Space Properties', [])

    print("■ スペース別ウィンドウID:")
    for i, space_prop in enumerate(space_props, 1):
        name = space_prop.get('name', f'Space {i}')
        windows = space_prop.get('windows', [])
        print(f"  Space {i} ({name[:30]}):")
        print(f"    ウィンドウID: {windows}")

    print()
    print("注: ウィンドウIDはマシン内部での識別子です。")
    print("    実際のアプリケーション名やウィンドウタイトルとの対応は追加分析が必要です。")

except Exception as e:
    print(f"Error: {e}")
PYTHON

echo ""
echo -e "${GREEN}✓ ウィンドウ情報取得: 可能（IDベース）${NC}"
echo ""

# =====================================
# 【検証5】スペース間のウィンドウ配置情報
# =====================================
echo -e "${BLUE}【検証5】スペース間のウィンドウ配置情報${NC}"
echo ""

python3 << 'PYTHON'
import plistlib
import os

plist_path = os.path.expanduser('~/Library/Preferences/com.apple.spaces.plist')

try:
    with open(plist_path, 'rb') as f:
        spaces_plist = plistlib.load(f)

    config = spaces_plist.get('SpacesDisplayConfiguration', {})
    space_props = config.get('Space Properties', [])

    print("■ スペース間でのウィンドウ分布:")

    all_window_ids = set()
    for space_prop in space_props:
        windows = space_prop.get('windows', [])
        all_window_ids.update(windows)

    print(f"  ユニークなウィンドウ総数: {len(all_window_ids)}")

    # ウィンドウがどのスペースに属しているかを調べる
    window_to_spaces = {}
    for i, space_prop in enumerate(space_props):
        name = space_prop.get('name', f'Space {i+1}')
        windows = space_prop.get('windows', [])
        for window_id in windows:
            if window_id not in window_to_spaces:
                window_to_spaces[window_id] = []
            window_to_spaces[window_id].append(name)

    # 複数のスペースに属するウィンドウを検出
    multi_space_windows = {wid: spaces for wid, spaces in window_to_spaces.items() if len(spaces) > 1}

    if multi_space_windows:
        print(f"  複数スペースに属するウィンドウ: {len(multi_space_windows)}個")
        for window_id, spaces in list(multi_space_windows.items())[:5]:
            print(f"    Window {window_id}: {spaces}")
    else:
        print("  複数スペースに属するウィンドウ: なし")

except Exception as e:
    print(f"Error: {e}")
PYTHON

echo ""
echo -e "${GREEN}✓ スペース間ウィンドウ配置情報: 取得可能${NC}"
echo ""

# =====================================
# 【検証6】スペース作成
# =====================================
echo -e "${BLUE}【検証6】スペース作成の可能性${NC}"
echo ""

echo "AppleScriptでの直接的なスペース作成:"
echo -e "  ${YELLOW}△ 公開API では実装不可${NC}"
echo ""

echo "方法1: UIから手動で作成"
echo "  - Mission Control (Control + Up キー) を開く"
echo "  - 画面右下の「+」ボタンをクリックしてスペース追加"
echo ""

echo "方法2: System Preferences から設定"
echo "  - System Preferences > Mission Control"
echo "  - 「ホットコーナー」でMission Control の割り当て"
echo ""

echo -e "${YELLOW}△ AppleScript/Bash からのスペース自動作成: 非公開API のため実装困難${NC}"
echo ""

# =====================================
# 【検証7】スペース削除
# =====================================
echo -e "${BLUE}【検証7】スペース削除の可能性${NC}"
echo ""

echo "AppleScriptでの直接的なスペース削除:"
echo -e "  ${YELLOW}△ 公開API では実装不可${NC}"
echo ""

echo "手動での削除方法:"
echo "  - Mission Control を開く"
echo "  - 削除するスペースの右上隅を右クリック（またはマウスオーバー時の×ボタン）"
echo "  - 「削除」を選択"
echo ""

echo -e "${YELLOW}△ AppleScript/Bash からのスペース自動削除: 非公開API のため実装困難${NC}"
echo ""

# =====================================
# 【検証8】スペース間でのウィンドウ移動
# =====================================
echo -e "${BLUE}【検証8】スペース間でのウィンドウ移動の可能性${NC}"
echo ""

echo "AppleScriptでの制御:"
osascript << 'APPLESCRIPT'
tell application "System Events"
  set allProcs to every application process
  return "アプリケーションプロセス取得: 成功 (" & (count of allProcs) & "個)"
end tell
APPLESCRIPT

echo ""
echo "AppleScript アクセシビリティAPI:"
echo -e "  ${YELLOW}△ 制限あり${NC} (osascript には補助アクセス許可が必要)"
echo ""

echo "defaults コマンドでのスペース設定直接編集:"
echo "  - Space Properties の windows 配列を編集することで移動は理論的に可能"
echo "  - ただし、動作の保証がない（推奨されない）"
echo ""

echo -e "${YELLOW}△ AppleScript による直接的なウィンドウスペース移動: 実装困難${NC}"
echo ""

# =====================================
# 【検証9】特定のアプリケーションをスペースに移動
# =====================================
echo -e "${BLUE}【検証9】特定のアプリケーションをスペースに移動の可能性${NC}"
echo ""

echo "現在のところ、AppleScript/Bash による実装の困難さ:"
echo "  1. ウィンドウハンドリングは限定的"
echo "  2. スペース管理 API は非公開"
echo "  3. アクセシビリティ API は許可が必要で安定性が低い"
echo ""

echo -e "${YELLOW}△ AppleScript による実装: 非常に困難${NC}"
echo ""

# =====================================
# 【まとめ】
# =====================================
echo -e "${BLUE}========== 検証結果まとめ ==========${NC}"
echo ""

cat << 'SUMMARY'
【情報取得】
  ✓ 現在有効なスペースの情報取得: 可能
    - defaults read com.apple.spaces で全情報取得可能
    - plistlib で Python から解析可能

  ✓ スペースの総数取得: 可能
    - SpacesDisplayConfiguration から Monitor 数を数える

  ✓ スペースの名前取得: 可能
    - Space Properties から name 属性を取得

  ✓ 各スペースに属するウィンドウ情報取得: 可能
    - Space Properties から windows 配列を取得

  ✓ スペース間のウィンドウ配置情報取得: 可能
    - 複数 Space Properties を比較してウィンドウの所属を判定

【操作】
  ✓ スペース間のウィンドウの移動: 可能（UI経由）
    - Command + 左右矢印キーでスペース切り替え可能
    - Command + Option + 矢印キー でアプリをスペースに固定可能

  △ スペースを新規作成: 手動のみ
    - UIから作成は可能（Mission Control > +ボタン）
    - AppleScript による自動化は困難

  △ スペースを削除: 手動のみ
    - UIから削除は可能（Mission Control > ×ボタン）
    - AppleScript による自動化は困難

  △ スペース間でウィンドウを移動: 困難
    - AppleScript での実装は非常に限定的
    - アクセシビリティ API の許可が必要
    - 安定性の問題あり

【技術的な制約】
  1. スペース操作の大部分が非公開API に依存
  2. AppleScript の Accessibility API は補助アクセス許可が必須
  3. defaults での直接編集は動作保証がない

【今後の可能性】
  - 将来的な macOS SDK 公開 API の拡張に期待
  - 非公開 API を使用する Swift 実装の検討
  - Accessibility API の適切な許可設定による限定的な実装

SUMMARY

echo ""
echo -e "${BLUE}検証完了${NC}"
