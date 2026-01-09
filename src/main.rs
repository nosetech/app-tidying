mod applescript;
mod cli;
mod config;
mod loader;
mod logger;

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
            if let Some(path) = path {
                log::info!("Saving layout to: {}", path.display());
            } else {
                log::info!("Saving layout to default configuration");
            }
            if own {
                log::info!("Including terminal window where apptidying is running");
            }
        }
    }
}
