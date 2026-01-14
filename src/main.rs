mod applescript;
mod cli;
mod config;
mod loader;
mod logger;
mod saver;

use clap::Parser;

use cli::{Cli, Commands};
use logger::LoggerConfig;

fn main() {
    let args = Cli::parse();

    let logger_config = LoggerConfig {
        debug_mode: args.verbose,
        notification_config: None,
    };

    logger::init(logger_config);

    log::info!("App Tidying started");
    log::debug!("Command: {:?}", args.command);

    match args.command {
        Commands::Load { path } => {
            // 1. 設定ファイルのロード
            let config = if let Some(path) = path {
                log::info!("Loading layout from: {}", path.display());
                match config::load_config_file(&path) {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        log::error!("設定ファイルの読み込みに失敗しました: {}", e);
                        logger::show_notification(
                            logger::NotificationLevel::Error,
                            &format!("設定ファイルの読み込みに失敗しました: {}", e),
                        );
                        std::process::exit(1);
                    }
                }
            } else {
                log::info!("Loading layout from default configuration");
                match config::load_default_config() {
                    Ok(cfg) => cfg,
                    Err(e) => {
                        log::error!("デフォルト設定ファイルの読み込みに失敗しました: {}", e);
                        logger::show_notification(
                            logger::NotificationLevel::Error,
                            &format!("デフォルト設定ファイルの読み込みに失敗しました: {}", e),
                        );
                        std::process::exit(1);
                    }
                }
            };

            // 2. タイムアウト設定の取得（デフォルト3000ms）
            let timeout = config.timeout.unwrap_or(3000);

            // 3. load処理の実行
            match loader::load_layout(&config, timeout) {
                Ok(result) => {
                    // 成功時の処理
                    if result.all_success {
                        log::info!("ウィンドウレイアウトを正常に復元しました");
                        logger::show_notification(
                            logger::NotificationLevel::Info,
                            "ウィンドウレイアウトを正常に復元しました",
                        );
                    } else {
                        // 部分失敗
                        let message = format!(
                            "一部のウィンドウ配置に失敗しました（成功: {}, 失敗: {}）",
                            result.success_count, result.failure_count
                        );
                        log::warn!("{}", message);
                        logger::show_notification(logger::NotificationLevel::Warn, &message);
                    }
                }
                Err(e) => {
                    // 全体失敗
                    log::error!("ウィンドウレイアウトの復元に失敗しました: {}", e);
                    logger::show_notification(
                        logger::NotificationLevel::Error,
                        &format!("ウィンドウレイアウトの復元に失敗しました: {}", e),
                    );
                    std::process::exit(1);
                }
            }
        }
        Commands::Save { path, own } => {
            // 1. 保存先パスを決定
            let output_path = if let Some(path) = path {
                log::info!("レイアウトを保存します: {}", path.display());
                path
            } else {
                log::info!("デフォルト設定にレイアウトを保存します");
                match config::get_default_config_path() {
                    Ok(default_path) => default_path,
                    Err(e) => {
                        log::error!("デフォルト設定パスの取得に失敗しました: {}", e);
                        logger::show_notification(
                            logger::NotificationLevel::Error,
                            &format!("デフォルト設定パスの取得に失敗しました: {}", e),
                        );
                        std::process::exit(1);
                    }
                }
            };

            if own {
                log::info!("ターミナルウィンドウを含めて保存します");
            }

            // 2. save処理の実行
            match saver::save_layout(&output_path, own) {
                Ok(result) => {
                    // 成功時の処理
                    if result.all_success {
                        let message = format!(
                            "ウィンドウレイアウトを保存しました（アプリ: {}, ウィンドウ: {}）",
                            result.saved_app_count, result.saved_window_count
                        );
                        log::info!("{}", message);
                        logger::show_notification(logger::NotificationLevel::Info, &message);
                    } else {
                        // 部分失敗
                        let message = format!(
                            "一部のウィンドウ情報取得に失敗しました（保存: {}, スキップ: {}, 失敗アプリ: {}）",
                            result.saved_window_count,
                            result.skipped_window_count,
                            result.failed_apps.join(", ")
                        );
                        log::warn!("{}", message);
                        logger::show_notification(logger::NotificationLevel::Warn, &message);
                    }
                }
                Err(e) => {
                    // 全体失敗
                    log::error!("ウィンドウレイアウトの保存に失敗しました: {}", e);
                    logger::show_notification(
                        logger::NotificationLevel::Error,
                        &format!("ウィンドウレイアウトの保存に失敗しました: {}", e),
                    );
                    std::process::exit(1);
                }
            }
        }
    }
}
