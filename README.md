# App Tidying

macOS のウィンドウレイアウトを自動整頓するアプリケーション

**App Tidying** は、アプリケーションのウィンドウの位置とサイズを設定ファイル（JSON）で定義し、ワンコマンドで理想的なレイアウトを復元するツールです。複数のディスプレイにも対応しており、複雑なウィンドウ配置も簡単に管理できます。

## 主な機能

- **ウィンドウ配置の保存** (`save` コマンド)
  - 現在のウィンドウ配置をJSON形式で保存
  - 複数ディスプレイのレイアウトに対応

- **ウィンドウ配置の復元** (`load` コマンド)
  - 保存したレイアウトを復元
  - 一括ウィンドウ整頓

- **マルチディスプレイ対応**
  - 複数ディスプレイの設定を保存・復元
  - ディスプレイの接続・切断時にも対応

- **柔軟な位置・サイズ指定**
  - ピクセル単位での絶対指定
  - 相対指定（left/right, top/bottom, half/third/max）

- **複数ウィンドウ対応**
  - 同一アプリケーションの複数ウィンドウを個別に操作

- **ターミナルウィンドウの自動除外**
  - `save` コマンド実行時、ターミナルウィンドウを自動で除外（`--own` オプションで包含可能）

## インストール

### 前提条件

- macOS 10.15 以上
- Rust ツールチェーン（ビルド時のみ必要）

### ビルド・インストール手順

1. **Rust ツールチェーンのインストール**

   Rust がインストール済みでない場合は、以下のコマンドを実行してください：

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **リポジトリをクローン**

   ```bash
   git clone https://github.com/nosetech/app-tidying.git
   cd app-tidying
   ```

3. **ビルド**

   ```bash
   cargo build --release
   ```

   ビルド済みバイナリは `target/release/apptidying` に生成されます。

4. **バイナリをインストール**

   ```bash
   # オプション 1: /usr/local/bin にインストール
   sudo cp target/release/apptidying /usr/local/bin/

   # オプション 2: Cargo を使用してインストール
   cargo install --path .
   ```

   インストール後、任意の場所から `apptidying` コマンドを実行できます。

## 初回セットアップ

### Accessibility API 権限の設定

App Tidying がウィンドウを操作するには、Accessibility API へのアクセス許可が必要です。

1. **システム設定を開く**
   - メニューバーから「Apple メニュー」→「システム設定」を選択

2. **プライバシーとセキュリティを選択**
   - 「プライバシーとセキュリティ」を開く

3. **Accessibility を選択**
   - 左側メニューから「Accessibility」をクリック

4. **ターミナルまたはシェルを追加**
   - 「Accessibility」セクションで、ロック マークをクリックしてロックを解除
   - 「+」ボタンをクリック
   - ターミナルアプリケーション（Terminal、iTerm2 など）を選択して追加

   **注**: Rust でコンパイル・実行する場合は、ターミナルを追加してください。

5. **権限の確認**

   権限が正しく設定されていれば、以下のコマンドが正常に実行されます：

   ```bash
   apptidying load
   ```

   権限がない場合は、エラーダイアログが表示されます。

### 設定ファイルの配置

App Tidying は、以下の場所に設定ファイルを配置します：

```
~/Library/Application Support/biz.nosetech.apptidying/
├── settings.json          # アプリケーション設定
├── layout.json            # ウィンドウレイアウト定義
└── apptidying.log         # ログファイル（自動生成）
```

初回実行時に、デフォルトの `settings.json` が自動生成されます。

## 基本的な使い方

### ウィンドウ配置を復元

```bash
# デフォルトレイアウトを復元
apptidying load

# 指定ファイルを使用
apptidying load /path/to/layout.json
```

### ウィンドウ配置を保存

```bash
# デフォルト設定に保存
apptidying save

# 指定ファイルに保存
apptidying save /path/to/layout.json

# ターミナルウィンドウを含めて保存
apptidying save --own

# ターミナルウィンドウを含めて、指定ファイルに保存
apptidying save --own /path/to/layout.json
```

### グローバルオプション

```bash
# デバッグ出力を有効化
apptidying -v load

# ヘルプを表示
apptidying -h

# バージョンを表示
apptidying -V
```

## 設定ファイル

### settings.json（アプリケーション設定）

```json
{
  "version": "1.0",
  "timeout": 3000,
  "notification": {
    "info": "notification",
    "warn": "notification",
    "error": "dialog"
  },
  "log_rotation": {
    "rotation_type": "size",
    "max_size_mb": 10,
    "max_files": 5
  }
}
```

**フィールド説明**:

- `version`: バージョン情報（サポート: `1.0`）
- `timeout`: アプリケーション起動待機時間（ミリ秒、デフォルト: 3000）
- `notification`: 通知設定
  - `info`: INFO レベルの通知方式（`notification`, `dialog`, `none`）
  - `warn`: WARN レベルの通知方式（`notification`, `dialog`, `none`）
  - `error`: ERROR レベルの通知方式（`notification`, `dialog`, `none`）
- `log_rotation`: ログローテーション設定
  - `rotation_type`: ローテーション方式（`size` のみサポート）
  - `max_size_mb`: 最大ファイルサイズ（MB 単位）
  - `max_files`: 保持する世代数

### layout.json（ウィンドウレイアウト定義）

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

詳細な設定方法は [SETTINGS_GUIDE.md](docs/SETTINGS_GUIDE.md) を参照してください。

## よくある質問（FAQ）

### Q. 「Accessibility API へのアクセス許可がない」というエラーが出ました

**A.** 初回セットアップの「Accessibility API 権限の設定」セクションを参照し、ターミナルアプリケーションに権限を付与してください。

### Q. ウィンドウが移動しません

**A.** 以下をご確認ください：

1. Accessibility API の権限が正しく設定されているか
2. `apptidying -v load` でデバッグ出力を確認
3. ログファイルを確認（`~/Library/Application Support/biz.nosetech.apptidying/apptidying.log`）

### Q. 最小化されたウィンドウはどうなりますか？

**A.** 最小化されたウィンドウは以下のように処理されます：

- **`save` コマンド**: 最小化されたウィンドウは保存対象外です（INFO メッセージで通知）
- **`load` コマンド**: 保存したウィンドウ配置を復元時に、最小化されたウィンドウは復元されません

### Q. ターミナルウィンドウも保存したいです

**A.** `--own` オプションを使用してください：

```bash
apptidying save --own
```

このオプションで、実行中のターミナルウィンドウも含めて保存できます。

### Q. 複数のディスプレイを使用しています

**A.** App Tidying はマルチディスプレイに対応しており、各ディスプレイのウィンドウ配置を個別に保存・復元できます。詳細は [SETTINGS_GUIDE.md](docs/SETTINGS_GUIDE.md) の「複数ディスプレイの例」を参照してください。

## トラブルシューティング

### ログファイルの確認

ウィンドウが予期したように配置されない場合は、ログファイルを確認してください：

```bash
# ログファイルを表示
cat ~/Library/Application\ Support/biz.nosetech.apptidying/apptidying.log

# リアルタイムでログを監視
tail -f ~/Library/Application\ Support/biz.nosetech.apptidying/apptidying.log
```

### デバッグ出力の有効化

詳細なデバッグ情報が必要な場合は、`-v` オプションを使用してください：

```bash
apptidying -v load
apptidying -v save
```

デバッグ出力はターミナルに表示され、ログファイルにも記録されます。

### ウィンドウが見つからない場合

- 指定したアプリケーションが起動しているか確認
- ウィンドウタイトルが正確か確認（設定ファイルのタイトルと一致する必要があります）
- ウィンドウが最小化されていないか確認

## ライセンス

このプロジェクトは MIT ライセンスの下で公開されています。詳細は [LICENSE](LICENSE) を参照してください。

## 開発者向け情報

### リポジトリ構造

```
app-tidying/
├── src/                          # Rust ソースコード
├── tests/                        # テストコード
├── docs/                         # ドキュメント
├── CLAUDE.md                     # 開発仕様書
├── README.md                     # このファイル
└── Cargo.toml                    # Rust パッケージ定義
```

### テスト実行

```bash
# 標準テスト実行（CI推奨）
cargo test

# すべてのテストを実行（#[ignore] テストも含む）
cargo test -- --ignored

# テスト出力を表示
cargo test -- --nocapture
```

詳細は [CLAUDE.md](CLAUDE.md) の「テスト・品質保証」セクションを参照してください。

### API ドキュメント

Rust API ドキュメントの生成：

```bash
# ドキュメントを生成
cargo doc --no-deps

# ブラウザで確認
cargo doc --no-deps --open
```

### コントリビューション

バグ報告や機能要望、コントリビューションを歓迎します。詳細な開発ルールは [CLAUDE.md](CLAUDE.md) を参照してください。

## サポート

問題が発生した場合は、以下をお試しください：

1. この README と [SETTINGS_GUIDE.md](docs/SETTINGS_GUIDE.md) を確認
2. ログファイルを確認（`~/Library/Application Support/biz.nosetech.apptidying/apptidying.log`）
3. `apptidying -v load` でデバッグ出力を確認
4. GitHub リポジトリで Issue を報告
