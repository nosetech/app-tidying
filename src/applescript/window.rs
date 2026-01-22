//! ウィンドウ操作
//!
//! ウィンドウの情報取得、リサイズ、移動、新規作成などを行います。

use serde_json::{json, Value};

use crate::applescript::osascript::run_osascript;
use crate::applescript::utils::{
    escape_applescript_string, parse_single_window, parse_window_list,
};

/// ウィンドウ情報
#[derive(Debug, Clone)]
pub struct WindowInfo {
    /// ウィンドウタイトル
    pub title: String,
    /// ウィンドウ位置（x, y）
    pub position: (i32, i32),
    /// ウィンドウサイズ（幅, 高さ）
    pub size: (i32, i32),
    /// 最小化状態
    pub minimized: bool,
    /// 表示状態
    pub visible: bool,
}

impl WindowInfo {
    /// JSON オブジェクトに変換
    #[allow(dead_code)]
    pub fn to_json(&self) -> Value {
        json!({
            "title": self.title,
            "position": {
                "x": self.position.0,
                "y": self.position.1
            },
            "size": {
                "width": self.size.0,
                "height": self.size.1
            },
            "minimized": self.minimized,
            "visible": self.visible
        })
    }
}

/// ウィンドウ情報取得エラー
#[derive(Debug)]
pub struct WindowInfoError {
    pub message: String,
}

impl std::fmt::Display for WindowInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for WindowInfoError {}

/// ウィンドウリサイズ結果
#[derive(Debug, Clone)]
pub struct WindowResizeResult {
    /// 実行結果のステータス
    pub status: String,
    /// 結果メッセージ
    pub message: String,
    /// 新しいウィンドウ位置（x, y）
    pub new_position: Option<(i32, i32)>,
    /// 新しいウィンドウサイズ（幅, 高さ）
    pub new_size: Option<(i32, i32)>,
}

impl WindowResizeResult {
    /// JSON オブジェクトに変換
    #[allow(dead_code)]
    pub fn to_json(&self) -> Value {
        let mut obj = json!({
            "status": self.status,
            "message": self.message,
        });

        if let Some((x, y)) = self.new_position {
            obj["new_position"] = json!({"x": x, "y": y});
        }

        if let Some((width, height)) = self.new_size {
            obj["new_size"] = json!({"width": width, "height": height});
        }

        obj
    }
}

/// ウィンドウリサイズエラー
#[derive(Debug)]
pub struct WindowResizeError {
    pub message: String,
}

impl std::fmt::Display for WindowResizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for WindowResizeError {}

/// アプリケーションのすべてのウィンドウを取得
///
/// 指定されたアプリケーションのすべてのウィンドウ情報を取得します。
///
/// # Arguments
/// * `app_name` - アプリケーション名
///
/// # Returns
/// * `Ok(Vec<WindowInfo>)` - ウィンドウ情報のベクトル
/// * `Err(WindowInfoError)` - 失敗
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::get_all_windows;
///
/// let windows = get_all_windows("Safari")?;
/// for window in windows {
///     println!("Title: {}", window.title);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn get_all_windows(app_name: &str) -> Result<Vec<WindowInfo>, WindowInfoError> {
    let script = format!(
        r#"
tell application "System Events"
    tell process "{}"
        try
            set windowList to every window
            set windowDataList to {{}}

            repeat with win in windowList
                try
                    set winTitle to title of win
                    set winPos to position of win
                    set winSize to size of win

                    try
                        set winMinimized to miniaturized of win
                    on error
                        set winMinimized to false
                    end try

                    try
                        set winVisible to visible of win
                    on error
                        set winVisible to true
                    end try

                    set windowData to winTitle & "|" & (item 1 of winPos) & "," & (item 2 of winPos) & "|" & (item 1 of winSize) & "," & (item 2 of winSize) & "|" & winMinimized & "|" & winVisible
                    set end of windowDataList to windowData
                on error
                    -- ウィンドウをスキップ
                end try
            end repeat

            return windowDataList
        on error errMsg
            return "error: " & errMsg
        end try
    end tell
end tell
"#,
        escape_applescript_string(app_name)
    );

    let output = run_osascript(&script).map_err(|e| WindowInfoError { message: e.message })?;

    if !output.status.success() {
        return Err(WindowInfoError {
            message: format!(
                "ウィンドウ一覧の取得に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // エラーをチェック
    if result_str.starts_with("error:") {
        return Err(WindowInfoError {
            message: result_str,
        });
    }

    // 結果をパース
    parse_window_list(&result_str)
}

/// 特定のウィンドウ情報を取得
///
/// 指定されたアプリケーション内で、タイトルで特定のウィンドウ情報を取得します。
/// window_title が None の場合は最初のウィンドウを取得します。
///
/// # Arguments
/// * `app_name` - アプリケーション名
/// * `window_title` - ウィンドウタイトル（オプション）
///
/// # Returns
/// * `Ok(WindowInfo)` - ウィンドウ情報
/// * `Err(WindowInfoError)` - ウィンドウが見つからない等のエラー
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::get_window_info;
///
/// let window = get_window_info("Safari", Some("Development"))?;
/// println!("Size: {}x{}", window.size.0, window.size.1);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[allow(dead_code)]
pub fn get_window_info(
    app_name: &str,
    window_title: Option<&str>,
) -> Result<WindowInfo, WindowInfoError> {
    let mut script = format!(
        r#"
tell application "System Events"
    tell process "{}"
        try
"#,
        escape_applescript_string(app_name)
    );

    // タイトルでウィンドウを選択、またはアプリケーションの最初のウィンドウを使用
    if let Some(title) = window_title {
        script.push_str(&format!(
            r#"
            set targetWindow to first window whose name contains "{}"
"#,
            escape_applescript_string(title)
        ));
    } else {
        script.push_str(
            r#"
            set targetWindow to window 1
"#,
        );
    }

    script.push_str(
        r#"
            set winPos to position of targetWindow
            set winSize to size of targetWindow
            set winTitle to title of targetWindow

            try
                set winMinimized to miniaturized of targetWindow
            on error
                set winMinimized to false
            end try

            try
                set winVisible to visible of targetWindow
            on error
                set winVisible to true
            end try

            return winTitle & "|" & (item 1 of winPos) & "," & (item 2 of winPos) & "|" & (item 1 of winSize) & "," & (item 2 of winSize) & "|" & winMinimized & "|" & winVisible
        on error errMsg
            return "error: " & errMsg
        end try
    end tell
end tell
"#,
    );

    let output = run_osascript(&script).map_err(|e| WindowInfoError { message: e.message })?;

    if !output.status.success() {
        return Err(WindowInfoError {
            message: format!(
                "ウィンドウ情報の取得に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str.starts_with("error:") {
        return Err(WindowInfoError {
            message: result_str,
        });
    }

    // parse_single_window を再利用（一貫性のあるパース）
    parse_single_window(&result_str)
}

/// タイトルでウィンドウを検索
///
/// 指定されたタイトルを含むウィンドウを検索します。
/// **複数マッチした場合**は、`get_all_windows()` が返す順序の最初のウィンドウを返します。
/// （通常は最前面のウィンドウですが、AppleScript の実装に依存します）
///
/// # Arguments
/// * `app_name` - アプリケーション名
/// * `window_title` - 検索するウィンドウタイトル（部分一致）
///
/// # Returns
/// * `Ok(Some(WindowInfo))` - ウィンドウが見つかった
/// * `Ok(None)` - ウィンドウが見つからなかった
/// * `Err(WindowInfoError)` - AppleScript 実行エラー
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::find_window_by_title;
///
/// let result = find_window_by_title("Safari", "Development")?;
/// if let Some(window) = result {
///     println!("ウィンドウが見つかりました: {:?}", window);
/// } else {
///     println!("ウィンドウが見つかりません");
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn find_window_by_title(
    app_name: &str,
    window_title: &str,
) -> Result<Option<WindowInfo>, WindowInfoError> {
    // すべてのウィンドウを取得
    let windows = get_all_windows(app_name)?;

    // タイトルで検索（部分一致）
    for window in windows {
        if window.title.contains(window_title) {
            return Ok(Some(window));
        }
    }

    // 見つからなかった
    Ok(None)
}

/// ウィンドウをリサイズ・移動
///
/// 指定されたウィンドウをリサイズおよび移動します。
///
/// # Arguments
/// * `app_name` - アプリケーション名
/// * `window_title` - ウィンドウタイトル（オプション）
/// * `position` - 新しい位置（x, y）（オプション）
/// * `size` - 新しいサイズ（幅, 高さ）（オプション）
///
/// # Returns
/// * `Ok(WindowResizeResult)` - リサイズ成功
/// * `Err(WindowResizeError)` - 失敗
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::resize_window;
///
/// let result = resize_window("Safari", None, Some((0, 0)), Some((1440, 900)))?;
/// println!("Resized: {}", result.message);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn resize_window(
    app_name: &str,
    window_title: Option<&str>,
    position: Option<(i32, i32)>,
    size: Option<(i32, i32)>,
) -> Result<WindowResizeResult, WindowResizeError> {
    // ウィンドウをリサイズするための AppleScript を構築
    let mut script = format!(
        r#"
tell application "System Events"
    try
        tell process "{}"
"#,
        escape_applescript_string(app_name)
    );

    // タイトルでウィンドウを選択、またはアプリケーションの最初のウィンドウを使用
    if let Some(title) = window_title {
        script.push_str(&format!(
            r#"
            set targetWindow to first window whose name contains "{}"
"#,
            escape_applescript_string(title)
        ));
    } else {
        script.push_str(
            r#"
            set targetWindow to window 1
"#,
        );
    }

    // 位置を設定（指定されている場合）
    if let Some((x, y)) = position {
        script.push_str(&format!(
            r#"
            set position of targetWindow to {{{}, {}}}
"#,
            x, y
        ));
    }

    // サイズを設定（指定されている場合）
    if let Some((width, height)) = size {
        script.push_str(&format!(
            r#"
            set size of targetWindow to {{{}, {}}}
"#,
            width, height
        ));
    }

    script.push_str(
        r#"
        end tell
        return "success"
    on error errMsg
        return "error: " & errMsg
    end try
end tell
"#,
    );

    let output = run_osascript(&script).map_err(|e| WindowResizeError { message: e.message })?;

    if !output.status.success() {
        return Err(WindowResizeError {
            message: format!(
                "ウィンドウのリサイズに失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str.contains("error") {
        return Err(WindowResizeError {
            message: result_str,
        });
    }

    Ok(WindowResizeResult {
        status: "success".to_string(),
        message: "ウィンドウをリサイズしました".to_string(),
        new_position: position,
        new_size: size,
    })
}

/// 新規ウィンドウを作成
///
/// AppleScript でアプリケーションの File メニューから
/// 「New Window」「新規ウィンドウ」メニューアイテムを検索して実行します。
///
/// # 注意
/// この関数はメニューをクリックするだけで、ウィンドウが実際に開くまで待機しません。
/// 新規ウィンドウの作成完了を確認する必要がある場合は、呼び出し側で以下のような処理を実装してください：
///
/// ```ignore
/// create_new_window("Safari")?;
/// std::thread::sleep(std::time::Duration::from_millis(500));
/// let window = find_window_by_title("Safari", "新しいウィンドウのタイトル")?;
/// ```
///
/// # Arguments
/// * `app_name` - アプリケーション名（例: "Finder", "Safari", "Google Chrome"）
///
/// # Returns
/// * `Ok(())` - 新規ウィンドウ作成に成功
/// * `Err(WindowInfoError)` - 失敗（メニューが見つからない、権限がない等）
///
/// # Examples
/// ```ignore
/// use apptidying::applescript::create_new_window;
///
/// create_new_window("Safari")?;
/// create_new_window("Google Chrome")?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn create_new_window(app_name: &str) -> Result<(), WindowInfoError> {
    let script = format!(
        r#"
tell application "System Events"
    tell process "{}"
        try
            activate

            -- メニューバーを取得
            set menubar to menu bar 1
            set allMenus to (every menu of menubar)
            set fileMenu to {{}}

            -- File または ファイル メニューを検索
            repeat with m in allMenus
                if name of m is "File" or name of m is "ファイル" then
                    set fileMenu to m
                    exit repeat
                end if
            end repeat

            if fileMenu is {{}} then
                return "error: ファイルメニューが見つかりません"
            end if

            -- メニューアイテムを取得
            set menuItems to (every menu item of fileMenu)

            -- 「New Window」「新規ウインドウ」「新規Finderウインドウ」を検索してクリック
            repeat with mi in menuItems
                try
                    set itemName to name of mi
                    -- 英語と日本語の両方に対応
                    if (itemName contains "New Window") or (itemName contains "新規ウインドウ") or (itemName contains "新規Finderウインドウ") then
                        click mi
                        return "success"
                    end if
                end try
            end repeat

            return "error: 新規ウィンドウメニューアイテムが見つかりません"
        on error errMsg
            return "error: " & errMsg
        end try
    end tell
end tell
"#,
        escape_applescript_string(app_name)
    );

    let output = run_osascript(&script).map_err(|e| WindowInfoError { message: e.message })?;

    if !output.status.success() {
        return Err(WindowInfoError {
            message: format!(
                "新規ウィンドウ作成に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let result_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if result_str.starts_with("error:") {
        return Err(WindowInfoError {
            message: result_str,
        });
    }

    Ok(())
}
