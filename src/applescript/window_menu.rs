//! ウィンドウメニュー操作（OS標準タイリング・ディスプレイ移動）
//!
//! macOSアプリケーション標準メニューバーの「ウインドウ」メニュー（表記は
//! アプリによって「ウインドウ」/「ウィンドウ」/`"Window"` と揺れる）にある
//! 「移動とサイズ変更」サブメニュー、「画面全体に表示」、「\[ディスプレイ名\]に移動」を
//! `System Events` 経由でクリックして実行する。
//!
//! Issue #116（`technical-verification/verify_window_menu_control.sh`）で
//! 実機検証済みのAppleScriptロジックをRust側に移植したもの。

use crate::applescript::osascript::run_osascript;
use crate::applescript::utils::escape_applescript_string;
use crate::config::TileKeyword;

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

/// OS標準タイリング機能でウィンドウを配置する
///
/// 「ウインドウ」メニューの「移動とサイズ変更」サブメニュー項目
/// （左/右/上/下/左上/右上/左下/右下）、または「画面全体に表示」
/// （`TileKeyword::FullScreen` の場合。こちらはサブメニューを経由しない）を
/// `System Events` 経由でクリックする。
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
    let escaped_item_name = escape_applescript_string(tile_keyword.menu_item_name());

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
        if n is "{item}" then
          set targetItem to mi
          exit repeat
        end if
      end repeat
"#,
            item = escaped_item_name
        )
    } else {
        format!(
            r#"
      set targetItem to missing value
      repeat with mi in (every menu item of winMenu)
        set n to (name of mi)
        if n is "{item}" then
          set targetItem to mi
          exit repeat
        end if
      end repeat
"#,
            item = escaped_item_name
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
        item = escaped_item_name
    );

    let output = run_osascript(&script).map_err(|e| WindowMenuError { message: e.message })?;
    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str == "Success" {
        Ok(())
    } else {
        Err(WindowMenuError {
            message: if result_str.is_empty() {
                format!(
                    "OS標準タイリング操作に失敗しました: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
            } else {
                result_str
            },
        })
    }
}

/// ウィンドウを指定したディスプレイへメニュー操作で移動する
///
/// 「ウインドウ」メニュー直下の「\[ディスプレイ名\]に移動」メニュー項目を
/// `System Events` 経由でクリックする。このメニュー項目は、ウィンドウが
/// 現在表示されていないディスプレイに対してのみ表示されるため（Issue #116
/// で実機確認済み）、既にウィンドウが対象ディスプレイ上にある場合は
/// メニュー項目が見つからず `Err` を返す。呼び出し側はこれを異常とはせず、
/// 既存の絶対座標移動処理へフォールバックすればよい。
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
    let escaped_target_name = escape_applescript_string(&format!("{}に移動", display_name));

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
        if n is "{target}" then
          set targetItem to mi
          exit repeat
        end if
      end repeat

      if targetItem is missing value then
        return "NotFound"
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
        target = escaped_target_name
    );

    let output = run_osascript(&script).map_err(|e| WindowMenuError { message: e.message })?;
    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str == "Success" {
        Ok(())
    } else {
        Err(WindowMenuError {
            message: if result_str.is_empty() {
                format!(
                    "ディスプレイ移動メニュー操作に失敗しました: {}",
                    String::from_utf8_lossy(&output.stderr)
                )
            } else {
                result_str
            },
        })
    }
}
