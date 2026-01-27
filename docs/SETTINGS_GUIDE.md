# 設定ガイド

App Tidying の設定ファイルについての詳細ガイドです。

## 設定ファイルの概要

App Tidying では、2つのJSON設定ファイルを使用します：

| ファイル名 | 保存先 | 役割 |
|-----------|------|------|
| `settings.json` | `~/Library/Application Support/biz.nosetech.apptidying/` | アプリケーション動作設定 |
| `layout.json` | `~/Library/Application Support/biz.nosetech.apptidying/` | ウィンドウレイアウト定義 |

## settings.json の詳細

### ファイル構造

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

### フィールド説明

#### `version` (必須)

バージョン情報です。現在は `"1.0"` のみサポートされています。

- **型**: 文字列
- **デフォルト**: `"1.0"`
- **例**: `"version": "1.0"`

#### `timeout` (オプション)

アプリケーション起動後の待機時間（ミリ秒）です。アプリケーションが完全に起動するまでの時間を指定します。

- **型**: 整数
- **デフォルト**: `3000`（3秒）
- **範囲**: 100 ～ 60000（100ms ～ 60秒）
- **例**: `"timeout": 5000`

**用途**: Google Chrome など、起動に時間がかかるアプリケーションの場合は、値を大きくしてください。

#### `notification` (オプション)

各ログレベルの通知方式を指定します。

- **型**: オブジェクト
- **デフォルト**:
  ```json
  {
    "info": "notification",
    "warn": "notification",
    "error": "dialog"
  }
  ```

**各フィールド**:

- `info`: INFO レベルメッセージの通知方式
- `warn`: WARN レベルメッセージの通知方式
- `error`: ERROR レベルメッセージの通知方式

**指定可能な値**:

| 値 | 説明 |
|---|---|
| `"notification"` | macOS 通知センターに表示 |
| `"dialog"` | ダイアログボックスで表示 |
| `"none"` | 通知なし（ログファイルには記録される） |

**例**:

```json
{
  "notification": {
    "info": "none",
    "warn": "notification",
    "error": "dialog"
  }
}
```

#### `log_rotation` (オプション)

ログファイルのローテーション設定です。ログファイルが大きくなりすぎるのを防ぎます。

- **型**: オブジェクト
- **デフォルト**:
  ```json
  {
    "rotation_type": "size",
    "max_size_mb": 10,
    "max_files": 5
  }
  ```

**各フィールド**:

- `rotation_type`: ローテーション方式（現在は `"size"` のみサポート）
- `max_size_mb`: ログファイルの最大サイズ（MB 単位）
- `max_files`: 保持する世代数

**例**:

```json
{
  "log_rotation": {
    "rotation_type": "size",
    "max_size_mb": 20,
    "max_files": 10
  }
}
```

この設定では、ログファイルが 20MB に達すると自動的に世代交代し、最大10世代まで保持されます。

### 設定例

#### シンプルな設定

```json
{
  "version": "1.0"
}
```

デフォルト値が適用されます。

#### カスタマイズした設定

```json
{
  "version": "1.0",
  "timeout": 5000,
  "notification": {
    "info": "none",
    "warn": "notification",
    "error": "dialog"
  },
  "log_rotation": {
    "rotation_type": "size",
    "max_size_mb": 20,
    "max_files": 10
  }
}
```

## layout.json の詳細

### ファイル構造

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

### トップレベルフィールド

#### `version` (必須)

バージョン情報です。現在は `"1.0"` のみサポートされています。

- **型**: 文字列
- **値**: `"1.0"`

#### `layouts` (必須)

レイアウト定義の配列です。複数のレイアウトを保存できます。

- **型**: 配列
- **要素**: レイアウトオブジェクト

### レイアウトオブジェクト

```json
{
  "displays": [
    {
      "name": "Built-in",
      "windows": [...]
    }
  ]
}
```

#### `displays` (必須)

ディスプレイ定義の配列です。各ディスプレイのウィンドウ配置を指定します。

### ディスプレイオブジェクト

```json
{
  "name": "Built-in",
  "windows": [...]
}
```

#### `name` (必須)

ディスプレイ名です。接続しているディスプレイの名前を指定します。

- **型**: 文字列
- **例**: `"Built-in"`, `"Enhanced"`, `"外部ディスプレイ"`

**ディスプレイ名の確認方法**:

```bash
# 接続しているディスプレイを確認
system_profiler SPDisplaysDataType
```

#### `windows` (必須)

ウィンドウ定義の配列です。このディスプレイに配置するウィンドウを指定します。

### ウィンドウオブジェクト

```json
{
  "app": "Google Chrome",
  "title": "Development",
  "position": { "x": 0, "y": 0 },
  "size": { "width": 1440, "height": 900 }
}
```

#### `app` (必須)

アプリケーション名です。macOS のアプリケーション名を指定します。

- **型**: 文字列
- **例**: `"Google Chrome"`, `"Safari"`, `"Finder"`, `"Visual Studio Code"`

**アプリケーション名の確認方法**:

App Tidying で `save` コマンドを実行すると、現在のウィンドウ情報がJSON形式で保存され、正確なアプリケーション名が記録されます。

#### `title` (オプション)

ウィンドウタイトルです。同一アプリケーションで複数ウィンドウを操作する場合に指定します。

- **型**: 文字列
- **例**: `"Development"`, `"inbox - Gmail"`, `"Untitled Document"`

**注意**:

- 指定されない場合は、そのアプリケーションの最前面のウィンドウが対象になります
- 複数のウィンドウがマッチした場合は、最前面のウィンドウが選択されます
- 正確なタイトルマッチが必要です

#### `position` (必須)

ウィンドウの位置を指定します。X座標とY座標を含みます。

- **型**: オブジェクト
- **必須フィールド**: `x`, `y`

**値の形式**:

- **文字列（相対指定）**: `"left"`, `"right"`, `"top"`, `"bottom"`
- **数値（絶対指定）**: ピクセル単位の座標

**例**:

```json
{
  "position": { "x": 0, "y": 0 }
}
```

```json
{
  "position": { "x": "left", "y": "top" }
}
```

#### `size` (必須)

ウィンドウのサイズを指定します。幅と高さを含みます。

- **型**: オブジェクト
- **必須フィールド**: `width`, `height`

**値の形式**:

- **文字列（相対指定）**: `"half"` (画面の1/2), `"third"` (画面の1/3), `"max"` (フル幅/高さ)
- **数値（絶対指定）**: ピクセル単位のサイズ

**例**:

```json
{
  "size": { "width": 1440, "height": 900 }
}
```

```json
{
  "size": { "width": "half", "height": "max" }
}
```

## 指定方式の詳細

### ピクセル指定（絶対位置）

ウィンドウを正確なピクセル位置に配置します。

**例**:

```json
{
  "app": "Google Chrome",
  "position": { "x": 0, "y": 25 },
  "size": { "width": 1440, "height": 900 }
}
```

このレイアウトでは、Chrome を画面の左上（X=0, Y=25）に、1440×900 のサイズで配置します。

**注意**:

- Y座標が 25 以上である理由は、macOS のメニューバーの高さです
- 異なるディスプレイでは、座標値が異なります
- ディスプレイの解像度が変わった場合、ウィンドウが画面外になる可能性があります

### パターン指定（相対位置）- 推奨

ディスプレイの解像度に関わらず、相対的な位置でウィンドウを配置します。複数のディスプレイや異なる解像度に対応する場合は、このパターン指定が推奨されます。

**位置の指定可能値**:

| 位置 | 値 | 説明 |
|-----|---|------|
| X 座標 | `"left"` | ディスプレイの左端 |
| X 座標 | `"right"` | ディスプレイの右端 |
| Y 座標 | `"top"` | ディスプレイの上端 |
| Y 座標 | `"bottom"` | ディスプレイの下端 |

**サイズの指定可能値**:

| サイズ | 値 | 説明 |
|------|---|------|
| 幅 | `"half"` | ディスプレイ幅の1/2 |
| 幅 | `"third"` | ディスプレイ幅の1/3 |
| 幅 | `"max"` | ディスプレイ幅と同じ |
| 高さ | `"half"` | ディスプレイ高さの1/2 |
| 高さ | `"third"` | ディスプレイ高さの1/3 |
| 高さ | `"max"` | ディスプレイ高さと同じ |

**例**:

```json
{
  "app": "Google Chrome",
  "position": { "x": "left", "y": "top" },
  "size": { "width": "half", "height": "max" }
}
```

このレイアウトでは、Chrome を画面の左側に、ディスプレイの幅の1/2、高さいっぱいで配置します。

### パターン組み合わせ例

```json
{
  "app": "Visual Studio Code",
  "position": { "x": "left", "y": "top" },
  "size": { "width": "two-third", "height": "max" }
}
```

```json
{
  "app": "Safari",
  "position": { "x": "right", "y": "top" },
  "size": { "width": "half", "height": "half" }
}
```

```json
{
  "app": "Mail",
  "position": { "x": "right", "y": "bottom" },
  "size": { "width": "half", "height": "half" }
}
```

## 設定例

### 単一ウィンドウの例

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

### 複数ウィンドウの例

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
              "app": "Visual Studio Code",
              "position": { "x": "left", "y": "top" },
              "size": { "width": "half", "height": "max" }
            },
            {
              "app": "Safari",
              "position": { "x": "right", "y": "top" },
              "size": { "width": "half", "height": "half" }
            },
            {
              "app": "Mail",
              "position": { "x": "right", "y": "bottom" },
              "size": { "width": "half", "height": "half" }
            }
          ]
        }
      ]
    }
  ]
}
```

### 複数ディスプレイの例

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
              "app": "Visual Studio Code",
              "position": { "x": "left", "y": "top" },
              "size": { "width": "max", "height": "max" }
            }
          ]
        },
        {
          "name": "Enhanced",
          "windows": [
            {
              "app": "Google Chrome",
              "position": { "x": "left", "y": "top" },
              "size": { "width": "max", "height": "max" }
            }
          ]
        }
      ]
    }
  ]
}
```

### 同一アプリの複数ウィンドウの例

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
              "title": "Work",
              "position": { "x": "left", "y": "top" },
              "size": { "width": "half", "height": "max" }
            },
            {
              "app": "Google Chrome",
              "title": "Personal",
              "position": { "x": "right", "y": "top" },
              "size": { "width": "half", "height": "max" }
            }
          ]
        }
      ]
    }
  ]
}
```

## エラーケースと対処方法

### エラー: 「無効なサイズ指定」

**原因**: ウィンドウサイズが画面より大きい、または負の座標が指定されている

**対応**: 以下を確認してください：
- ウィンドウサイズがディスプレイの解像度内か
- 座標が負の値になっていないか

**例（エラー）**:

```json
{
  "position": { "x": -100, "y": 0 },
  "size": { "width": 3000, "height": 3000 }
}
```

**例（修正）**:

```json
{
  "position": { "x": "left", "y": "top" },
  "size": { "width": "max", "height": "max" }
}
```

### エラー: 「ディスプレイが接続されていない」

**原因**: 定義ファイルのディスプレイが接続されていない

**対応**:
- 正しいディスプレイ名を確認してください
- 接続しているディスプレイに合わせて設定を修正してください
- App Tidying は、接続されているディスプレイのウィンドウ配置のみ実行します

### エラー: 「ウィンドウが見つからない」

**原因**: 指定したアプリケーションが起動していない、またはウィンドウタイトルが不正確

**対応**:
- アプリケーションが起動しているか確認
- ウィンドウタイトルが設定ファイルと一致しているか確認
- `save` コマンドで現在のウィンドウを保存し、生成されたJSONファイルを参照してください

## 技術的制約

### 非対応機能（Ver 1.0）

以下の機能はVer 1.0では対応していません：

- **ステージマネージャ対応**: 表示中のディスプレイのみサポート
- **仮想デスクトップ（Space）操作**: 表示中の Space のみサポート
- **複数 Space の管理**: 実装予定なし

### 最小化・非表示ウィンドウの扱い

- **`save` コマンド**: 最小化されたウィンドウ、非表示のウィンドウは保存対象外です
- **`load` コマンド**: 最小化されたウィンドウは復元されません

## トラブルシューティング

### 設定ファイルが見つからない

```bash
ls ~/Library/Application\ Support/biz.nosetech.apptidying/
```

ファイルが見つらない場合は、`apptidying save` を実行して自動生成させてください。

### JSON ファイルの妥当性確認

JSON ファイルの文法をオンラインで確認できます：

- https://jsonlint.com/

または、コマンドラインで確認：

```bash
python3 -m json.tool ~/Library/Application\ Support/biz.nosetech.apptidying/layout.json
```

### 設定ファイルが無視される

`settings.json` が存在しない場合、デフォルト設定が使用されます。カスタム設定を適用するには、`settings.json` を作成してください。

## 詳細情報

- **ユーザーガイド**: [README.md](../README.md)
- **開発者向け仕様書**: [CLAUDE.md](../CLAUDE.md)
