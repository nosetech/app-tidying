# App Tidying Project

## プロジェクトの目的

macOS で起動しているアプリケーションのウィンドウのサイズと位置をユーザーが決めた通りに設定する整頓アプリを作成する。

ユーザーがウィンドウの配置やサイズを事前に設定ファイル（JSON）で定義しておき、ワンクリック（またはコマンド実行）で理想的なウィンドウレイアウトを復元できるようにすることが目標。

さらに、RightCheat と連携して、複数のパターンでのウィンドウ配置を簡単に切り替えられる環境を実現する。

## 開発ルール

### 言語・コーディング規約

#### 日本語使用ルール（必須）

プロジェクト全体で**日本語の使用を徹底**します。以下を対象とします：

- **コメント**: ソースコード内のコメントは必ず日本語で記述
- **ドキュメント**: README、CLAUDE.md、その他の説明文は日本語で記述
- **コミットメッセージ**: Gitのコミットメッセージは日本語で記述
- **プルリクエスト**: PRのタイトル、説明、レビューコメントは日本語で記述
- **Changelog**: バージョン履歴やリリースノートは日本語で記述
- **テストメッセージ**: テストコードのアサーションメッセージは日本語で記述

#### 英語を使用する例外

以下の場合のみ、英語（または英語表記）を使用してください：

- **ライブラリ・フレームワーク名**: `cargo`、`Rust`、`AppleScript`、`JXA` など
- **API名・関数名**: `get_all_connected_displays()`、`NSScreen.screens` など
- **技術用語**: `JSON`、`API`、`osascript`、`GitHub Actions` など
- **固有名詞**: `macOS`、`Accessibility API`、`System Events` など

#### 具体例

✅ **推奨例（日本語）**:

```rust
/// すべての接続ディスプレイ情報を取得する
///
/// JXAを使用してmacOSのNSScreen APIから全ディスプレイ情報を取得します。
pub fn get_all_connected_displays() -> Result<Vec<DisplayInfo>, DisplayError> {
    // ディスプレイ情報を取得
    // ...
}
```

❌ **非推奨例（英語混在）**:

```rust
/// Get information about all connected displays
///
/// This function uses JXA to retrieve display information from macOS NSScreen API.
pub fn get_all_connected_displays() -> Result<Vec<DisplayInfo>, DisplayError> {
    // Get display information
    // ...
}
```

✅ **プルリクエスト例（日本語）**:

```
タイトル: feat: P2-3-1: 接続ディスプレイ一覧取得スクリプト実装

説明:
- JXAで全接続ディスプレイ情報を取得
- 各ディスプレイの名前、解像度、原点座標を取得
```

#### 効果

日本語使用を徹底することで：

- チームの認識統一（全員が同じ言語で思考）
- メンテナンス性の向上（日本語の方が文脈が理解しやすい）
- ドキュメント整合性の向上（説明とコードが同じ言語）
- 日本人開発者の生産性向上（母語で記述するため誤解が少ない）

## アーキテクチャ方針

### 実装言語の役割分担

- **Rust（メインアプリケーション）**
  - CUIアプリケーションの実装
  - JSON設定ファイルの解析
  - 設定管理とウィンドウ配置ロジック
  - ユーザー通知とログ出力
  - クロスプラットフォーム対応を視野に入れたコード設計
  - OSに依らない共通ロジックを実装

- **AppleScript/JXA（macOS固有処理）**
  - `osascript` コマンドで実行
  - アプリケーション起動
  - ウィンドウ情報の取得（位置、サイズ、タイトル）
  - ウィンドウの移動・リサイズ
  - ディスプレイ情報の取得

  **実装時は `technical-verification/` の検証結果を参考にする**:
  - スクリプト実装前に `technical-verification/README.md` の対応技術を確認
  - 実装困難な機能（スペース操作、ステージマネージャ制御など）の制約を理解
  - 既存の検証スクリプトを実行して、ローカル環境での動作確認を実施
  - 多言語対応（英語/日本語メニュー）が必要な場合は `verify_new_window_menu.sh` の実装例を参考

- **Bash Script（補助処理）**
  - 必要に応じてスクリプト管理用に使用

### macOS標準コマンドの使用

- **積極的に使用する**
  - `osascript`（AppleScript実行）
  - `system_profiler`（システム情報取得）
  - その他の `/usr/bin`、`/usr/sbin` に含まれる標準コマンド

- **外部ツールについて**
  - Homebrewでインストールが必要なツールは避ける
  - ただし、開発環境では Rust ツールチェーンの使用は許可
  - リリースは Rust コンパイル済みバイナリで配布（ユーザーは Rust インストール不要）

### 将来のマルチプラットフォーム対応

- Windows版の実装を想定
- OS依存処理（ウィンドウ操作）をRust側でカプセル化
- AppleScript相当の処理を Windows 用に別実装
- 共通ロジックはプラットフォーム間で再利用可能な設計を心がける

## CLIコマンド仕様

### コマンド名

`apptidying`

### 基本コマンド

#### load: ウィンドウ配置を復元

```bash
apptidying load                              # デフォルト設定を使用
apptidying load <path/to/layout.json>        # 指定ファイルを使用
```

#### save: 現在のウィンドウ配置を保存

```bash
apptidying save                              # デフォルト設定に保存
apptidying save <path/to/layout.json>        # 指定ファイルに保存
apptidying save --own                        # ターミナルウィンドウも含めて保存
apptidying save --own <path/to/layout.json>  # ターミナルウィンドウも含めて、指定ファイルに保存
```

### グローバルオプション

- `-v, --verbose`: デバッグ出力有効化
- `-h, --help`: ヘルプ表示
- `-V, --version`: バージョン表示

## 設定ファイル仕様

### ファイルパスとデフォルト設定

- **デフォルト設定ファイル**: `~/Library/Application Support/biz.nosetech.apptidying/settings.json`
- **ログファイル**: `~/Library/Application Support/biz.nosetech.apptidying/apptidying.log`

### 設定ファイルフォーマット

#### 基本構造

```json
{
  "version": "1.0",
  "layouts": [
    {
      "displays": [
        {
          "name": "Built-in",
          "windows": [
            {
              "app": "Google Chrome",
              "title": "Development",
              "position": { "x": 0, "y": 0 },
              "size": { "width": 1440, "height": 900 }
            }
          ]
        }
      ]
    }
  ]
}
```

#### サポートされる指定方式

##### 1. ピクセル単位での指定（絶対位置）

```json
{
  "position": { "x": 0, "y": 25 },
  "size": { "width": 1440, "height": 900 }
}
```

##### 2. パターン指定（相対位置・推奨）

```json
{
  "position": { "x": "left", "y": "top" },
  "size": { "width": "half", "height": "max" }
}
```

**位置の値**:

- `x`: `left`, `right`
- `y`: `top`, `bottom`

**サイズの値**:

- `width`: `half` (画面の1/2), `third` (画面の1/3), `max` (フル幅)
- `height`: `half`, `third`, `max`

### フォーマットバージョニング

- JSON ファイル内に `version` フィールドを含める
- 現在サポートバージョン: `1.0`
- 扱えないバージョンの場合はエラー表示
- 古いバージョンからの移行は README で説明（アプリケーション側では自動変換しない）

### マルチウィンドウ対応

同一アプリケーションで複数ウィンドウを操作する場合、`title` フィールドで識別：

```json
{
  "app": "Google Chrome",
  "title": "Development Tab",
  "position": { "x": "left", "y": "top" },
  "size": { "width": "half", "height": "max" }
}
```

## ユーザー通知方針

### メッセージレベルと通知方式

実行コンテキストにより通知方式を自動選択：

#### ターミナル実行時

- すべてのメッセージを標準出力に出力
- ログレベル: `DEBUG`, `INFO`, `WARN`, `ERROR`

#### ターミナル外での実行時

- **INFO, WARN レベル**: macOS通知センターで通知
- **ERROR レベル**: ダイアログ表示（長い文言はログファイル参照案内）

### ログファイル

- **出力先**: `~/Library/Application Support/biz.nosetech.apptidying/apptidying.log`
- **ログレベル**: `DEBUG`, `INFO`, `WARN`, `ERROR`
- **DEBUG出力**: 開発モード時のみ（`--verbose` フラグ使用時）
- **ログ記録方針**: 標準出力や通知センター/ダイアログに表示するすべてのメッセージはログファイルに自動的に記録される
  - ターミナル実行時：標準出力に出力される全メッセージ
  - 非ターミナル実行時：通知センター/ダイアログに表示される全メッセージ
  - タイムスタンプ付き（`YYYY-MM-DD HH:MM:SS`）でログファイルに記録

### 通知のカスタマイズ

設定ファイルで各レベルの通知方式を指定可能：

```json
{
  "notification": {
    "info": "notification", // notification, dialog, none
    "warn": "notification",
    "error": "dialog"
  }
}
```

## エラーハンドリング方針

### 部分的な失敗への対応

複数アプリのウィンドウ操作時：

- 複数アプリの操作に失敗した場合も、成功した分は継続実行
- ワーニングレベルで通知してユーザーに情報提供
- 処理結果を詳細にログ出力

### 失敗時の分類

#### 全体失敗

- すべてのアプリケーション操作が失敗
- **通知**: ERROR（ダイアログ）
- **ログ**: 詳細なエラー情報を記録

#### 部分失敗

- 1つ以上のアプリは成功、1つ以上は失敗
- **通知**: WARN（通知センター）
- **ログ**: 失敗したアプリケーション名と原因を記録

### よくあるエラーケース

| エラー           | 原因                                         | 対応                                       |
| ---------------- | -------------------------------------------- | ------------------------------------------ |
| アプリ不在       | 指定アプリがインストールされていない         | アプリ名をログ出力、部分失敗として処理     |
| ウィンドウなし   | アプリは起動済みだがウィンドウなし           | 新規ウィンドウを起動                       |
| 権限なし         | Accessibility API アクセス許可なし           | ダイアログでエラー表示、設定方法をログ出力 |
| 無効なサイズ     | 画面より大きい、負の座標                     | WARN 表示、その設定は実行しない            |
| ディスプレイなし | 定義ファイルのディスプレイが接続されていない | WARN 表示、他のディスプレイの処理は継続    |

## マルチディスプレイ対応

### ディスプレイ検出

- 接続されているすべてのディスプレイを自動検出
- ディスプレイ名で識別（例: `Built-in`, `Enhanced`, `External Display` など）
- ディスプレイの解像度情報も保存

### 接続・切断時の処理

#### ディスプレイが接続されていない場合

- ワーニング表示（通知センターまたはログ）
- 該当ディスプレイのウィンドウ配置は実行しない
- 他のディスプレイの操作は継続実行

#### ディスプレイの配置や解像度が変わった場合

- 定義ファイルのディスプレイ情報と現在のディスプレイ情報を比較
- 一致しない場合ワーニング表示
- 最善の努力で処理を続行

### 非サポート機能（Ver1.0）

- **ステージマネージャ対応**: 表示中のディスプレイのみサポート
- **仮想デスクトップ（Space）操作**: 表示中の Space のみサポート
- **複数 Space の管理**: 実装予定なし

## アプリケーション起動の詳細仕様

### 起動時の状態判定

| 状況                            | 処理                             |
| ------------------------------- | -------------------------------- |
| アプリ起動済み + ウィンドウあり | 既存ウィンドウを移動・リサイズ   |
| アプリ起動済み + ウィンドウなし | 新規ウィンドウを起動してから操作 |
| アプリ未起動                    | アプリを起動してから操作         |

### 起動完了待機

- **デフォルト待機時間**: 3秒
- **設定方法**: 設定ファイルで `timeout` を指定
  ```json
  {
    "timeout": 5000 // ミリ秒
  }
  ```

### 複数ウィンドウの扱い

同一アプリケーションで複数ウィンドウを操作する場合：

1. JSON で `title` フィールドで指定されたウィンドウを検索
2. 一致するウィンドウがあれば操作
3. ない場合は新規ウィンドウを起動
4. 複数マッチした場合は最前面のウィンドウを優先

### 保存時（save コマンド）の境界条件

#### 最小化されたウィンドウ

- 保存対象外
- INFO メッセージで「最小化されたウィンドウは保存対象外」と通知

#### 非表示のウィンドウ

- 最小化されたウィンドウと同様に処理
- 保存対象外

#### システムウィンドウ

- Finder のように操作可能なウィンドウは保存対象
- メニューバー、ドックは対象外

## 技術的な制約と対応方針

### AppleScript のバージョン対応

- macOS バージョンやセキュリティアップデートにより AppleScript が変更される可能性
- **対応**: プログラム内で `osascript` のバージョンを確認し、バージョン別の実装を用意

### Accessibility API の権限

- ウィンドウ操作には Accessibility API へのアクセス許可が必須
- **対応**:
  - README に明記
  - 権限がない場合は適切なエラーメッセージとセットアップ手順を表示

### ウィンドウサイズの制約

画面外や画面より大きいウィンドウサイズが指定された場合：

- ワーニング表示
- その設定は実行しない（他の設定は継続実行）

### エラーハンドリング

Rust での実装では、以下を原則とする：

- `expect()` や `unwrap()` は避ける
- `Result` を返す設計にする
- プロジェクト固有のエラー型を定義
- ユーザーに適切なエラーメッセージを提供

### save コマンドのターミナルウィンドウ除外機能

`apptidying save` コマンドを `--own` オプションなしで実行した場合、実行中のターミナルアプリケーションのウィンドウを自動的に除外します。

#### 対応ターミナルアプリケーション

以下のターミナルアプリケーションに対応しています：

- Terminal.app（macOS 標準）
- iTerm2 / iTerm
- ghostty
- kitty
- WezTerm
- Alacritty

#### ターミナル特定メカニズム

優先順位：

1. **TERM_PROGRAM 環境変数** → ダイレクトに判定
2. **ターミナル固有の環境変数** → tmux経由実行時に元のターミナルを特定
   - ghostty: `GHOSTTY_BIN_DIR`, `__CFBundleIdentifier`
   - iTerm2: `ITERM_SESSION_ID`, `ITERM_PROFILE`
   - kitty: `KITTY_WINDOW_ID`, `KITTY_LISTEN_ON`
   - WezTerm: `WEZTERM_PANE`, `WEZTERM_EXECUTABLE`
3. **プロセスツリー遡り** → フォールバック

#### 制限事項

- **非サポートターミナル**: 上記以外のターミナルアプリケーション（例：screen経由の実行、カスタムターミナル）を使用している場合、ウィンドウが除外されない可能性があります
- **環境変数なし**: 環境変数が設定されていない特殊な実行環境では、正しく特定できない場合があります
- **tmux以外の多重化ツール**: screen や multiplexer など、tmux以外の多重化ツール経由での実行時は、ターミナルを特定できない可能性があります

非サポートターミナルを使用している場合は、`apptidying save --own` で手動指定してください。

## テスト・品質保証

### テスト対象範囲

- JSON ファイルパースのテスト（正常系・エラー系）
- エラーハンドリングのテスト
- マルチディスプレイシミュレーション
- ログ出力の確認
- AppleScript 実行結果の検証

### テスト実装方針

- テストコードはtests/配下に配置する。
- **テストコード内にコメントを充実させる**
  - 各テスト関数の目的を説明するコメントを記載（`//` または `///` コメント）
  - テスト関数内の処理フローを説明するコメントを記載
  - テストが検証する項目（正常系/異常系/境界条件など）をコメントで明記
  - テストの実行環境や制限事項がある場合は `#[ignore]` の上にコメントで説明
  - 複雑なアサーションには、期待値と実際の値の意味を日本語で説明するコメントを記載

- **テストドキュメント（.md）ファイルは作成しない**
  - テスト説明ドキュメント（`*_test_README.md` など）は作成禁止
  - テストコード自体がドキュメントになるよう、コメントを充実させる
  - テスト実行方法は CLAUDE.md に記載し、個別のドキュメントは作成しない

- **テストコード例（推奨形式）**

  ```rust
  /// SaveResult が all_success = true で正しく作成できることを確認
  #[test]
  fn test_save_result_all_success() {
      // 目的: SaveResult 構造体の正常系動作を検証
      // 検証項目: all_success, saved_app_count, saved_window_count, skipped_window_count, failed_apps

      // テストデータを構築
      let result = SaveResult {
          all_success: true,
          saved_app_count: 3,
          saved_window_count: 5,
          skipped_window_count: 0,
          failed_apps: vec![],
      };

      // 検証: すべてのフィールドが期待通りに設定されている
      assert!(result.all_success);
      assert_eq!(result.saved_app_count, 3);
  }

  /// osascript に依存する統合テスト
  /// ローカル macOS 環境でのみ実行可能（CI環境ではスキップ）
  #[test]
  #[ignore]
  fn test_save_layout_default_path() {
      // 目的: デフォルトパスに保存ができることを確認
      // 環境要件: macOS で osascript が利用可能
      // 制限事項: 実行環境によってウィンドウの配置が異なるため、
      //          JSON 構造の妥当性のみ検証し、具体的なウィンドウ数は検証しない

      let result = save_layout(&get_default_config_path().unwrap(), false);

      // 検証: save_layout が成功している
      assert!(result.is_ok());
  }
  ```

### テスト実行方法

#### 標準テスト実行（CI環境推奨）

```bash
cargo test
```

通常のテスト実行。以下のテストが実行されます：

- 構造体やロジックのユニットテスト
- エラーハンドリングテスト
- ターミナル実行環境のテスト

CI環境（GitHub Actions等）ではこのコマンドで実行します。実行時間は数秒です。

#### #[ignore] テストの実行

**CI環境でテストできないテストケース(osascriptを実行するものなど)には#[ignore]を設定します**

```bash
cargo test -- --ignored
```

以下のテストが実行されます：

- `osascript` 実行に依存する非ターミナル実行テスト

これらのテストはローカルmacOS環境でのみ実行可能です。CI環境では osascript が利用できないため、スキップされます。

#### 標準出力を確認するテストの実行

```bash
cargo test -- --nocapture
```

テストの標準出力（`println!` など）がターミナルに表示されます。通常、テスト実行時は標準出力がキャプチャされて非表示になりますが、このオプションで確認できます。

#### 複数オプションの組み合わせ

```bash
# #[ignore]テストを実行して、標準出力を確認
cargo test -- --ignored --nocapture

# 特定のテストのみ実行（例：logger関連）
cargo test logger -- --nocapture

# 特定の#[ignore]テストのみ実行
cargo test test_show_notification_info_non_terminal -- --ignored --nocapture
```

### テストカバレッジ

#### 現在のテスト実行状況（Issue #31 完了時点）

- **合計テスト数**: 522 テスト
  - 実行可能テスト: 406 テスト（passed）
  - スキップテスト: 116 テスト（ignored、osascript依存）
  - 失敗テスト: 0 テスト

#### テストファイル別の構成

1. **tests/config_test.rs**: 56 テスト
   - JSONパーサ、バリデーション、パターン計算テスト

2. **tests/cli_test.rs**: 32 テスト
   - CLI引数解析、コマンドテスト

3. **tests/integration_test.rs**: 17 テスト
   - JSON設定ファイル統合テスト

4. **tests/loader_test.rs**: 27 テスト（6 passed + 21 ignored）
   - ウィンドウ配置テスト、エラーケーステスト
   - Accessibility API 権限テストを追加

5. **tests/applescript_test.rs**: 207 テスト（128 passed + 79 ignored）
   - AppleScript エスケープ、ディスプレイ情報、ウィンドウ操作テスト

6. **tests/logger_test.rs**: 89 テスト（81 passed + 8 ignored）
   - ログ出力、通知機能テスト

7. **tests/saver_test.rs**: 13 テスト（8 passed + 5 ignored）
   - save/load往復テスト

8. **tests/system_window_detection_test.rs**: 74 テスト
   - システムウィンドウ分類、ターミナル検出テスト

#### テスト実行方法 - カバレッジ測定

ローカル macOS 環境でのテスト実行：

```bash
# 標準テスト実行（CI環境推奨）
cargo test

# すべてのテストを実行（#[ignore]テストも含める）
cargo test -- --ignored

# テスト出力を表示
cargo test -- --nocapture

# 特定のモジュールのテストのみ実行
cargo test loader -- --ignored --nocapture
```

### 本番環境での確認項目

- 複数アプリの同時操作
- 大量ウィンドウ（10+）の処理
- 外部ディスプレイの接続・切断
- Accessibility API 権限エラーの処理
- 異なる macOS バージョンでの動作

## Journaling workflow

InkdropのMCPサーバーを使用できる状態の場合、あなた (AI エージェント) は、このプロジェクトで行った作業を、タスクの終了ごとに私の Inkdrop ノートに報告してください。

「App Tidying」ノートブックに「Log: <Job title>」というタイトルで作成します。
同じセッション全体で同じメモを更新します。

タスクの終了ごとに、次の形式でノートを書いてください。:

## Log: <task title>

- **Prompt**: <受け取った指示>
- **Issue**: <課題の内容>

### What I did: <やったことの要約>

...

### How I did it: <どうやって解決したか>

...

### What were challenging: <難しかったこと>

...

### Future work (optional)

- <今後の改善案など>
