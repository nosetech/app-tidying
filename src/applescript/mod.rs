//! AppleScript モジュール
//!
//! macOS のウィンドウ操作、アプリケーション管理、ディスプレイ情報取得を
//! AppleScript/JXA を通じて実現するモジュール。
//!
//! # 公開 API について
//!
//! このモジュールは以下の公開 API を提供します：
//!
//! - アプリケーション操作: `launch_or_activate_app()`, `get_running_applications()`
//! - ウィンドウ操作: `get_all_windows()`, `resize_window()`, `create_new_window()`
//! - ディスプレイ情報: `get_all_connected_displays()`, `get_display_info()`
//! - ユーティリティ: `escape_applescript_string()`, `parse_window_list()` 他
//!
//! これらの API は library ユーザーおよびテストコードから使用されることを想定しています。
//! 一部の型（例: `DisplayInfo`, `WindowInfo` など）のメソッド（`to_json()` など）は
//! JSON 形式への変換を提供し、テストやデバッグで活用されます。

pub mod app;
pub mod display;
pub mod osascript;
pub mod utils;
pub mod window;

// 以下の pub use は、外部ユーザーおよびテストコードから使用される公開 API です。
// binary では使用されていないものも含まれていますが、library として公開 API を提供するために
// 再エクスポート（re-export）しています。
#[allow(unused_imports)]
pub use app::{
    get_running_applications, launch_or_activate_app, AppInfo, AppLaunchError, AppLaunchResult,
    RunningAppsError,
};
#[allow(unused_imports)]
pub use display::{get_all_connected_displays, get_display_info, DisplayInfo};
#[allow(unused_imports)]
pub use utils::{
    classify_window, escape_applescript_string, is_excluded_window, is_system_app,
    parse_single_window, parse_window_list, WindowType,
};
#[allow(unused_imports)]
pub use window::{
    create_new_window, get_all_windows, resize_window, WindowInfo, WindowInfoError,
    WindowResizeError, WindowResizeResult,
};
