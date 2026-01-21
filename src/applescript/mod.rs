//! AppleScript モジュール
//!
//! macOS のウィンドウ操作、アプリケーション管理、ディスプレイ情報取得を
//! AppleScript/JXA を通じて実現するモジュール。

pub mod app;
pub mod display;
pub mod osascript;
pub mod utils;
pub mod window;

pub use app::{get_running_applications, launch_or_activate_app};
pub use display::{get_all_connected_displays, get_display_info, DisplayInfo};
pub use utils::{escape_applescript_string, is_excluded_window};
pub use window::{
    create_new_window, find_window_by_title, get_all_windows, resize_window, WindowInfo,
};
