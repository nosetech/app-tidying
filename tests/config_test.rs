use apptidying::applescript::DisplayInfo;
use apptidying::config::{
    parse_position_value, parse_size_value, validate_config, validate_config_bounds,
    validate_config_syntax, AppConfig, AppWindowConfig, DisplayConfig, LayoutConfig, Position,
    Size,
};
use serde_json::json;

// =============================================================================
// parse_position_value() Tests
// =============================================================================

#[test]
fn test_parse_position_left_top() {
    let position = json!({
        "x": "left",
        "y": "top"
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 0);
    assert_eq!(y, 25); // top = menu bar height
}

#[test]
fn test_parse_position_right_bottom() {
    let position = json!({
        "x": "right",
        "y": "bottom"
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 1920 - 800); // display_width - window_width
    assert_eq!(y, 1080 - 600); // display_height - window_height
}

#[test]
fn test_parse_position_left_bottom() {
    let position = json!({
        "x": "left",
        "y": "bottom"
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 0);
    assert_eq!(y, 1080 - 600);
}

#[test]
fn test_parse_position_right_top() {
    let position = json!({
        "x": "right",
        "y": "top"
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 1920 - 800);
    assert_eq!(y, 25);
}

#[test]
fn test_parse_position_absolute_coordinates() {
    let position = json!({
        "x": 100,
        "y": 200
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 100);
    assert_eq!(y, 200);
}

#[test]
fn test_parse_position_mixed_pattern_and_number() {
    let position = json!({
        "x": "left",
        "y": 300
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 0);
    assert_eq!(y, 300);
}

#[test]
fn test_parse_position_boundary_zero() {
    let position = json!({
        "x": 0,
        "y": 0
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 0);
    assert_eq!(y, 0);
}

#[test]
fn test_parse_position_boundary_max() {
    let position = json!({
        "x": 1920,
        "y": 1080
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 1920);
    assert_eq!(y, 1080);
}

#[test]
fn test_parse_position_window_larger_than_display() {
    // window_width > display_width の場合でも計算可能
    let position = json!({
        "x": "right",
        "y": "bottom"
    });

    let result = parse_position_value(&position, 1920, 1080, 2500, 1500, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 1920 - 2500); // 負の値になる
    assert_eq!(y, 1080 - 1500);
}

#[test]
fn test_parse_position_invalid_x_pattern() {
    let position = json!({
        "x": "center",
        "y": "top"
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("無効な x 値"));
}

#[test]
fn test_parse_position_invalid_y_pattern() {
    let position = json!({
        "x": "left",
        "y": "middle"
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("無効な y 値"));
}

#[test]
fn test_parse_position_negative_x() {
    let position = json!({
        "x": -100,
        "y": 200
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("x が負です"));
}

#[test]
fn test_parse_position_negative_y() {
    let position = json!({
        "x": 100,
        "y": -200
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("y が負です"));
}

#[test]
fn test_parse_position_missing_x_field() {
    let position = json!({
        "y": "top"
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("x フィールドが見つかりません"));
}

#[test]
fn test_parse_position_missing_y_field() {
    let position = json!({
        "x": "left"
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("y フィールドが見つかりません"));
}

#[test]
fn test_parse_position_not_object() {
    let position = json!("not an object");

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("オブジェクトである必要があります"));
}

#[test]
fn test_parse_position_x_invalid_type() {
    let position = json!({
        "x": true,
        "y": "top"
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("文字列または数値である必要があります"));
}

#[test]
fn test_parse_position_y_invalid_type() {
    let position = json!({
        "x": "left",
        "y": []
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("文字列または数値である必要があります"));
}

#[test]
fn test_parse_position_float_coordinates() {
    // 浮動小数点数の扱い (整数として扱われない)
    let position = json!({
        "x": 100.5,
        "y": 200.7
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("整数である必要があります"));
}

// =============================================================================
// parse_size_value() Tests
// =============================================================================

#[test]
fn test_parse_size_half_half() {
    let size = json!({
        "width": "half",
        "height": "half"
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 1920 / 2);
    assert_eq!(height, 1080 / 2);
}

#[test]
fn test_parse_size_third_third() {
    let size = json!({
        "width": "third",
        "height": "third"
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 1920 / 3);
    assert_eq!(height, 1080 / 3);
}

#[test]
fn test_parse_size_max_max() {
    let size = json!({
        "width": "max",
        "height": "max"
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 1920);
    assert_eq!(height, 1080);
}

#[test]
fn test_parse_size_mixed_pattern() {
    let size = json!({
        "width": "half",
        "height": "max"
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 1920 / 2);
    assert_eq!(height, 1080);
}

#[test]
fn test_parse_size_absolute_values() {
    let size = json!({
        "width": 1440,
        "height": 900
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 1440);
    assert_eq!(height, 900);
}

#[test]
fn test_parse_size_mixed_pattern_and_number() {
    let size = json!({
        "width": "half",
        "height": 500
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 1920 / 2);
    assert_eq!(height, 500);
}

#[test]
fn test_parse_size_boundary_min() {
    let size = json!({
        "width": 1,
        "height": 1
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 1);
    assert_eq!(height, 1);
}

#[test]
fn test_parse_size_boundary_max() {
    let size = json!({
        "width": 3840,
        "height": 2160
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 3840);
    assert_eq!(height, 2160);
}

#[test]
fn test_parse_size_larger_than_display() {
    // ディスプレイより大きいサイズでも計算は可能（後でバリデーション）
    let size = json!({
        "width": 5000,
        "height": 3000
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 5000);
    assert_eq!(height, 3000);
}

#[test]
fn test_parse_size_invalid_width_pattern() {
    let size = json!({
        "width": "quarter",
        "height": "half"
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("無効な width 値"));
}

#[test]
fn test_parse_size_invalid_height_pattern() {
    let size = json!({
        "width": "half",
        "height": "quarter"
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("無効な height 値"));
}

#[test]
fn test_parse_size_zero_width() {
    let size = json!({
        "width": 0,
        "height": 500
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("正の値である必要があります"));
}

#[test]
fn test_parse_size_zero_height() {
    let size = json!({
        "width": 500,
        "height": 0
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("正の値である必要があります"));
}

#[test]
fn test_parse_size_negative_width() {
    let size = json!({
        "width": -100,
        "height": 500
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("正の値である必要があります"));
}

#[test]
fn test_parse_size_negative_height() {
    let size = json!({
        "width": 500,
        "height": -100
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("正の値である必要があります"));
}

#[test]
fn test_parse_size_missing_width_field() {
    let size = json!({
        "height": 500
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("width フィールドが見つかりません"));
}

#[test]
fn test_parse_size_missing_height_field() {
    let size = json!({
        "width": 500
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("height フィールドが見つかりません"));
}

#[test]
fn test_parse_size_not_object() {
    let size = json!("not an object");

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("オブジェクトである必要があります"));
}

#[test]
fn test_parse_size_width_invalid_type() {
    let size = json!({
        "width": true,
        "height": 500
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("文字列または数値である必要があります"));
}

#[test]
fn test_parse_size_height_invalid_type() {
    let size = json!({
        "width": 500,
        "height": []
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("文字列または数値である必要があります"));
}

#[test]
fn test_parse_size_float_values() {
    // 浮動小数点数の扱い
    let size = json!({
        "width": 100.5,
        "height": 200.7
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("整数である必要があります"));
}

// =============================================================================
// Edge Cases and Boundary Value Tests
// =============================================================================

#[test]
fn test_parse_position_very_large_coordinates() {
    let position = json!({
        "x": 100000,
        "y": 100000
    });

    let result = parse_position_value(&position, 1920, 1080, 800, 600, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 100000);
    assert_eq!(y, 100000);
}

#[test]
fn test_parse_size_very_large_values() {
    let size = json!({
        "width": 100000,
        "height": 100000
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 100000);
    assert_eq!(height, 100000);
}

#[test]
fn test_parse_position_small_display() {
    // 小さいディスプレイでの動作
    let position = json!({
        "x": "right",
        "y": "bottom"
    });

    let result = parse_position_value(&position, 800, 600, 400, 300, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 400);
    assert_eq!(y, 300);
}

#[test]
fn test_parse_size_small_display() {
    let size = json!({
        "width": "half",
        "height": "third"
    });

    let result = parse_size_value(&size, 800, 600, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 400);
    assert_eq!(height, 200);
}

#[test]
fn test_parse_position_4k_display() {
    // 4Kディスプレイでの動作
    let position = json!({
        "x": "right",
        "y": "bottom"
    });

    let result = parse_position_value(&position, 3840, 2160, 1920, 1080, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 1920);
    assert_eq!(y, 1080);
}

#[test]
fn test_parse_size_4k_display() {
    let size = json!({
        "width": "half",
        "height": "half"
    });

    let result = parse_size_value(&size, 3840, 2160, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 1920);
    assert_eq!(height, 1080);
}

#[test]
fn test_parse_position_odd_display_dimensions() {
    // 奇数サイズのディスプレイ
    let position = json!({
        "x": "right",
        "y": "bottom"
    });

    let result = parse_position_value(&position, 1921, 1081, 801, 601, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 1120);
    assert_eq!(y, 480);
}

#[test]
fn test_parse_size_odd_display_dimensions() {
    // 奇数サイズでの half, third の計算
    let size = json!({
        "width": "half",
        "height": "third"
    });

    let result = parse_size_value(&size, 1921, 1081, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 1921 / 2); // 整数除算
    assert_eq!(height, 1081 / 3);
}

// =============================================================================
// Configuration Validation Tests (Phase 3-4)
// =============================================================================

/// validate_config_syntax() がバージョン確認を正確に実行することを検証
#[test]
fn test_validate_config_syntax_version_ok() {
    // 目的: サポートされているバージョン (1.0) の設定が成功することを確認
    // 検証項目: バージョン 1.0 がサポートされていることを確認

    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Google Chrome".to_string(),
                    title: None,
                    position: None,
                    size: None,
                }],
            }],
        }],
        notification: None,
        timeout: None,
    };

    // 検証: バージョンチェックが成功する
    let result = validate_config_syntax(&config);
    assert!(result.is_ok());
}

/// validate_config_syntax() がサポートされていないバージョンでエラーを返すことを確認
#[test]
fn test_validate_config_syntax_version_ng() {
    // 目的: サポートされていないバージョン (2.0) の設定がエラーになることを確認
    // 検証項目: バージョン 2.0 がエラーになる

    let config = AppConfig {
        version: "2.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Google Chrome".to_string(),
                    title: None,
                    position: None,
                    size: None,
                }],
            }],
        }],
        notification: None,
        timeout: None,
    };

    // 検証: バージョンチェックがエラーになる
    let result = validate_config_syntax(&config);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .message
        .contains("サポートされていないバージョン"));
}

/// validate_config_bounds() がディスプレイ外の座標を検出することを確認
#[test]
fn test_validate_display_bounds_position_out_of_display() {
    // 目的: ウィンドウの右端がディスプレイを超える場合にワーニングが発生することを確認
    // 検証項目: 座標とサイズの組み合わせでディスプレイ外判定が正確に動作

    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Google Chrome".to_string(),
                    title: None,
                    position: Some(Position {
                        x: json!(1800), // 1800 から始まる
                        y: json!("top"),
                    }),
                    size: Some(Size {
                        width: json!(500), // 500 幅（1800 + 500 = 2300 > 1920）
                        height: json!("max"),
                    }),
                }],
            }],
        }],
        notification: None,
        timeout: None,
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: 座標がディスプレイ外の場合、ワーニングが返される
    let result = validate_config_bounds(&config, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("ウィンドウの右端"));
    assert_eq!(warnings[0].app_name, "Google Chrome");
}

/// validate_config_bounds() が画面より大きいサイズを検出することを確認
#[test]
fn test_validate_display_bounds_size_larger_than_display() {
    // 目的: ウィンドウの高さがディスプレイを超える場合にワーニングが発生することを確認
    // 検証項目: サイズがディスプレイより大きい場合の検出

    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Safari".to_string(),
                    title: None,
                    position: Some(Position {
                        x: json!("left"),
                        y: json!("top"),
                    }),
                    size: Some(Size {
                        width: json!(800),
                        height: json!(1500), // 1500 > 1080（ディスプレイ高）
                    }),
                }],
            }],
        }],
        notification: None,
        timeout: None,
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: サイズがディスプレイを超える場合、ワーニングが返される
    let result = validate_config_bounds(&config, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("ウィンドウの下端"));
    assert_eq!(warnings[0].app_name, "Safari");
}

/// validate_config_bounds() が接続されているディスプレイを正確に判定することを確認
#[test]
fn test_validate_display_exists_ok() {
    // 目的: 接続されているディスプレイ名が正確に判定されることを確認
    // 検証項目: 複数のディスプレイが接続されている場合、正しいものを特定

    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "External Display".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Xcode".to_string(),
                    title: None,
                    position: None,
                    size: None,
                }],
            }],
        }],
        notification: None,
        timeout: None,
    };

    let connected_displays = vec![
        DisplayInfo {
            name: "Built-in".to_string(),
            width: 1440,
            height: 900,
            origin_x: 0,
            origin_y: 0,
        },
        DisplayInfo {
            name: "External Display".to_string(),
            width: 2560,
            height: 1440,
            origin_x: 1440,
            origin_y: 0,
        },
    ];

    // 検証: 接続されているディスプレイに対してワーニングが発生しない
    let result = validate_config_bounds(&config, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert!(warnings.is_empty());
}

/// validate_config_bounds() が接続されていないディスプレイを検出することを確認
#[test]
fn test_validate_display_exists_ng() {
    // 目的: 接続されていないディスプレイ名でワーニングが発生することを確認
    // 検証項目: 存在しないディスプレイに対してワーニングが返される

    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Nonexistent Display".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Terminal".to_string(),
                    title: None,
                    position: None,
                    size: None,
                }],
            }],
        }],
        notification: None,
        timeout: None,
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1440,
        height: 900,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: 接続されていないディスプレイに対してワーニングが発生する
    let result = validate_config_bounds(&config, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("ディスプレイ '"));
    assert!(warnings[0].message.contains("が接続されていません"));
    assert_eq!(warnings[0].display_name, "Nonexistent Display");
}

/// validate_config_bounds() が複数の警告を正確に返すことを確認
#[test]
fn test_validate_config_bounds_all_warnings() {
    // 目的: 複数の問題（座標外、サイズ大きい、ディスプレイなし）が同時に検出されることを確認
    // 検証項目: 複数の異なるウィンドウ設定で複数の警告が返される

    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![
                // ディスプレイ 1: 座標外
                DisplayConfig {
                    name: "Built-in".to_string(),
                    windows: vec![AppWindowConfig {
                        app: "Chrome".to_string(),
                        title: None,
                        position: Some(Position {
                            x: json!(1900),
                            y: json!("top"),
                        }),
                        size: Some(Size {
                            width: json!(500),
                            height: json!(600),
                        }),
                    }],
                },
                // ディスプレイ 2: 接続されていない
                DisplayConfig {
                    name: "Disconnected".to_string(),
                    windows: vec![AppWindowConfig {
                        app: "Safari".to_string(),
                        title: None,
                        position: None,
                        size: None,
                    }],
                },
            ],
        }],
        notification: None,
        timeout: None,
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: 複数の警告が返される
    let result = validate_config_bounds(&config, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 2); // 座標外 + ディスプレイなし
    assert!(warnings[0].message.contains("ウィンドウの右端"));
    assert!(warnings[1].message.contains("ディスプレイ"));
}

/// validate_config() が構文チェックと境界値チェックを組み合わせることを確認
#[test]
fn test_validate_config_syntax_and_bounds() {
    // 目的: ラッパー関数が構文チェックと境界値チェックを正確に実行することを確認
    // 検証項目: バージョンエラー（構文）と座標外エラー（境界値）の両方が検出される

    let config = AppConfig {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "App".to_string(),
                    title: None,
                    position: Some(Position {
                        x: json!(2000),
                        y: json!("top"),
                    }),
                    size: Some(Size {
                        width: json!(500),
                        height: json!(600),
                    }),
                }],
            }],
        }],
        notification: None,
        timeout: None,
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: 構文チェックを通り、境界値警告が返される
    let result = validate_config(&config, Some(&connected_displays));
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("右端"));
}
