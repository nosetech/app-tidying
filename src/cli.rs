use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "apptidying")]
#[command(about = "macOS application window layout management tool")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// 詳細/デバッグ出力を有効化
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 設定ファイルからウィンドウレイアウトを復元
    #[command(about = "Restore window layout")]
    Load {
        /// レイアウト設定ファイルのパス（デフォルト: ~/Library/Application Support/biz.nosetech.apptidying/layout.json）
        path: Option<PathBuf>,
    },
    /// 現在のウィンドウレイアウトを設定ファイルに保存
    #[command(about = "Save current window layout")]
    Save {
        /// 保存先レイアウト設定ファイルのパス（デフォルト: ~/Library/Application Support/biz.nosetech.apptidying/layout.json）
        path: Option<PathBuf>,

        /// 実行中のターミナルウィンドウも含める
        #[arg(long)]
        own: bool,
    },
}
