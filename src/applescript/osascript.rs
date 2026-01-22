//! osascript 実行基盤
//!
//! AppleScript と JXA を実行するための基盤関数を提供します。
//! このモジュールは低レベルの `osascript` コマンド実行を行い、
//! 他の高レベルモジュール（app, window, display など）で使用されます。

use std::process::Command;

use crate::applescript::app::AppLaunchError;

/// osascript コマンドを実行する基盤関数
///
/// AppleScript スクリプトを `osascript -e` で実行します。
/// このモジュール内のみで使用される内部関数です。
///
/// # Arguments
/// * `script` - 実行する AppleScript コード
///
/// # Returns
/// * `Ok(std::process::Output)` - 実行結果
/// * `Err(AppLaunchError)` - osascript の実行に失敗
///
/// # 内部使用
/// このモジュール内部でのみ `pub(super)` で公開されています。
pub(super) fn run_osascript(script: &str) -> Result<std::process::Output, AppLaunchError> {
    Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| AppLaunchError {
            message: format!("osascriptの実行に失敗しました: {}", e),
        })
}

/// JXA (JavaScript for Automation) スクリプトを実行する基盤関数
///
/// JXA スクリプトを `osascript -l JavaScript -e` で実行します。
/// このモジュール内のみで使用される内部関数です。
///
/// # Arguments
/// * `script` - 実行する JXA コード
///
/// # Returns
/// * `Ok(std::process::Output)` - 実行結果
/// * `Err(AppLaunchError)` - osascript の実行に失敗
///
/// # 内部使用
/// このモジュール内部でのみ `pub(super)` で公開されています。
pub(super) fn run_jxa(script: &str) -> Result<std::process::Output, AppLaunchError> {
    Command::new("osascript")
        .arg("-l")
        .arg("JavaScript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| AppLaunchError {
            message: format!("osascript (JXA) の実行に失敗しました: {}", e),
        })
}
