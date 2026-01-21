//! AppleScript モジュール
//!
//! macOS のウィンドウ操作、アプリケーション管理、ディスプレイ情報取得を
//! AppleScript/JXA を通じて実現するモジュール。

pub mod app;
pub mod display;
pub mod osascript;
pub mod utils;
pub mod window;

pub use app::{
    get_running_applications, launch_or_activate_app, AppInfo, AppLaunchError, AppLaunchResult,
    RunningAppsError,
};
pub use display::{get_all_connected_displays, get_display_info, DisplayInfo};
pub use utils::{
    classify_window, escape_applescript_string, is_excluded_window, is_system_app,
    parse_single_window, parse_window_list,
};
pub use window::{
    create_new_window, find_window_by_title, get_all_windows, get_window_info, resize_window,
    WindowInfo, WindowInfoError, WindowResizeError, WindowResizeResult, WindowType,
};
