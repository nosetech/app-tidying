use app_tidying::cli::Cli;
use clap::Parser;

#[test]
fn test_cli_parser() {
    let args = vec!["app-tidying"];
    let cli = Cli::try_parse_from(&args);
    assert!(cli.is_ok());
}
