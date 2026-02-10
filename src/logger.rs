use log::LevelFilter;
use std::cell::RefCell;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Mutex, OnceLock};

/// ユーザー通知のレベル
///
/// `show_notification()` 関数で使用される通知レベルを表します。
///
/// # バリアント
/// * `Info` - 情報通知（デフォルトは通知センター）
/// * `Warn` - 警告通知（デフォルトは通知センター）
/// * `Error` - エラー通知（デフォルトはダイアログ）
pub enum NotificationLevel {
    Info,
    Warn,
    Error,
}

/// ロガーの設定
///
/// ログ出力、通知、ログローテーションの動作を制御する設定を格納します。
pub struct LoggerConfig {
    /// デバッグモード（有効時は DEBUG レベルのログを出力）
    pub debug_mode: bool,
    /// サイレントモード（有効時は標準出力・標準エラーへの出力を抑制）
    pub silent_mode: bool,
    /// 通知設定（settings.json から読み込まれる）
    pub notification_config: Option<crate::config::NotificationConfig>,
    /// ログローテーション設定（settings.json から読み込まれる）
    pub log_rotation_config: Option<crate::config::LogRotationConfig>,
}

// NotificationConfig は config モジュールから再エクスポート
pub use crate::config::NotificationConfig;

thread_local! {
    static LOGGER_CONFIG: RefCell<Option<LoggerConfig>> = const { RefCell::new(None) };
}

/// ログファイルアクセスの排他制御用 Mutex
/// マルチスレッド環境でのローテーション処理とログ書き込みを保護する
static LOG_FILE_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn get_log_file_lock() -> &'static Mutex<()> {
    LOG_FILE_LOCK.get_or_init(|| Mutex::new(()))
}

pub fn get_log_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home = dirs::home_dir().ok_or("ホームディレクトリの取得に失敗しました")?;
    let log_dir = home.join("Library/Application Support/biz.nosetech.apptidying");
    fs::create_dir_all(&log_dir)?;
    Ok(log_dir.join("apptidying.log"))
}

fn is_running_in_terminal() -> bool {
    std::env::var("TERM").is_ok()
}

/// ログファイルから直近のログを読み込む
///
/// ログファイルの最後のN行を読み込み、文字列として返します。
///
/// # Arguments
/// * `lines` - 取得する最大行数（デフォルト: 5）
///
/// # Returns
/// * `Ok(String)` - ログ内容（複数行を改行で結合）
/// * `Err(Box<dyn std::error::Error>)` - ファイル読み込みエラーなど
///
/// # 使用状況
/// 現在この関数は内部で使用されていませんが、将来的にエラーダイアログに
/// ログの直近内容を表示する機能拡張のために実装されています。
///
/// 注: テスト用に公開（将来的な機能拡張で使用する可能性があるため pub として残す）
#[allow(dead_code)]
pub fn read_recent_logs(lines: usize) -> Result<String, Box<dyn std::error::Error>> {
    let path = get_log_file_path()?;

    // ファイルが存在しない場合はスキップ
    if !path.exists() {
        return Ok(String::new());
    }

    let content = fs::read_to_string(&path)?;
    let log_lines: Vec<&str> = content.lines().collect();

    // 最後のN行を取得
    let start_idx = if log_lines.len() > lines {
        log_lines.len() - lines
    } else {
        0
    };

    let recent_logs = log_lines[start_idx..].join("\n");
    Ok(recent_logs)
}

/// ログローテーションが必要かどうかを判定する
fn should_rotate_log(log_path: &std::path::Path) -> std::io::Result<bool> {
    // ファイルが存在しない場合はローテーション不要
    if !log_path.exists() {
        return Ok(false);
    }

    // ファイルサイズをバイト単位で取得
    let metadata = fs::metadata(log_path)?;
    let file_size_bytes = metadata.len();

    // 設定値を取得（デフォルト: 10MB）
    let max_size_mb = get_log_rotation_config()
        .map(|c| c.max_size_mb)
        .unwrap_or(10);

    // バイト単位での正確な比較
    let max_size_bytes = max_size_mb * 1024 * 1024;

    Ok(file_size_bytes >= max_size_bytes)
}

/// ログファイルをローテーションする
fn rotate_log_file(log_path: &std::path::Path) -> std::io::Result<()> {
    // 世代数を取得（デフォルト: 5）
    let max_files = get_log_rotation_config().map(|c| c.max_files).unwrap_or(5);

    let log_dir = log_path.parent().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "ログディレクトリが見つかりません",
        )
    })?;

    // 最古の世代を削除
    let oldest_path = log_dir.join(format!("apptidying.log.{}", max_files - 1));
    if oldest_path.exists() {
        fs::remove_file(&oldest_path)?;
    }

    // 世代をずらす（古い方から）
    for i in (1..max_files).rev() {
        let src = if i == 1 {
            log_dir.join("apptidying.log")
        } else {
            log_dir.join(format!("apptidying.log.{}", i - 1))
        };
        let dst = log_dir.join(format!("apptidying.log.{}", i));

        if src.exists() {
            fs::rename(&src, &dst)?;
        }
    }

    Ok(())
}

/// ログローテーション設定を取得する
fn get_log_rotation_config() -> Option<crate::config::LogRotationConfig> {
    LOGGER_CONFIG.with(|cfg| {
        cfg.borrow()
            .as_ref()
            .and_then(|c| c.log_rotation_config.clone())
    })
}

fn append_to_log_file(message: &str) -> std::io::Result<()> {
    // ログファイルアクセスを排他制御で保護（マルチスレッド環境での競合を防止）
    let _guard = get_log_file_lock().lock().map_err(|e| {
        std::io::Error::other(format!("ログファイルロックの取得に失敗しました: {}", e))
    })?;

    if let Ok(path) = get_log_file_path() {
        // ローテーションチェック
        if should_rotate_log(&path).unwrap_or(false) {
            if let Err(e) = rotate_log_file(&path) {
                let error_message = format!("ログローテーションに失敗しました: {}", e);

                // ターミナル実行時は標準エラー出力のみ
                if is_running_in_terminal() {
                    eprintln!("[WARN] {}", error_message);
                } else {
                    // 非ターミナル実行時は通知センターにも表示
                    eprintln!("[WARN] {}", error_message);
                    show_rotation_error_notification(&error_message);
                }
            }
        }

        // ログファイルに追記
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", message)?;
    }
    Ok(())
}

/// 現在時刻のタイムスタンプを YYYY-MM-DD HH:MM:SS 形式で取得する
fn get_timestamp_string() -> String {
    use chrono::Local;
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn init(config: LoggerConfig) {
    let filter_level = if config.debug_mode {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    // コンフィグをスレッドローカルストレージに保存
    LOGGER_CONFIG.with(|cfg| {
        *cfg.borrow_mut() = Some(LoggerConfig {
            debug_mode: config.debug_mode,
            silent_mode: config.silent_mode,
            notification_config: config.notification_config.clone(),
            log_rotation_config: config.log_rotation_config.clone(),
        });
    });

    env_logger::Builder::from_default_env()
        .filter_level(filter_level)
        .format(|buf, record| {
            let log_message = format!(
                "[{}] [{}] {}",
                get_timestamp_string(),
                record.level(),
                record.args()
            );

            // ログファイルに書き込む
            let _ = append_to_log_file(&log_message);

            writeln!(buf, "{}", log_message)
        })
        .try_init()
        .ok();
}

pub fn init_simple() {
    let config = LoggerConfig {
        debug_mode: false,
        silent_mode: false,
        notification_config: Some(NotificationConfig::default()),
        log_rotation_config: None,
    };
    init(config);
}

pub fn show_notification(level: NotificationLevel, message: &str) {
    let notification_type = match level {
        NotificationLevel::Info => "INFO",
        NotificationLevel::Warn => "WARN",
        NotificationLevel::Error => "ERROR",
    };

    let output_message = format!("[{}] {}", notification_type, message);

    // タイムスタンプ付きメッセージをログファイルに記録
    let log_message = format!("[{}] {}", get_timestamp_string(), output_message);
    let _ = append_to_log_file(&log_message);

    if is_running_in_terminal() {
        // ターミナル実行時は標準出力に出力（サイレントモードで抑制可能）
        let silent_mode = LOGGER_CONFIG.with(|cfg| {
            cfg.borrow()
                .as_ref()
                .map(|c| c.silent_mode)
                .unwrap_or(false)
        });

        if !silent_mode {
            println!("{}", output_message);
        }
    } else {
        // ターミナル外実行時は通知を表示（サイレントモードでも表示）
        show_os_notification(level, message);
    }
}

fn show_os_notification(level: NotificationLevel, message: &str) {
    // LoggerConfig から通知設定を取得
    let notification_method = LOGGER_CONFIG.with(|cfg| {
        cfg.borrow().as_ref().and_then(|config| {
            config.notification_config.as_ref().map(|nc| match level {
                NotificationLevel::Info => nc.info.clone(),
                NotificationLevel::Warn => nc.warn.clone(),
                NotificationLevel::Error => nc.error.clone(),
            })
        })
    });

    // デフォルト値を使用（設定がない場合）
    let notification_method = notification_method.unwrap_or_else(|| {
        let default_config = NotificationConfig::default();
        match level {
            NotificationLevel::Info => default_config.info,
            NotificationLevel::Warn => default_config.warn,
            NotificationLevel::Error => default_config.error,
        }
    });

    // 通知方法に応じて実行
    match notification_method.as_str() {
        "none" => {
            // 通知なし
        }
        "notification" => {
            show_notification_center(message);
        }
        "dialog" => {
            show_dialog(level, message);
        }
        _ => {
            // デフォルトは設定に応じて
            match level {
                NotificationLevel::Info | NotificationLevel::Warn => {
                    show_notification_center(message);
                }
                NotificationLevel::Error => {
                    show_dialog(level, message);
                }
            }
        }
    }
}

/// 通知センターに通知を表示する
///
/// **アイコン表示について**:
/// AppleScript の `display notification` ではカスタムアイコンファイル（.icns）の
/// 直接指定はサポートされていません。通知センターに表示されるアイコンは、
/// osascript を実行した親プロセス（Script Editor）のアイコンが使用されます。
///
/// 将来、アプリケーションバンドル化した際には、バンドルのアイコンが自動的に適用されます。
fn show_notification_center(message: &str) {
    let script = format!(
        r#"display notification "{}" with title "App Tidying""#,
        super::applescript::escape_applescript_string(message)
    );
    match Command::new("osascript").arg("-e").arg(&script).output() {
        Ok(output) if !output.status.success() => {
            log::warn!(
                "通知の表示に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            log::warn!("osascript の実行に失敗しました: {}", e);
        }
        _ => {}
    }
}

/// ダイアログ表示用のメッセージを作成する
///
/// エラーレベルの場合、ログファイルへの参照を含めたメッセージを生成します。
/// その他のレベルではメッセージをそのまま返します。
///
/// 注: テスト用に公開（将来的な機能拡張で使用する可能性があるため pub として残す）
pub fn create_dialog_message(level: &NotificationLevel, message: &str) -> String {
    match level {
        NotificationLevel::Error => {
            // ログファイルパスを取得
            let log_file_path = match get_log_file_path() {
                Ok(path) => path.display().to_string(),
                Err(_) => "~/Library/Application Support/biz.nosetech.apptidying/apptidying.log"
                    .to_string(),
            };

            format!(
                "{}\n\n詳細はログファイルを参照してください:\n{}",
                message, log_file_path
            )
        }
        _ => message.to_string(),
    }
}

/// ダイアログを表示する
///
/// 通知レベルに応じた標準アイコンを表示します：
/// - INFO: note（青い情報アイコン）
/// - WARN: caution（黄色い警告アイコン）
/// - ERROR: stop（赤いエラーアイコン）
fn show_dialog(level: NotificationLevel, message: &str) {
    let icon = match level {
        NotificationLevel::Info => "note",
        NotificationLevel::Warn => "caution",
        NotificationLevel::Error => "stop",
    };

    // ダイアログ表示用のメッセージを作成
    let dialog_message = create_dialog_message(&level, message);

    let script = format!(
        r#"display dialog "{}" buttons {{"OK"}} default button "OK" with icon {}"#,
        super::applescript::escape_applescript_string(&dialog_message),
        icon
    );
    match Command::new("osascript").arg("-e").arg(&script).output() {
        Ok(output) if !output.status.success() => {
            log::warn!(
                "ダイアログの表示に失敗しました: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Err(e) => {
            log::warn!("osascript の実行に失敗しました: {}", e);
        }
        _ => {}
    }
}

/// 現在のロガー設定から通知設定を取得
///
/// スレッドローカルストレージ内の通知設定を返します。
///
/// # 戻り値
/// - `Some(NotificationConfig)` - 通知設定が設定されている場合
/// - `None` - 通知設定が設定されていない場合
///
/// 注意: この関数は将来の通知設定取得API用に残されています。
#[allow(dead_code)] // 将来の拡張用に残す（通知設定取得API）
pub fn get_notification_config() -> Option<NotificationConfig> {
    LOGGER_CONFIG.with(|cfg| {
        cfg.borrow()
            .as_ref()
            .and_then(|c| c.notification_config.clone())
    })
}

pub fn escape_applescript_string_for_test(s: &str) -> String {
    // このメソッドはテスト互換性のため、applescript モジュールにデリゲート
    super::applescript::escape_applescript_string(s)
}

/// ローテーション失敗時の通知を表示（循環依存を避けるため単純な実装）
fn show_rotation_error_notification(error_message: &str) {
    let notification_message =
        format!("App Tidying: ログローテーションエラー\n\n{}", error_message);

    let script = format!(
        r#"display notification "{}" with title "App Tidying""#,
        super::applescript::escape_applescript_string(&notification_message)
    );

    match Command::new("osascript").arg("-e").arg(&script).output() {
        Ok(output) if !output.status.success() => {
            eprintln!("ローテーション失敗通知の表示に失敗しました");
        }
        Err(e) => {
            eprintln!(
                "ローテーション失敗通知の osascript 実行に失敗しました: {}",
                e
            );
        }
        _ => {}
    }
}
