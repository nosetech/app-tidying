//! AppleScript モジュール
//!
//! macOS のウィンドウ操作、アプリケーション管理、ディスプレイ情報取得を
//! AppleScript/JXA を通じて実現するモジュール。

pub mod app;
pub mod display;
pub mod osascript;
pub mod utils;
pub mod window;

pub use app::{AppInfo, AppLaunchError, AppLaunchResult, RunningAppsError, get_running_applications, launch_or_activate_app};
pub use window::{WindowInfo, WindowInfoError, WindowResizeError, WindowResizeResult, WindowType, create_new_window, find_window_by_title, get_all_windows, get_window_info, resize_window};
pub use display::{DisplayError, DisplayInfo, get_all_connected_displays, get_display_info};
pub use utils::{classify_window, escape_applescript_string, is_excluded_window, is_system_app, parse_single_window, parse_window_list};
