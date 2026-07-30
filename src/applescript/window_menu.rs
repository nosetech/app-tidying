//! ウィンドウメニュー操作（OS標準タイリング・ディスプレイ移動）
//!
//! macOSアプリケーション標準メニューバーの「ウインドウ」メニュー（表記は
//! アプリによって「ウインドウ」/「ウィンドウ」/`"Window"` と揺れる）にある
//! 「移動とサイズ変更」サブメニュー、「画面全体に表示」、「\[ディスプレイ名\]に移動」を
//! `System Events` 経由でクリックして実行する。
//!
//! Issue #116（`technical-verification/verify_window_menu_control.sh`）で
//! 実機検証済みのAppleScriptロジックをRust側に移植したもの。
//!
//! # フォールバックについて（Issue #121/#122）
//!
//! layout.json の `tiling` フィールドと `position`/`size` は相互排他（同時指定不可）
//! であるため、本モジュールの関数の呼び出し元（`loader::process_window`）には
//! フォールバック先となる絶対座標指定が存在しない。そのため、以下の関数の呼び出しが
//! 失敗した場合、直接プロパティ設定へのフォールバックは行わず、そのウィンドウの
//! 配置は失敗として扱われる（部分失敗としてWARN通知され、他のウィンドウの処理は
//! 継続する。詳細はCLAUDE.md「タイリング指定」の「メニュー操作が失敗した場合の
//! 扱い」を参照）。
//!
//! 唯一の例外は [`move_window_to_display_via_menu`] で、ウィンドウが既に対象
//! ディスプレイ上にありメニュー項目自体が存在しない場合の失敗であり、これは
//! 異常ではないため呼び出し元で無視される（後続の [`tile_window_via_menu`] は
//! そのまま実行される）。
//!
//! # 既知の制約
//!
//! - **Finderの不安定な挙動**: Issue #116 の検証で、Finderは「\[ディスプレイ名\]に移動」
//!   メニュー項目の表示状態が他アプリと異なる不安定な挙動を示すことを確認している。
//!   本モジュールの関数はFinderを特別扱いしていない。
//! - **並列実行時の `activate` 競合**: 本モジュールの関数は対象アプリを
//!   `activate`（前面化）してからメニュー操作を行う。`loader::load_layout()` は
//!   同一ディスプレイ内の複数ウィンドウを rayon で並列処理するため、異なるアプリを
//!   対象とする複数スレッドがほぼ同時に `activate` を実行すると、意図しないアプリが
//!   前面化された状態でメニュー探索が行われる可能性がある（Issue #122のコードレビュー
//!   で指摘）。`tiling` 指定時はフォールバックが存在せずこの競合がそのまま操作失敗に
//!   つながるため、`run_menu_action_script` 内で `MENU_ACTION_LOCK` により
//!   本モジュールが実行する `activate` からメニュークリックまでの一連の操作を
//!   プロセス全体で直列化し、競合を防止している。

use crate::applescript::osascript::run_osascript;
use crate::applescript::utils::escape_applescript_string;
use crate::config::TileKeyword;
use std::sync::{Mutex, OnceLock};

/// ウィンドウメニュー操作（`activate`を伴う）の排他制御用 Mutex
///
/// `activate` はシステム全体のフォアグラウンドアプリを切り替えるため、複数スレッドが
/// 異なるアプリに対して同時にメニュー操作を行うと、意図しないアプリが前面化された
/// 状態でメニュー探索が行われる可能性がある。`loader::load_layout()` が同一
/// ディスプレイ内の複数ウィンドウを rayon で並列処理するため、
/// [`run_menu_action_script`] 内で本モジュールの `activate` からメニュークリック
/// までの一連の操作をプロセス全体で直列化する（Issue #122のコードレビューで指摘）。
static MENU_ACTION_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn get_menu_action_lock() -> &'static Mutex<()> {
    MENU_ACTION_LOCK.get_or_init(|| Mutex::new(()))
}

/// [`move_window_to_display_via_menu`] が「ディスプレイ移動メニュー項目が
/// 見つかりません」で失敗した場合に返すメッセージ
///
/// このメッセージは、ウィンドウが既に対象ディスプレイ上にあるためメニュー項目自体が
/// 表示されない、という想定内のケースを示す。[`WindowMenuError::is_display_menu_item_not_found`]
/// で判定できる（Issue #122のコードレビューで指摘。詳細は
/// [`move_window_to_display_via_menu`] のドキュメントを参照）。
const DISPLAY_MENU_ITEM_NOT_FOUND_MESSAGE: &str = "ディスプレイ移動メニュー項目が見つかりません";

/// ウィンドウメニュー操作エラー
#[derive(Debug)]
pub struct WindowMenuError {
    pub message: String,
}

impl std::fmt::Display for WindowMenuError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for WindowMenuError {}

impl WindowMenuError {
    /// [`move_window_to_display_via_menu`] が「ディスプレイ移動メニュー項目が
    /// 見つからない」ことを理由に失敗したかどうかを判定する
    ///
    /// このケースは、ウィンドウが既に対象ディスプレイ上にあるために発生する想定内の
    /// 失敗であり、異常とはみなさない（呼び出し元の `loader::process_window` は
    /// この場合は無視して後続の [`tile_window_via_menu`] を実行する）。
    ///
    /// これが `false` を返す場合（「ウインドウ」メニュー自体が見つからない、
    /// Accessibility API の権限がない等）は、想定外の異常な失敗であるため、
    /// 呼び出し元は WARN レベルでログ出力するなど、区別して扱うべきである
    /// （Issue #122のコードレビューで指摘）。
    pub fn is_display_menu_item_not_found(&self) -> bool {
        self.message.contains(DISPLAY_MENU_ITEM_NOT_FOUND_MESSAGE)
    }
}

/// 「ウインドウ」メニューを探索するAppleScript共通部分
///
/// メニュー名の表記揺れ（「ウインドウ」/「ウィンドウ」/`"Window"`）に対応する。
const FIND_WINDOW_MENU_SCRIPT: &str = r#"
      set menubar to menu bar 1
      set allMenus to (every menu of menubar)
      set winMenu to missing value
      repeat with m in allMenus
        set n to (name of m)
        if n is "ウインドウ" or n is "ウィンドウ" or n is "Window" then
          set winMenu to m
          exit repeat
        end if
      end repeat

      if winMenu is missing value then
        return "Error: ウインドウメニューが見つかりません"
      end if
"#;

/// `name is "A" or name is "B" or ...` 形式のAppleScript条件式を組み立てる
///
/// メニュー項目名の表記揺れ（日本語/英語）に対応するため、複数の候補文字列を
/// OR条件で結合する。
fn build_name_match_condition(var_name: &str, candidates: &[&str]) -> String {
    candidates
        .iter()
        .map(|c| format!(r#"{} is "{}""#, var_name, escape_applescript_string(c)))
        .collect::<Vec<_>>()
        .join(" or ")
}

/// `osascript` を実行し、`"Success"` / それ以外 の結果を `Result` に変換する
///
/// メニュー操作系スクリプトはいずれも成功時に `"Success"` を返す規約になっている。
///
/// スクリプト内で対象アプリを `activate`（前面化）してからメニュー操作を行うため、
/// [`MENU_ACTION_LOCK`] により実行全体をプロセス全体で直列化し、並列実行時の
/// `activate` 競合を防止する（Issue #122のコードレビューで指摘）。
fn run_menu_action_script(script: &str, failure_prefix: &str) -> Result<(), WindowMenuError> {
    let _guard = get_menu_action_lock().lock().map_err(|e| WindowMenuError {
        message: format!("メニュー操作ロックの取得に失敗しました: {}", e),
    })?;

    let output = run_osascript(script).map_err(|e| WindowMenuError { message: e.message })?;

    if !output.status.success() {
        return Err(WindowMenuError {
            message: format!(
                "{}: {}",
                failure_prefix,
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str == "Success" {
        Ok(())
    } else if result_str.is_empty() {
        Err(WindowMenuError {
            message: format!(
                "{}: {}",
                failure_prefix,
                String::from_utf8_lossy(&output.stderr)
            ),
        })
    } else {
        Err(WindowMenuError {
            message: result_str,
        })
    }
}

/// OS標準タイリング機能でウィンドウを配置する
///
/// 「ウインドウ」メニューの「移動とサイズ変更」サブメニュー項目
/// （左/右/上/下/左上/右上/左下/右下）、または「画面全体に表示」
/// （`TileKeyword::FullScreen` の場合。こちらはサブメニューを経由しない）を
/// `System Events` 経由でクリックする。
///
/// メニュー項目の有無のみで成否を判定するため、クリック対象のウィンドウが
/// 実際に存在しない場合でも `Ok(())` を返す可能性がある点に注意（メニュー自体は
/// ウィンドウが無くてもアクセス可能な場合があるため）。
///
/// # Arguments
/// * `app_name` - アプリケーション名（例: `"Safari"`, `"Google Chrome"`）
/// * `tile_keyword` - 配置したいタイリングパターン
///
/// # Returns
/// * `Ok(())` - メニュー操作に成功
/// * `Err(WindowMenuError)` - メニューが見つからない、クリックに失敗した等
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::tile_window_via_menu;
/// use apptidying::config::TileKeyword;
///
/// tile_window_via_menu("Safari", &TileKeyword::Left)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn tile_window_via_menu(
    app_name: &str,
    tile_keyword: &TileKeyword,
) -> Result<(), WindowMenuError> {
    let escaped_app_name = escape_applescript_string(app_name);
    let item_condition = build_name_match_condition("n", tile_keyword.menu_item_name_candidates());

    // 「移動とサイズ変更」サブメニュー経由か、「ウインドウ」メニュー直下かで
    // 探索先が異なるため、AppleScript側で分岐する
    let item_lookup_script = if tile_keyword.is_submenu_item() {
        format!(
            r#"
      set mrItem to missing value
      repeat with mi in (every menu item of winMenu)
        set n to (name of mi)
        if n is "移動とサイズ変更" or n is "Move & Resize" then
          set mrItem to mi
          exit repeat
        end if
      end repeat

      if mrItem is missing value then
        return "Error: 移動とサイズ変更メニューが見つかりません"
      end if

      set subMenu to menu 1 of mrItem
      set targetItem to missing value
      repeat with mi in (every menu item of subMenu)
        set n to (name of mi)
        if {condition} then
          set targetItem to mi
          exit repeat
        end if
      end repeat
"#,
            condition = item_condition
        )
    } else {
        format!(
            r#"
      set targetItem to missing value
      repeat with mi in (every menu item of winMenu)
        set n to (name of mi)
        if {condition} then
          set targetItem to mi
          exit repeat
        end if
      end repeat
"#,
            condition = item_condition
        )
    };

    let script = format!(
        r#"
tell application "{app}" to activate
delay 0.3
tell application "System Events"
  tell process "{app}"
    try
{find_menu}
{item_lookup}
      if targetItem is missing value then
        return "Error: メニュー項目が見つかりません: {item}"
      end if

      click targetItem
      return "Success"
    on error err_msg
      return "Error: " & err_msg
    end try
  end tell
end tell
"#,
        app = escaped_app_name,
        find_menu = FIND_WINDOW_MENU_SCRIPT,
        item_lookup = item_lookup_script,
        item = escape_applescript_string(tile_keyword.menu_item_name())
    );

    run_menu_action_script(&script, "OS標準タイリング操作に失敗しました")
}

/// ウィンドウを指定したディスプレイへメニュー操作で移動する
///
/// 「ウインドウ」メニュー直下の「\[ディスプレイ名\]に移動」メニュー項目を
/// `System Events` 経由でクリックする。このメニュー項目は、ウィンドウが
/// 現在表示されていないディスプレイに対してのみ表示されるため（Issue #116
/// で実機確認済み）、既にウィンドウが対象ディスプレイ上にある場合は
/// メニュー項目が見つからず `Err` を返す（[`WindowMenuError::is_display_menu_item_not_found`]
/// が `true` を返す）。これは異常な失敗ではなく想定内のケースであるため、呼び出し元
/// （`loader::process_window`）はこの場合に限り `Err` を無視し、後続の
/// [`tile_window_via_menu`] をそのまま実行する（Issue #121/#122。`tiling` 指定時は
/// `position`/`size` によるフォールバックが存在しないため、本関数の失敗自体を
/// 許容できるのはこのケースのみである点に注意）。
///
/// 一方、`is_display_menu_item_not_found()` が `false` を返す場合（「ウインドウ」
/// メニュー自体が見つからない、Accessibility API の権限がない等）は想定外の異常な
/// 失敗であるため、呼び出し元はこれを区別してWARNレベルでログ出力する
/// （Issue #122のコードレビューで指摘）。
///
/// # Arguments
/// * `app_name` - アプリケーション名
/// * `display_name` - 移動先のディスプレイ名（`get_all_connected_displays()` で取得した名前）
///
/// # Returns
/// * `Ok(())` - メニュー操作に成功
/// * `Err(WindowMenuError)` - メニュー項目が見つからない、クリックに失敗した等
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::move_window_to_display_via_menu;
///
/// move_window_to_display_via_menu("Safari", "Built-in")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn move_window_to_display_via_menu(
    app_name: &str,
    display_name: &str,
) -> Result<(), WindowMenuError> {
    let escaped_app_name = escape_applescript_string(app_name);
    // ディスプレイ移動メニューの文言は言語設定により異なる
    // （日本語: 「{ディスプレイ名}に移動」、英語: "Move to {ディスプレイ名}"）
    let target_candidates = [
        format!("{}に移動", display_name),
        format!("Move to {}", display_name),
    ];
    let target_condition = build_name_match_condition(
        "n",
        &target_candidates
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>(),
    );

    let script = format!(
        r#"
tell application "{app}" to activate
delay 0.3
tell application "System Events"
  tell process "{app}"
    try
{find_menu}
      set targetItem to missing value
      repeat with mi in (every menu item of winMenu)
        set n to (name of mi)
        if {condition} then
          set targetItem to mi
          exit repeat
        end if
      end repeat

      if targetItem is missing value then
        return "Error: {not_found_message}"
      end if

      click targetItem
      return "Success"
    on error err_msg
      return "Error: " & err_msg
    end try
  end tell
end tell
"#,
        app = escaped_app_name,
        find_menu = FIND_WINDOW_MENU_SCRIPT,
        condition = target_condition,
        not_found_message = DISPLAY_MENU_ITEM_NOT_FOUND_MESSAGE
    );

    run_menu_action_script(&script, "ディスプレイ移動メニュー操作に失敗しました")
}
