use apptidying::cli::{Cli, Commands};
use clap::Parser;
use std::path::PathBuf;

// ========================================
// 正常系テスト（同値分割: 有効な入力）
// ========================================

#[test]
fn test_load_command_without_path() {
    // 境界値: pathが指定されていないケース
    let args = vec!["apptidying", "load"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Load { path } => {
            assert!(path.is_none(), "path should be None when not specified");
        }
        _ => panic!("Expected Load command"),
    }
    assert!(!cli.verbose, "verbose should be false by default");
}

#[test]
fn test_load_command_with_path() {
    // 境界値: pathが指定されているケース
    let test_path = "/tmp/test_layout.json";
    let args = vec!["apptidying", "load", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Load { path } => {
            assert!(path.is_some(), "path should be Some when specified");
            assert_eq!(
                path.unwrap(),
                PathBuf::from(test_path),
                "path should match the specified value"
            );
        }
        _ => panic!("Expected Load command"),
    }
}

#[test]
fn test_load_command_with_relative_path() {
    // 同値分割: 相対パスの有効な入力
    let test_path = "layouts/my_layout.json";
    let args = vec!["apptidying", "load", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Load { path } => {
            assert_eq!(path.unwrap(), PathBuf::from(test_path));
        }
        _ => panic!("Expected Load command"),
    }
}

#[test]
fn test_save_command_without_path() {
    // 境界値: pathが指定されていないケース
    let args = vec!["apptidying", "save"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Save { path, own } => {
            assert!(path.is_none(), "path should be None when not specified");
            assert!(!own, "own flag should be false by default");
        }
        _ => panic!("Expected Save command"),
    }
}

#[test]
fn test_save_command_with_path() {
    // 境界値: pathが指定されているケース
    let test_path = "/tmp/saved_layout.json";
    let args = vec!["apptidying", "save", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Save { path, own } => {
            assert!(path.is_some(), "path should be Some when specified");
            assert_eq!(
                path.unwrap(),
                PathBuf::from(test_path),
                "path should match the specified value"
            );
            assert!(!own, "own flag should be false when not specified");
        }
        _ => panic!("Expected Save command"),
    }
}

#[test]
fn test_save_command_with_own_flag() {
    // 境界値: ownフラグが指定されているケース
    let args = vec!["apptidying", "save", "--own"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Save { path, own } => {
            assert!(path.is_none(), "path should be None when not specified");
            assert!(own, "own flag should be true when specified");
        }
        _ => panic!("Expected Save command"),
    }
}

#[test]
fn test_save_command_with_own_flag_and_path() {
    // 同値分割: ownフラグとpathの両方が指定されているケース
    let test_path = "/tmp/my_layout.json";
    let args = vec!["apptidying", "save", "--own", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Save { path, own } => {
            assert!(path.is_some(), "path should be Some when specified");
            assert_eq!(path.unwrap(), PathBuf::from(test_path));
            assert!(own, "own flag should be true when specified");
        }
        _ => panic!("Expected Save command"),
    }
}

#[test]
fn test_save_command_with_path_and_own_flag_reverse_order() {
    // 同値分割: 引数の順序が逆のケース
    let test_path = "/tmp/my_layout.json";
    let args = vec!["apptidying", "save", test_path, "--own"];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Save { path, own } => {
            assert_eq!(path.unwrap(), PathBuf::from(test_path));
            assert!(own);
        }
        _ => panic!("Expected Save command"),
    }
}

// ========================================
// グローバルオプション: --verbose / -v
// ========================================

#[test]
fn test_verbose_flag_long_form_with_load() {
    // 同値分割: --verbose フラグが指定されているケース
    let args = vec!["apptidying", "--verbose", "load"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert!(cli.verbose, "verbose flag should be true when --verbose is specified");
}

#[test]
fn test_verbose_flag_short_form_with_load() {
    // 同値分割: -v フラグが指定されているケース
    let args = vec!["apptidying", "-v", "load"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert!(cli.verbose, "verbose flag should be true when -v is specified");
}

#[test]
fn test_verbose_flag_with_save() {
    let args = vec!["apptidying", "--verbose", "save"];
    let cli = Cli::try_parse_from(args).unwrap();

    assert!(cli.verbose);
    match cli.command {
        Commands::Save { .. } => {}
        _ => panic!("Expected Save command"),
    }
}

#[test]
fn test_verbose_flag_after_subcommand() {
    // 境界値: --verbose フラグがサブコマンドの後に指定されているケース（無効）
    // clapのグローバルオプションはサブコマンドの前に配置する必要がある
    let args = vec!["apptidying", "load", "--verbose"];
    let result = Cli::try_parse_from(args);

    assert!(
        result.is_err(),
        "should fail when global option is placed after subcommand"
    );
}

// ========================================
// パスのバリエーション（同値分割）
// ========================================

#[test]
fn test_path_with_spaces() {
    // 同値分割: スペースを含むパス
    let test_path = "/tmp/my layout/config.json";
    let args = vec!["apptidying", "load", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Load { path } => {
            assert_eq!(path.unwrap(), PathBuf::from(test_path));
        }
        _ => panic!("Expected Load command"),
    }
}

#[test]
fn test_path_with_tilde() {
    // 同値分割: チルダを含むパス
    let test_path = "~/Documents/layout.json";
    let args = vec!["apptidying", "save", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Save { path, .. } => {
            assert_eq!(path.unwrap(), PathBuf::from(test_path));
        }
        _ => panic!("Expected Save command"),
    }
}

#[test]
fn test_path_with_special_characters() {
    // 同値分割: 特殊文字を含むパス
    let test_path = "/tmp/layout_v1.0-beta.json";
    let args = vec!["apptidying", "load", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Load { path } => {
            assert_eq!(path.unwrap(), PathBuf::from(test_path));
        }
        _ => panic!("Expected Load command"),
    }
}

// ========================================
// 異常系テスト（同値分割: 無効な入力）
// ========================================

#[test]
fn test_no_subcommand() {
    // 境界値: サブコマンドがないケース
    let args = vec!["apptidying"];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err(), "should fail when no subcommand is provided");
}

#[test]
fn test_invalid_subcommand() {
    // 同値分割: 無効なサブコマンド
    let args = vec!["apptidying", "invalid"];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err(), "should fail with invalid subcommand");
}

#[test]
fn test_load_with_multiple_paths() {
    // 境界値: 複数のパス引数（無効）
    let args = vec!["apptidying", "load", "/tmp/path1.json", "/tmp/path2.json"];
    let result = Cli::try_parse_from(args);

    assert!(
        result.is_err(),
        "should fail when multiple paths are provided to load"
    );
}

#[test]
fn test_save_with_multiple_paths() {
    // 境界値: 複数のパス引数（無効）
    let args = vec!["apptidying", "save", "/tmp/path1.json", "/tmp/path2.json"];
    let result = Cli::try_parse_from(args);

    assert!(
        result.is_err(),
        "should fail when multiple paths are provided to save"
    );
}

#[test]
fn test_unknown_flag() {
    // 同値分割: 無効なフラグ
    let args = vec!["apptidying", "--unknown", "load"];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err(), "should fail with unknown flag");
}

#[test]
fn test_load_with_own_flag() {
    // 同値分割: loadコマンドに--ownフラグは無効
    let args = vec!["apptidying", "load", "--own"];
    let result = Cli::try_parse_from(args);

    assert!(
        result.is_err(),
        "should fail when --own flag is used with load command"
    );
}

// ========================================
// ヘルプとバージョン表示テスト
// ========================================

#[test]
fn test_help_flag_long_form() {
    // --help フラグのテスト
    let args = vec!["apptidying", "--help"];
    let result = Cli::try_parse_from(args);

    // clapはヘルプ表示時にエラーを返すが、これは正常な動作
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
}

#[test]
fn test_help_flag_short_form() {
    // -h フラグのテスト
    let args = vec!["apptidying", "-h"];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
}

#[test]
fn test_version_flag_long_form() {
    // --version フラグのテスト
    let args = vec!["apptidying", "--version"];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
}

#[test]
fn test_version_flag_short_form() {
    // -V フラグのテスト
    let args = vec!["apptidying", "-V"];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::DisplayVersion);
}

#[test]
fn test_subcommand_help() {
    // サブコマンドのヘルプ表示
    let args = vec!["apptidying", "load", "--help"];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::DisplayHelp);
}

// ========================================
// 複雑な組み合わせテスト
// ========================================

#[test]
fn test_all_options_combined() {
    // 同値分割: すべてのオプションの組み合わせ
    let test_path = "/tmp/complex_layout.json";
    let args = vec!["apptidying", "-v", "save", "--own", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    assert!(cli.verbose, "verbose should be true");
    match cli.command {
        Commands::Save { path, own } => {
            assert_eq!(path.unwrap(), PathBuf::from(test_path));
            assert!(own, "own flag should be true");
        }
        _ => panic!("Expected Save command"),
    }
}

#[test]
fn test_verbose_and_load_with_path() {
    let test_path = "/tmp/layout.json";
    let args = vec!["apptidying", "--verbose", "load", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    assert!(cli.verbose);
    match cli.command {
        Commands::Load { path } => {
            assert_eq!(path.unwrap(), PathBuf::from(test_path));
        }
        _ => panic!("Expected Load command"),
    }
}

// ========================================
// エッジケース（境界値分析）
// ========================================

#[test]
fn test_empty_path() {
    // 境界値: 空のパス文字列（無効）
    // clapは空文字列をパスとして受け付けない
    let args = vec!["apptidying", "load", ""];
    let result = Cli::try_parse_from(args);

    assert!(result.is_err(), "should fail with empty path string");
}

#[test]
fn test_very_long_path() {
    // 境界値: 非常に長いパス
    let long_path = format!("/tmp/{}", "a".repeat(200));
    let args = vec!["apptidying", "save", &long_path];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Save { path, .. } => {
            assert_eq!(path.unwrap(), PathBuf::from(&long_path));
        }
        _ => panic!("Expected Save command"),
    }
}

#[test]
fn test_path_with_unicode() {
    // 同値分割: Unicode文字を含むパス
    let test_path = "/tmp/レイアウト設定.json";
    let args = vec!["apptidying", "load", test_path];
    let cli = Cli::try_parse_from(args).unwrap();

    match cli.command {
        Commands::Load { path } => {
            assert_eq!(path.unwrap(), PathBuf::from(test_path));
        }
        _ => panic!("Expected Load command"),
    }
}

// ========================================
// デバッグ出力のテスト（動作確認用）
// ========================================

#[test]
fn test_debug_format() {
    // Debugトレイトが正しく実装されていることを確認
    let args = vec!["apptidying", "load"];
    let cli = Cli::try_parse_from(args).unwrap();

    let debug_string = format!("{:?}", cli);
    assert!(debug_string.contains("Load"), "Debug output should contain 'Load'");
}
