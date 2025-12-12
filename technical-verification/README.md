# Technical Verification Scripts - 技術検証スクリプト集

このディレクトリには、macOSのウィンドウ管理機能に関する技術検証スクリプトが含まれています。App Tidying プロジェクトの実装可能性調査や機能検証に使用されます。

## スクリプト一覧

### 1. `move_to_external_display.sh`

**目的**: 指定したアプリケーションを外部ディスプレイの中央に移動させる

**機能**:
- 引数で指定されたアプリケーション名を受け取る
- アプリケーションの起動状態を確認（起動していなければ起動する）
- ウィンドウが表示されるまで最大10秒待機
- 外部ディスプレイの座標情報を取得（Finder経由）
- AppleScriptでウィンドウのサイズと位置を設定

**使用例**:
```bash
./move_to_external_display.sh "Google Chrome"
./move_to_external_display.sh "Finder"
```

**技術詳細**:
- 外部ディスプレイの座標: `左上: (-560, -1440)`, サイズ: `2560x1440`（ハードコード値）
- ウィンドウ待機時間: 最大10秒（1秒ごとにウィンドウ数をチェック）
- AppleScript経由でウィンドウ操作を実行

---

### 2. `detect_stdout.sh`

**目的**: 標準出力が利用可能かどうかを判定し、実行環境に応じて出力方法を切り替える

**機能**:
- TTY（端末）が接続されているかをチェック（`-t 1`）
- `$TERM` 環境変数の有無を確認
- ターミナル実行時は標準出力に出力
- ターミナル外実行時（Automator等）はダイアログで表示

**判定ロジック**:
1. TTY接続: `if [ -t 1 ]` - 標準出力利用可能
2. TERM環境変数: `if [ -n "$TERM" ]` - 標準出力利用可能
3. その他: ダイアログ表示（標準出力利用不可）

**使用例**:
```bash
./detect_stdout.sh
```

**応用例（logger.rsでの実装）**:
```rust
fn is_running_in_terminal() -> bool {
    std::env::var("TERM").is_ok()
}
```

---

### 3. `test_stdout_detection.sh`

**目的**: `detect_stdout.sh`の標準出力判定機能を複数の環境でテストする

**テスト項目**:
1. **テスト1**: ターミナルでの通常実行（標準出力利用可能）
2. **テスト2**: パイプ経由での実行（標準出力利用不可をシミュレート）
3. **テスト3**: ファイルへのリダイレクト（標準出力利用不可をシミュレート）
4. **テスト4**: stdoutが閉じられている状態
5. **テスト5**: TERM環境変数が未設定の環境

**使用例**:
```bash
./test_stdout_detection.sh
```

**実行環境シミュレーション**:
- `bash script.sh 2>&1 | cat` - パイプ環境テスト
- `bash script.sh > /tmp/file` - リダイレクト環境テスト
- `TERM="" bash script.sh` - 環境変数空の環境テスト

---

### 4. `get_display_info.sh`

**目的**: macOSのディスプレイ情報を詳細に取得する

**取得情報**:
- **JXA（JavaScript for Automation）経由**:
  - ディスプレイ名（ローカライズされた名前）
  - 物理解像度（ピクセル数）
  - UI解像度（論理解像度）
  - スケーリング係数（Retinaディスプレイ判定）
  - メインディスプレイの判定

- **AppleScript経由**:
  - 全体の画面境界座標（左, 上, 右, 下）

**技術詳細**:
- NSScreen フレームワークでディスプレイ情報取得
- `backingScaleFactor` でRetinaディスプレイのスケーリング係数を取得
- `localizedName` でシステム言語に対応したディスプレイ名を取得

**使用例**:
```bash
./get_display_info.sh
```

**出力例**:
```
ディスプレイ 0: Built-in （メイン）
  物理解像度: 2560x1600
  UI解像度: 1280x800
  スケール係数: 2.0x
```

---

### 5. `get_window_info.sh`

**目的**: 実行中のアプリケーションのウィンドウ情報を取得し、各ウィンドウがどのディスプレイに位置しているかを判定する

**機能**:
- システム設定ファイルからディスプレイ配置情報を取得（`windowserver.displays.plist`）
- Python3で plistlib を使用してディスプレイ設定をパース
- JXAでディスプレイ名を取得
- AppleScriptで各アプリケーションのウィンドウ情報を取得
- ウィンドウの位置からディスプレイを判定

**ウィンドウ-ディスプレイ判定アルゴリズム**:
1. **中心座標判定**: ウィンドウ中心がディスプレイ内に含まれるかチェック
2. **重なり面積判定**: 複数ディスプレイに跨る場合、最大重なり面積のディスプレイを選択
3. **最近接ディスプレイ判定**: どのディスプレイとも重ならない場合、最も近いディスプレイを選択

**使用例**:
```bash
./get_window_info.sh
```

**出力形式**:
```
Google Chrome | Development | 0,25 | 1440x900 | Built-in
Safari | Homepage | 1920,100 | 1024x768 | External Display
```

**技術詳細**:
- AABB衝突判定（軸並行境界ボックス）でウィンドウ-ディスプレイの重なりを判定
- ディスプレイ座標は `OriginX, OriginY, Wide, High` で定義

---

### 6. `verify_spaces.sh`

**目的**: macOSの仮想デスクトップ（スペース）機能に関する技術的な制約と可能性を検証

**検証項目**:
1. **スペース情報取得**: ✓ 可能
   - `com.apple.spaces.plist` から全スペース情報を取得可能
   - スペース数、スペース名、ウィンドウ割り当て情報を取得可能

2. **スペース総数取得**: ✓ 可能
   - Management Data から Monitor ごとのスペース数を集計

3. **スペース名取得**: ✓ 可能
   - Space Properties から name 属性を取得

4. **各スペースのウィンドウ情報**: ✓ 可能（ID形式）
   - ウィンドウID のみ取得可能
   - アプリケーション名との対応付けには追加分析が必要

5. **スペース作成/削除**: △ 手動のみ
   - AppleScript による自動化は不可（非公開API）

6. **スペース間のウィンドウ移動**: △ 困難
   - AppleScript での実装は非常に限定的
   - アクセシビリティAPI の許可が必須

**結論**:
- ✓ 情報取得は Python + plistlib で可能
- △ 操作の大部分は非公開API に依存し実装困難
- 将来的な公開API 拡張に期待

**使用例**:
```bash
./verify_spaces.sh
```

---

### 7. `verify_stage_manager.sh`

**目的**: macOS 13 Ventura以降のステージマネージャ機能の状態と、ウィンドウ操作への影響を検証

**検証内容**:
- macOSのシステム情報取得
- ステージマネージャの有効/無効状態確認
- `com.apple.WindowManager` の設定キー探索
- AppleScript、JXA、defaults コマンドでのステージマネージャ検出
- ウィンドウアクセスの動作検証
- Mission Control の設定確認

**発見事項**:
- AppleScript での直接的なステージマネージャ操作は不可
- 検査可能: ウィンドウの取得、位置・サイズの取得（ただし不正確な可能性あり）
- ステージマネージャが有効な場合:
  - アクティブなグループ内のウィンドウのみ完全にアクセス可能
  - バックグラウンドグループのウィンドウは不完全または不正な位置・サイズデータを返す可能性
  - ウィンドウレイアウト復元が正常に機能しない可能性

**使用例**:
```bash
./verify_stage_manager.sh
```

---

### 8. `verify_stage_manager_detailed.sh`

**目的**: ステージマネージャの詳細な状態確認と、有効/無効がウィンドウ操作に与える具体的な影響を検証

**確認項目**:
- macOSバージョン取得
- ステージマネージャの有効状態確認（`defaults read com.apple.WindowManager GloballyEnabled`）
- `com.apple.WindowManager` の全キー表示
- ウィンドウ取得テスト（有効時の動作検証）
- ステージマネージャグループ情報（WindowServer管理）
- ウィンドウアクセスの制限事項

**制限事項（ステージマネージャ有効時）**:
1. **ウィンドウアクセスの制限**:
   - アクティブグループ内のウィンドウのみ完全にアクセス可能
   - バックグラウンドグループは不完全なデータを返す可能性

2. **ウィンドウの位置・サイズへの影響**:
   - グループ相対座標での表現（画面座標では表現されない可能性）
   - 実際の表示サイズとの乖離

3. **複数アプリのグループ化**:
   - 複数アプリが1つのステージマネージャグループに属する可能性
   - グループ化解除はAppleScriptではサポートされない

4. **実装上の推奨事項**:
   - ウィンドウ操作前にステージマネージャ状態をチェック
   - 有効な場合の警告表示をユーザーに提供
   - 一時的な無効化の検討

**使用例**:
```bash
./verify_stage_manager_detailed.sh
```

---

### 9. `open_new_window.sh`

**目的**: AppleScriptを使用して、指定したアプリケーションの新規ウィンドウをメニュー操作で開く

**対応アプリケーション**:
- Finder
- Safari
- Google Chrome

**機能**:
- アプリケーション名を引数で指定
- アプリケーション起動状態をチェック（起動していなければ起動）
- AppleScriptでメニューバーにアクセス
- File / ファイル メニューを検索
- 「New Window」「新規ウィンドウ」メニューアイテムをクリック

**メニュー検索ロジック**:
1. メニューバーを取得: `menu bar 1`
2. ファイルメニューを検索: "File" または "ファイル"
3. メニューアイテムをリスト化
4. 「New Window」または「新規ウィンドウ」を含むアイテムをクリック

**使用例**:
```bash
./open_new_window.sh Finder
./open_new_window.sh Safari
./open_new_window.sh Chrome
```

**出力例**:
```
[INFO] Google Chrome の新規ウィンドウを開きます...
[SUCCESS] Google Chrome の新規ウィンドウを開きました
```

---

### 10. `verify_new_window_menu.sh`

**目的**: AppleScriptでのメニューアイテム検索・実行機能の包括的な検証と実装例を提供

**検証テスト**:
1. **テスト1**: AppleScriptでメニュー構造へのアクセス可能性
2. **テスト2**: File メニュー内のメニューアイテム列挙
3. **テスト3**: Safari でのメニュー検索
4. **テスト4**: 汎用的な新規ウィンドウ実装例の表示
5. **テスト5**: Chrome での多言語メニュー検索テスト

**検証結果（全て ✓ 可能）**:
- AppleScriptでメニュー構造にアクセス可能
- 「New Window」「新規ウィンドウ」などの動的検索が可能
- メニューアイテムのクリック/実行が可能
- 複数アプリケーション（Finder、Safari、Chrome）で動作可能

**実装上の注意点**:
- **多言語対応**: メニュー名が言語設定によって異なる
  - 英語: "File", "New Window"
  - 日本語: "ファイル", "新規ウィンドウ"
- **エラーハンドリング**: メニューが存在しないアプリケーションもある
- **Accessibility API の許可**: UI要素へのアクセスには許可が必須
- **アプリケーション固有の対応**: メニュー項目がアプリごとに異なる

**使用例**:
```bash
./verify_new_window_menu.sh
```

**実装例（AppleScript）**:
```applescript
on openNewWindow(appName)
    tell application "System Events"
        tell process appName
            activate
            set menubar to menu bar 1
            set fileMenu to (first menu of menubar whose name is "File")

            repeat with mi in (every menu item of fileMenu)
                if name of mi contains "New Window" then
                    click mi
                    return "Success"
                end if
            end repeat
        end tell
    end tell
end openNewWindow
```

---

## 実行環境要件

- macOS 10.13 以上（AppleScript対応）
- Bash 3.2 以上
- Python 3.6 以上（plistlib使用）
- JXA対応macOS 10.10 以上

### 前提条件

**Accessibility API の許可**:
- システム設定 > セキュリティとプライバシー > アクセシビリティ
- ターミナル（またはターミナルアプリ）を追加

---

## 技術的な制約と今後の課題

### 実装可能な機能
- ✓ ウィンドウ情報の取得（位置、サイズ、タイトル）
- ✓ ウィンドウの移動・リサイズ
- ✓ アプリケーションの起動・活性化
- ✓ ディスプレイ情報の取得
- ✓ 新規ウィンドウの開き方（メニュー経由）

### 実装困難な機能
- △ スペース（仮想デスクトップ）の作成・削除・移動
- △ ステージマネージャ設定の制御
- △ ウィンドウの正確な位置・サイズ検出（ステージマネージャ有効時）

### 今後の改善案
- 非公開API（SweetBread など）の安全な使用法調査
- Swift実装での機能拡張
- macOS API の公開拡張に対応した設計

---

## 関連するGitHubイシュー

- Issue #4: macOS仮想デスクトップ（スペース）対応検証
- Issue #26: 複数ウィンドウ・新規ウィンドウ対応
- Issue #48: ステージマネージャ・マルチディスプレイ対応

---

## 参考資料

- [AppleScript Language Guide](https://developer.apple.com/library/archive/documentation/AppleScript/Conceptual/AppleScriptLangGuide/)
- [System Events Dictionary](https://developer.apple.com/library/archive/documentation/AppleScript/Conceptual/System_Events/)
- [macOS System Preferences Configuration](https://support.apple.com/guide/mac-help)
- [Accessibility API Documentation](https://developer.apple.com/accessibility/)

