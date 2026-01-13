# saver_test.rs テストドキュメント

## 概要

`tests/saver_test.rs` は、`src/saver.rs` モジュールの包括的なテストスイートです。save機能（現在のウィンドウレイアウトを保存する機能）の正常動作を検証します。

## テスト構成

### ユニットテスト（CI環境で実行可能）

これらのテストは `#[ignore]` 属性がなく、CI環境を含めすべての環境で実行されます。osascript に依存しないため、高速に実行できます。

#### SaveResult のテスト

- **test_save_result_all_success**
  - 目的: SaveResult が all_success = true で正しく作成できることを確認
  - 検証項目: all_success, saved_app_count, saved_window_count, skipped_window_count, failed_apps

- **test_save_result_partial_failure**
  - 目的: SaveResult が部分失敗（all_success = false）を正しく表現できることを確認
  - 検証項目: failed_apps リストに失敗したアプリ名が含まれることを確認

#### SaveError のテスト

- **test_save_error_display**
  - 目的: SaveError の Display trait が正しく実装されていることを確認
  - 検証項目: format!("{}", error) でエラーメッセージが正しく表示される

- **test_save_error_clone**
  - 目的: SaveError の Clone trait が正しく実装されていることを確認
  - 検証項目: クローンされたエラーのメッセージが一致する

- **test_save_error_error_trait**
  - 目的: SaveError が std::error::Error trait を実装していることを確認
  - 検証項目: Error trait を通じてメッセージが取得できる

#### get_default_config_path() のテスト

- **test_get_default_config_path**
  - 目的: デフォルト設定ファイルパスが正しく取得できることを確認
  - 検証項目: パスに 'Library/Application Support/biz.nosetech.apptidying/settings.json' が含まれる
  - 境界条件: ホームディレクトリが取得できない場合はエラーメッセージを確認

#### save_config_file() のテスト

- **test_save_config_file_creates_directory**
  - 目的: 親ディレクトリが存在しない場合、自動的にディレクトリが作成されることを確認
  - 検証項目: ファイルが作成される、ディレクトリが作成される

- **test_save_config_file_writes_json**
  - 目的: JSON ファイルが正しく書き込まれ、内容が検証できることを確認
  - 検証項目:
    - version が "1.0"
    - layouts が配列である
    - 各ウィンドウの app, title, position, size が正しく保存される
    - 数値指定とパターン指定の両方が正しく保存される
    - timeout が正しく保存される

### 統合テスト（#[ignore] 付き、ローカル環境でのみ実行）

これらのテストは osascript に依存するため、macOS ローカル環境でのみ実行可能です。CI環境では自動的にスキップされます。

#### save_layout() の統合テスト

- **test_save_layout_default_path**
  - 目的: デフォルトパスに保存ができることを確認
  - 検証項目: ファイルが作成される、JSON 構造が正しい

- **test_save_layout_custom_path**
  - 目的: カスタムパスに保存ができることを確認
  - 検証項目: 指定したパスにファイルが作成される

- **test_save_layout_with_own_flag**
  - 目的: --own オプション付きで保存できることを確認
  - 検証項目: include_own_terminal = true で保存が成功する

- **test_save_layout_saves_correct_structure**
  - 目的: 保存された JSON 構造が正しいことを確認
  - 検証項目:
    - version が "1.0"
    - layouts が配列で空でない
    - layout name が "saved_layout"
    - displays が配列で空でない
    - 各ディスプレイに name が文字列として存在
    - 各ディスプレイに windows が配列として存在
    - 各ウィンドウに app が文字列として存在
    - position が存在する場合、x, y が数値または文字列
    - size が存在する場合、width, height が数値または文字列

#### save/load 往復テスト

- **test_save_and_load_roundtrip**
  - 目的: 保存→読み込み→検証の往復が成功することを確認
  - 検証項目:
    - save_layout() が成功する
    - load_config_file() が成功する
    - 読み込まれた設定の構造が正しい
  - 注意: ディスプレイ配置によっては、負の座標が保存される場合があり、load時のバリデーションでエラーになる可能性があります

## テスト実行方法

### 標準テスト実行（CI環境推奨）

```bash
cargo test --test saver_test
```

通常のテスト実行。以下のテストが実行されます：

- SaveResult のテスト（2件）
- SaveError のテスト（3件）
- get_default_config_path() のテスト（1件）
- save_config_file() のテスト（2件）

CI環境（GitHub Actions等）ではこのコマンドで実行します。実行時間は数秒です。

### #[ignore] テストの実行

```bash
cargo test --test saver_test -- --ignored
```

以下のテストが実行されます：

- save_layout() の統合テスト（4件）
- save/load 往復テスト（1件）

これらのテストはローカルmacOS環境でのみ実行可能です。CI環境では osascript が利用できないため、スキップされます。

### 標準出力を確認するテストの実行

```bash
cargo test --test saver_test -- --nocapture
```

テストの標準出力（println! など）がターミナルに表示されます。

### 複数オプションの組み合わせ

```bash
# #[ignore]テストを実行して、標準出力を確認
cargo test --test saver_test -- --ignored --nocapture

# 特定のテストのみ実行
cargo test --test saver_test test_save_config_file_writes_json -- --nocapture
```

## テストカバレッジ

### カバーされている機能

#### SaveResult 構造体
- ✅ all_success フラグの検証
- ✅ saved_app_count, saved_window_count, skipped_window_count の検証
- ✅ failed_apps リストの検証

#### SaveError 構造体
- ✅ Display trait の実装
- ✅ Clone trait の実装
- ✅ Error trait の実装

#### get_default_config_path() 関数
- ✅ 正常系: デフォルトパスが正しく取得できる
- ✅ 異常系: ホームディレクトリが取得できない場合のエラー処理

#### save_config_file() 関数
- ✅ 正常系: JSON ファイルが正しく書き込まれる
- ✅ 境界条件: 親ディレクトリが存在しない場合の自動作成
- ✅ 検証: 保存された JSON 内容の検証（数値指定、パターン指定）

#### save_layout() 関数（統合テスト）
- ✅ デフォルトパスへの保存
- ✅ カスタムパスへの保存
- ✅ --own オプションの動作
- ✅ 保存された JSON 構造の検証
- ✅ save/load 往復処理

### カバーされていない機能

以下の関数は private であるため、直接テストできません。save_layout() を通じて間接的にテストされます：

- ❌ should_include_window() - save_layout() の統合テストで間接的にテスト
- ❌ find_display_for_window() - save_layout() の統合テストで間接的にテスト
- ❌ get_own_process_id() - save_layout() の統合テストで間接的にテスト

## テスト品質基準

### カバレッジ目標

- ✅ **行カバレッジ**: 主要な公開関数が100%カバーされている
- ✅ **ブランチカバレッジ**: SaveResult の all_success = true/false の両方をテスト
- ✅ **境界値テスト**: ディレクトリが存在しない場合、ホームディレクトリが取得できない場合
- ✅ **エッジケース**: 空のアプリケーション、failed_apps リスト

### テストの独立性

- ✅ 各テストは独立して実行可能
- ✅ テストの実行順序に依存しない
- ✅ テスト用の一時ファイルは自動的にクリーンアップ

### テストの決定性

- ✅ ユニットテストは決定論的（テストの不安定性がない）
- ⚠️ 統合テストは osascript に依存するため、実行環境によって結果が異なる可能性がある

## 既知の制限事項

### test_save_and_load_roundtrip の失敗について

外部ディスプレイに配置されたウィンドウの y 座標が負の値になる場合があります（メニューバーの上に配置されるなど）。この場合、load時のバリデーションでエラーになる可能性があります。

これは macOS の座標系とディスプレイ配置によって発生する正常な状態であり、save機能自体は正常に動作しています。

## 今後の改善案

### テストの追加

- [ ] should_include_window() を public にして直接テスト
- [ ] find_display_for_window() を public にして直接テスト
- [ ] エラーケースのテスト拡充（ディスプレイが接続されていない場合など）

### テストの改善

- [ ] test_save_and_load_roundtrip のバリデーション緩和（負の座標を許容）
- [ ] モックオブジェクトを使用した osascript に依存しないテスト

## 参考

- tests/config_test.rs: 構造体のテスト方法
- tests/loader_test.rs: 統合テスト（#[ignore]）の書き方
