use apptidying::applescript::DisplayInfo;
use apptidying::config::{
    parse_position_value, parse_settings_from_json, parse_size_value, resolve_tile_keyword,
    validate_layout, validate_layout_bounds, validate_layout_syntax, AppWindowConfig,
    DisplayConfig, LayoutConfig, LayoutFile, LogRotationConfig, Position, Size, TileKeyword,
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 2500, 1500, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("y が負です"));
}

#[test]
fn test_parse_position_missing_x_field() {
    let position = json!({
        "y": "top"
    });

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("x フィールドが見つかりません"));
}

#[test]
fn test_parse_position_missing_y_field() {
    let position = json!({
        "x": "left"
    });

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.message.contains("y フィールドが見つかりません"));
}

#[test]
fn test_parse_position_not_object() {
    let position = json!("not an object");

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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
    assert_eq!(height, 1055); // "max" はメニューバー高さ (25px) を考慮
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
    assert_eq!(height, 1055); // "max" はメニューバー高さ (25px) を考慮
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
// エッジケースと境界値テスト
// =============================================================================

#[test]
fn test_parse_position_very_large_coordinates() {
    let position = json!({
        "x": 100000,
        "y": 100000
    });

    let result = parse_position_value(&position, None, 1920, 1080, 800, 600, "position");
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

    let result = parse_position_value(&position, None, 800, 600, 400, 300, "position");
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

    let result = parse_position_value(&position, None, 3840, 2160, 1920, 1080, "position");
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

    let result = parse_position_value(&position, None, 1921, 1081, 801, 601, "position");
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
// 設定検証テスト（フェーズ 3-4）
// =============================================================================

/// validate_layout_syntax() がバージョン確認を正確に実行することを検証
#[test]
fn test_validate_layout_syntax_version_ok() {
    // 目的: サポートされているバージョン (1.0) の設定が成功することを確認
    // 検証項目: バージョン 1.0 がサポートされていることを確認

    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Google Chrome".to_string(),
                    position: None,
                    size: None,
                }],
            }],
        }],
    };

    // 検証: バージョンチェックが成功する
    let result = validate_layout_syntax(&layout);
    assert!(result.is_ok());
}

/// validate_layout_syntax() がサポートされていないバージョンでエラーを返すことを確認
#[test]
fn test_validate_layout_syntax_version_ng() {
    // 目的: サポートされていないバージョン (2.0) の設定がエラーになることを確認
    // 検証項目: バージョン 2.0 がエラーになる

    let layout = LayoutFile {
        version: "2.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Google Chrome".to_string(),
                    position: None,
                    size: None,
                }],
            }],
        }],
    };

    // 検証: バージョンチェックがエラーになる
    let result = validate_layout_syntax(&layout);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .message
        .contains("サポートされていないバージョン"));
}

/// validate_layout_bounds() がディスプレイ外の座標を検出することを確認
#[test]
fn test_validate_display_bounds_position_out_of_display() {
    // 目的: ウィンドウの右端がディスプレイを超える場合にワーニングが発生することを確認
    // 検証項目: 座標とサイズの組み合わせでディスプレイ外判定が正確に動作

    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Google Chrome".to_string(),
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
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: 座標がディスプレイ外の場合、ワーニングが返される
    let result = validate_layout_bounds(&layout, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("ウィンドウの右端"));
    assert_eq!(warnings[0].app_name, "Google Chrome");
}

/// validate_layout_bounds() が画面より大きいサイズを検出することを確認
#[test]
fn test_validate_display_bounds_size_larger_than_display() {
    // 目的: ウィンドウの高さがディスプレイを超える場合にワーニングが発生することを確認
    // 検証項目: サイズがディスプレイより大きい場合の検出

    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Safari".to_string(),
                    position: Some(Position {
                        x: json!("left"),
                        y: json!("top"),
                    }),
                    size: Some(Size {
                        width: json!(800),
                        height: json!(1500), // 1500 > 1080(ディスプレイ高)
                    }),
                }],
            }],
        }],
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: サイズがディスプレイを超える場合、ワーニングが返される
    let result = validate_layout_bounds(&layout, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("ウィンドウの下端"));
    assert_eq!(warnings[0].app_name, "Safari");
}

/// validate_layout_bounds() が接続されているディスプレイを正確に判定することを確認
#[test]
fn test_validate_display_exists_ok() {
    // 目的: 接続されているディスプレイ名が正確に判定されることを確認
    // 検証項目: 複数のディスプレイが接続されている場合、正しいものを特定

    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "External Display".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Xcode".to_string(),
                    position: None,
                    size: None,
                }],
            }],
        }],
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
    let result = validate_layout_bounds(&layout, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert!(warnings.is_empty());
}

/// validate_layout_bounds() が接続されていないディスプレイを検出することを確認
#[test]
fn test_validate_display_exists_ng() {
    // 目的: 接続されていないディスプレイ名でワーニングが発生することを確認
    // 検証項目: 存在しないディスプレイに対してワーニングが返される

    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Nonexistent Display".to_string(),
                windows: vec![AppWindowConfig {
                    app: "Terminal".to_string(),
                    position: None,
                    size: None,
                }],
            }],
        }],
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1440,
        height: 900,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: 接続されていないディスプレイに対してワーニングが発生する
    let result = validate_layout_bounds(&layout, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("ディスプレイ '"));
    assert!(warnings[0].message.contains("が接続されていません"));
    assert_eq!(warnings[0].display_name, "Nonexistent Display");
}

/// validate_layout_bounds() が複数の警告を正確に返すことを確認
#[test]
fn test_validate_config_bounds_all_warnings() {
    // 目的: 複数の問題（座標外、サイズ大きい、ディスプレイなし）が同時に検出されることを確認
    // 検証項目: 複数の異なるウィンドウ設定で複数の警告が返される

    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![
                // ディスプレイ 1: 座標外
                DisplayConfig {
                    name: "Built-in".to_string(),
                    windows: vec![AppWindowConfig {
                        app: "Chrome".to_string(),
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
                        position: None,
                        size: None,
                    }],
                },
            ],
        }],
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: 複数の警告が返される
    let result = validate_layout_bounds(&layout, &connected_displays);
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 2); // 座標外 + ディスプレイなし
    assert!(warnings[0].message.contains("ウィンドウの右端"));
    assert!(warnings[1].message.contains("ディスプレイ"));
}

/// validate_layout() が構文チェックと境界値チェックを組み合わせることを確認
#[test]
fn test_validate_layout_syntax_and_bounds() {
    // 目的: ラッパー関数が構文チェックと境界値チェックを正確に実行することを確認
    // 検証項目: バージョンエラー（構文）と座標外エラー（境界値）の両方が検出される

    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "App".to_string(),
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
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    }];

    // 検証: 構文チェックを通り、境界値警告が返される
    let result = validate_layout(&layout, Some(&connected_displays));
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].message.contains("右端"));
}

// =============================================================================
// LogRotationConfig テスト
// =============================================================================

/// LogRotationConfig がデフォルト値で正しく作成できることを確認
#[test]
fn test_log_rotation_config_default_values() {
    // 目的: LogRotationConfig のデフォルト値の検証
    // 検証項目: rotation_type, max_size_mb, max_files のデフォルト値

    let config = LogRotationConfig::default();

    // 検証: デフォルト値が設定されている
    assert_eq!(config.rotation_type, "size");
    assert_eq!(config.max_size_mb, 10);
    assert_eq!(config.max_files, 5);
}

/// LogRotationConfig がカスタム値で作成できることを確認
#[test]
fn test_log_rotation_config_custom_values() {
    // 目的: LogRotationConfig のカスタム値での動作を検証
    // 検証項目: 各フィールドに異なる値を設定できることを確認

    let config = LogRotationConfig {
        rotation_type: "size".to_string(),
        max_size_mb: 50,
        max_files: 10,
    };

    // 検証: カスタム値が設定されている
    assert_eq!(config.rotation_type, "size");
    assert_eq!(config.max_size_mb, 50);
    assert_eq!(config.max_files, 10);
}

/// settings.json に log_rotation が含まれる場合のパース
#[test]
fn test_parse_settings_with_log_rotation() {
    // 目的: log_rotation フィールドが含まれる settings.json を正しくパースできることを確認
    // 検証項目: JSON パース、log_rotation フィールドの抽出

    let json_str = r#"{
        "version": "1.0",
        "log_rotation": {
            "rotation_type": "size",
            "max_size_mb": 20,
            "max_files": 7
        }
    }"#;

    let result = parse_settings_from_json(json_str);

    // 検証: パースが成功し、log_rotation フィールドが正しく設定されている
    assert!(result.is_ok());
    let settings = result.unwrap();
    assert!(settings.log_rotation.is_some());

    let log_rotation = settings.log_rotation.unwrap();
    assert_eq!(log_rotation.rotation_type, "size");
    assert_eq!(log_rotation.max_size_mb, 20);
    assert_eq!(log_rotation.max_files, 7);
}

/// log_rotation フィールドが省略された場合、デフォルト値が使用されることを確認
#[test]
fn test_parse_settings_without_log_rotation() {
    // 目的: log_rotation フィールドが省略されても parse_settings_from_json がエラーにならないことを確認
    // 検証項目: オプショナルフィールドとしての動作

    let json_str = r#"{
        "version": "1.0"
    }"#;

    let result = parse_settings_from_json(json_str);

    // 検証: パースが成功し、log_rotation は None
    assert!(result.is_ok());
    let settings = result.unwrap();
    assert!(settings.log_rotation.is_none());
}

/// 無効な rotation_type がエラーになることを確認
#[test]
fn test_validate_settings_with_invalid_rotation_type() {
    // 目的: 無効な rotation_type 値がバリデーションエラーになることを確認
    // 検証項目: rotation_type の値チェック

    let json_str = r#"{
        "version": "1.0",
        "log_rotation": {
            "rotation_type": "invalid_type",
            "max_size_mb": 10,
            "max_files": 5
        }
    }"#;

    let result = parse_settings_from_json(json_str);

    // 検証: バリデーションエラーが返される
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .message
        .contains("無効な log_rotation.rotation_type"));
}

/// max_size_mb が 0 の場合、エラーになることを確認
#[test]
fn test_validate_settings_with_invalid_max_size_mb() {
    // 目的: max_size_mb が 0 以下の場合、バリデーションエラーになることを確認
    // 検証項目: max_size_mb の最小値チェック

    let json_str = r#"{
        "version": "1.0",
        "log_rotation": {
            "rotation_type": "size",
            "max_size_mb": 0,
            "max_files": 5
        }
    }"#;

    let result = parse_settings_from_json(json_str);

    // 検証: バリデーションエラーが返される
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("max_size_mb は1以上"));
}

/// max_files が 0 の場合、エラーになることを確認
#[test]
fn test_validate_settings_with_invalid_max_files() {
    // 目的: max_files が 0 以下の場合、バリデーションエラーになることを確認
    // 検証項目: max_files の最小値チェック

    let json_str = r#"{
        "version": "1.0",
        "log_rotation": {
            "rotation_type": "size",
            "max_size_mb": 10,
            "max_files": 0
        }
    }"#;

    let result = parse_settings_from_json(json_str);

    // 検証: バリデーションエラーが返される
    assert!(result.is_err());
    assert!(result.unwrap_err().message.contains("max_files は1以上"));
}

// =============================================================================
// Issue #92: height/width: "max" 対応テスト
// =============================================================================

/// height が "max" の場合、display_height - 25 を返すことを確認
#[test]
fn test_parse_size_with_height_max() {
    // 目的: height が "max" の場合、メニューバー高さ (25px) を考慮した値を返すことを確認
    // 検証項目: height = "max" の場合、戻り値が display_height - 25 であること

    let size = json!({
        "width": "half",
        "height": "max"
    });

    let result = parse_size_value(&size, 1920, 1080, "size");
    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 960); // half
    assert_eq!(height, 1055); // 1080 - 25（メニューバー高さを引く）
}

/// width が "max" の場合、X座標が 0 になることを確認
#[test]
fn test_parse_position_with_width_max() {
    // 目的: width が "max" の場合、X座標が 0 に設定されることを確認
    // 検証項目: width が "max" の場合、指定された X 座標（"left"）が無視され、0 が返される

    let position = json!({
        "x": "left",
        "y": "top"
    });
    let size = json!({
        "width": "max",
        "height": "half"
    });

    let result = parse_position_value(&position, Some(&size), 1920, 1080, 1920, 540, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 0); // width: max の場合、X座標は 0
    assert_eq!(y, 25); // height: half なので、Y座標は "top" の値
}

/// height が "max" の場合、Y座標が 0 になることを確認
#[test]
fn test_parse_position_with_height_max() {
    // 目的: height が "max" の場合、Y座標が 0 に設定されることを確認
    // 検証項目: height が "max" の場合、指定された Y 座標（"top"）が無視され、0 が返される

    let position = json!({
        "x": "left",
        "y": "top"
    });
    let size = json!({
        "width": "half",
        "height": "max"
    });

    // height="max" なので高さは 1080 - 25 = 1055
    let result = parse_position_value(&position, Some(&size), 1920, 1080, 960, 1055, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 0); // width: half なので、X座標は "left" の値（0）
    assert_eq!(y, 0); // height: max の場合、Y座標は 0
}

/// width/height が両方 "max" の場合、X/Y座標が両方 0 になることを確認
#[test]
fn test_parse_position_with_both_max() {
    // 目的: width と height が両方 "max" の場合、X と Y 座標が両方 0 に設定されることを確認
    // 検証項目: "max" 指定で座標が無視され、すべて 0 になること

    let position = json!({
        "x": "right",
        "y": "bottom"
    });
    let size = json!({
        "width": "max",
        "height": "max"
    });

    // height="max" なので高さは 1080 - 25 = 1055
    let result = parse_position_value(&position, Some(&size), 1920, 1080, 1920, 1055, "position");
    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 0); // width: max の場合、X座標は 0（"right" を無視）
    assert_eq!(y, 0); // height: max の場合、Y座標は 0（"bottom" を無視）
}

/// max 指定で境界値警告が出ないことを確認
#[test]
fn test_validate_layout_bounds_with_max_no_warning() {
    // 目的: "max" 指定でメニューバーを考慮した計算が行われ、境界値警告が出ないことを確認
    // 検証項目: width="max", height="max" でディスプレイ境界内に収まっていることを確認

    let layout = LayoutFile {
        version: "1.0".to_string(),
        layouts: vec![LayoutConfig {
            displays: vec![DisplayConfig {
                name: "Built-in".to_string(),
                windows: vec![AppWindowConfig {
                    app: "TestApp".to_string(),
                    position: Some(Position {
                        x: json!("left"),
                        y: json!("top"),
                    }),
                    size: Some(Size {
                        width: json!("max"),
                        height: json!("max"),
                    }),
                }],
            }],
        }],
    };

    let connected_displays = vec![DisplayInfo {
        name: "Built-in".to_string(),
        width: 1920,
        height: 1080,
        origin_x: 0,
        origin_y: 0,
    }];

    let result = validate_layout(&layout, Some(&connected_displays));
    assert!(result.is_ok());
    let warnings = result.unwrap();
    assert_eq!(warnings.len(), 0);
}

// =============================================================================
// TileKeyword / resolve_tile_keyword() Tests (Issue #118)
//
// resolve_tile_keyword() は osascript に依存しない純粋関数のため、CI環境でも
// 全ケースを実行可能。以下、ブラックボックス技法（同値分割・境界値分析）と
// ホワイトボックス技法（全 match アーム・全ガード条件の網羅）を併用してテストする。
// =============================================================================

/// position が未指定（`None`）の `Position` を作る補助関数
///
/// `x`/`y` に `serde_json::Value::Null` を設定し、「フィールドキー自体を
/// JSON上で省略した場合」の `#[serde(default)]` の挙動を再現する。
fn null_position() -> Position {
    Position {
        x: json!(null),
        y: json!(null),
    }
}

/// size が未指定（`None`）の `Size` を作る補助関数
fn null_size() -> Size {
    Size {
        width: json!(null),
        height: json!(null),
    }
}

// --- 表の8パターン（同値分割: 有効な組み合わせの代表値） ---

/// position.x = "left", size.width = "half" のみ指定 → TileKeyword::Left
#[test]
fn test_resolve_tile_keyword_left_half() {
    // 目的: 左半分パターンが正しく Left と判定されることを確認
    // 検証項目: y/height が未指定（null）でも x=left, width=half のみで判定可能なこと
    let position = Position {
        x: json!("left"),
        y: json!(null),
    };
    let size = Size {
        width: json!("half"),
        height: json!(null),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::Left));
}

/// position.x = "right", size.width = "half" のみ指定 → TileKeyword::Right
#[test]
fn test_resolve_tile_keyword_right_half() {
    // 目的: 右半分パターンが正しく Right と判定されることを確認
    let position = Position {
        x: json!("right"),
        y: json!(null),
    };
    let size = Size {
        width: json!("half"),
        height: json!(null),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::Right));
}

/// position.y = "top", size.height = "half" のみ指定 → TileKeyword::Top
#[test]
fn test_resolve_tile_keyword_top_half() {
    // 目的: 上半分パターンが正しく Top と判定されることを確認
    // 検証項目: x/width が未指定（null）でも y=top, height=half のみで判定可能なこと
    let position = Position {
        x: json!(null),
        y: json!("top"),
    };
    let size = Size {
        width: json!(null),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::Top));
}

/// position.y = "bottom", size.height = "half" のみ指定 → TileKeyword::Bottom
#[test]
fn test_resolve_tile_keyword_bottom_half() {
    // 目的: 下半分パターンが正しく Bottom と判定されることを確認
    let position = Position {
        x: json!(null),
        y: json!("bottom"),
    };
    let size = Size {
        width: json!(null),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::Bottom));
}

/// x=left, y=top, width=half, height=half → TileKeyword::TopLeft（左上4分割）
#[test]
fn test_resolve_tile_keyword_top_left_quarter() {
    // 目的: 4分割パターン（左上）が正しく判定されることを確認
    let position = Position {
        x: json!("left"),
        y: json!("top"),
    };
    let size = Size {
        width: json!("half"),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::TopLeft));
}

/// x=right, y=top, width=half, height=half → TileKeyword::TopRight（右上4分割）
#[test]
fn test_resolve_tile_keyword_top_right_quarter() {
    // 目的: 4分割パターン（右上）が正しく判定されることを確認
    let position = Position {
        x: json!("right"),
        y: json!("top"),
    };
    let size = Size {
        width: json!("half"),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::TopRight));
}

/// x=left, y=bottom, width=half, height=half → TileKeyword::BottomLeft（左下4分割）
#[test]
fn test_resolve_tile_keyword_bottom_left_quarter() {
    // 目的: 4分割パターン（左下）が正しく判定されることを確認
    let position = Position {
        x: json!("left"),
        y: json!("bottom"),
    };
    let size = Size {
        width: json!("half"),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::BottomLeft));
}

/// x=right, y=bottom, width=half, height=half → TileKeyword::BottomRight（右下4分割）
#[test]
fn test_resolve_tile_keyword_bottom_right_quarter() {
    // 目的: 4分割パターン（右下）が正しく判定されることを確認
    let position = Position {
        x: json!("right"),
        y: json!("bottom"),
    };
    let size = Size {
        width: json!("half"),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::BottomRight));
}

// --- FullScreen（position は任意。size.width/height が両方 "max" のときのみ判定） ---

/// position が None でも size が width=max, height=max なら FullScreen と判定される
#[test]
fn test_resolve_tile_keyword_full_screen_with_position_none() {
    // 目的: FullScreen 判定は position の値に依存しないことを確認
    let size = Size {
        width: json!("max"),
        height: json!("max"),
    };

    let result = resolve_tile_keyword(None, Some(&size));
    assert_eq!(result, Some(TileKeyword::FullScreen));
}

/// position が left/top でも size が max/max なら FullScreen と判定される（表の「(任意)」列）
#[test]
fn test_resolve_tile_keyword_full_screen_with_position_left_top() {
    // 目的: 表の「(任意)」列の通り、position が具体的な値を持っていても
    //       size.width/height が両方 "max" であれば FullScreen が優先されることを確認
    let position = Position {
        x: json!("left"),
        y: json!("top"),
    };
    let size = Size {
        width: json!("max"),
        height: json!("max"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::FullScreen));
}

/// position が right/bottom でも size が max/max なら FullScreen と判定される
#[test]
fn test_resolve_tile_keyword_full_screen_with_position_right_bottom() {
    // 目的: position が反対側（right/bottom）でも FullScreen 判定に影響しないことを確認
    let position = Position {
        x: json!("right"),
        y: json!("bottom"),
    };
    let size = Size {
        width: json!("max"),
        height: json!("max"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::FullScreen));
}

/// position が数値指定（本来は他の判定なら None になるはずの値）でも、
/// size が max/max であれば FullScreen 判定がそれより優先されることを確認
///
/// ホワイトボックス観点: `resolve_tile_keyword` 冒頭の early return
/// （`width_is_max && height_is_max` の分岐）が、後続の x_is_left 等の
/// 判定処理より先に評価されるパスを検証する
#[test]
fn test_resolve_tile_keyword_full_screen_priority_over_numeric_position() {
    // 目的: FullScreen の early return が数値指定の position よりも優先されることを確認
    let position = Position {
        x: json!(100),
        y: json!(200),
    };
    let size = Size {
        width: json!("max"),
        height: json!("max"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, Some(TileKeyword::FullScreen));
}

// --- None系: "third" を含む場合（同値分割: 無効クラス） ---

/// size.width = "third" を含む場合は必ず None（4分割/半分どちらの表にも該当しない）
#[test]
fn test_resolve_tile_keyword_none_when_width_third() {
    // 目的: "third" 指定はOS標準タイリングのメニュー項目に対応しないため、
    //       常に None（直接プロパティ設定へのフォールバック対象）になることを確認
    let position = Position {
        x: json!("left"),
        y: json!(null),
    };
    let size = Size {
        width: json!("third"),
        height: json!(null),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// size.height = "third" を含む場合は必ず None
#[test]
fn test_resolve_tile_keyword_none_when_height_third() {
    // 目的: height 側の "third" 指定でも同様に None になることを確認
    let position = Position {
        x: json!(null),
        y: json!("top"),
    };
    let size = Size {
        width: json!(null),
        height: json!("third"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// width/height が両方 "third"（3分割グリッド指定）の場合も None
#[test]
fn test_resolve_tile_keyword_none_when_both_third() {
    // 目的: 3分割グリッド全体（例: 9分割レイアウトの1マス）は
    //       OS標準タイリングの対応表に存在しないため None になることを確認
    let position = Position {
        x: json!("left"),
        y: json!("top"),
    };
    let size = Size {
        width: json!("third"),
        height: json!("third"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

// --- None系: 数値指定を含む場合（同値分割: 無効クラス） ---

/// position.x が数値指定の場合は None
#[test]
fn test_resolve_tile_keyword_none_when_x_numeric() {
    // 目的: x がピクセル数値指定の場合、パターン指定ではないため None になることを確認
    let position = Position {
        x: json!(100),
        y: json!(null),
    };
    let size = Size {
        width: json!("half"),
        height: json!(null),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// position.y が数値指定の場合は None
#[test]
fn test_resolve_tile_keyword_none_when_y_numeric() {
    // 目的: y がピクセル数値指定の場合も同様に None になることを確認
    let position = Position {
        x: json!(null),
        y: json!(200),
    };
    let size = Size {
        width: json!(null),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// size.width が数値指定の場合は None
#[test]
fn test_resolve_tile_keyword_none_when_width_numeric() {
    // 目的: width がピクセル数値指定の場合、half/max のいずれでもないため None になることを確認
    let position = Position {
        x: json!("left"),
        y: json!(null),
    };
    let size = Size {
        width: json!(800),
        height: json!(null),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// size.height が数値指定の場合は None
#[test]
fn test_resolve_tile_keyword_none_when_height_numeric() {
    // 目的: height がピクセル数値指定の場合も同様に None になることを確認
    let position = Position {
        x: json!(null),
        y: json!("top"),
    };
    let size = Size {
        width: json!(null),
        height: json!(600),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// position/size のすべてのフィールドが数値指定（絶対座標指定）の場合は None
#[test]
fn test_resolve_tile_keyword_none_when_all_numeric() {
    // 目的: 従来の絶対座標指定（すべて数値）は完全にパターン指定と無関係なため
    //       None になり、直接プロパティ設定にフォールバックすることを確認
    let position = Position {
        x: json!(100),
        y: json!(200),
    };
    let size = Size {
        width: json!(800),
        height: json!(600),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

// --- None系: position/size が両方未指定の場合（境界値） ---

/// position: None, size: None の場合は None（境界値テスト）
#[test]
fn test_resolve_tile_keyword_none_when_both_none() {
    // 目的: position/size が両方とも指定されていない極端なケースで
    //       パニックせず None を返すことを確認（呼び出し側のガード漏れ対策）
    let result = resolve_tile_keyword(None, None);
    assert_eq!(result, None);
}

/// position/size 構造体自体は Some だが、内部フィールドが全て null の場合も None
#[test]
fn test_resolve_tile_keyword_none_when_fields_all_null() {
    // 目的: struct が Some だが中身が空（null_position/null_size）の場合も
    //       None を返すことを確認。`Option<&Position>` が None のケースと
    //       「フィールドが null」のケースの両方で同じ結果になることを保証する
    let position = null_position();
    let size = null_size();

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

// --- None系: 中途半端な組み合わせ（表にない組み合わせ、境界値分析） ---

/// width=half, height=half だが x/y が未指定の場合は None
///
/// ユーザー指定の重要な境界ケース: 4分割でも単純な左右半分でもない
/// 中途半端な指定（サイズは半分×半分だが、どちらの端に寄せるか不明）を検証する
#[test]
fn test_resolve_tile_keyword_none_when_half_half_without_position() {
    // 目的: size が half/half でも position（x/y）が未指定の場合は
    //       4分割位置が特定できないため None になることを確認
    let position = null_position();
    let size = Size {
        width: json!("half"),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// position 自体が None（構造体レベルで未指定）で size のみ half/half の場合も None
#[test]
fn test_resolve_tile_keyword_none_when_half_half_with_position_struct_none() {
    // 目的: 上のテストと同様の意図だが、position 構造体自体が Option::None の
    //       ケースでも同じ結果（None）になることを確認（構造体 None とフィールド null の等価性）
    let size = Size {
        width: json!("half"),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(None, Some(&size));
    assert_eq!(result, None);
}

/// size.width = "max" のみ指定（height は "max" でない）の場合は FullScreen 条件を
/// 満たさず、かつ他のどのパターンにも一致しないため None
#[test]
fn test_resolve_tile_keyword_none_when_width_max_only() {
    // 目的: FullScreen 判定には width と height の両方が "max" である必要があり、
    //       片方のみでは判定されないことを確認
    let position = null_position();
    let size = Size {
        width: json!("max"),
        height: json!("half"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// size.height = "max" のみ指定（width は "max" でない）の場合も None
#[test]
fn test_resolve_tile_keyword_none_when_height_max_only() {
    // 目的: height のみ "max" の場合も FullScreen 条件を満たさないことを確認
    let position = null_position();
    let size = Size {
        width: json!("half"),
        height: json!("max"),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// x=left, y=top, width=half だが height が未指定の場合は None
///
/// ホワイトボックス観点: TopLeft の完全一致パターンにも、Left のガード条件
/// （y_is_absent && height_is_absent）にも該当しない中間状態を検証する
#[test]
fn test_resolve_tile_keyword_none_when_left_top_half_width_without_height() {
    // 目的: y=top が指定されているため Left のガード（y_is_absent）を満たさず、
    //       かつ height が未指定のため TopLeft のパターン（height_is_half）にも
    //       一致しない、という表にない中途半端な組み合わせが None になることを確認
    let position = Position {
        x: json!("left"),
        y: json!("top"),
    };
    let size = Size {
        width: json!("half"),
        height: json!(null),
    };

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// x=left のみ指定で width が未指定の場合は None（half 指定なしでは Left にならない）
#[test]
fn test_resolve_tile_keyword_none_when_left_without_width_half() {
    // 目的: position.x = "left" だけでは不十分で、size.width = "half" が
    //       伴わない限り Left と判定されないことを確認
    let position = Position {
        x: json!("left"),
        y: json!(null),
    };
    let size = null_size();

    let result = resolve_tile_keyword(Some(&position), Some(&size));
    assert_eq!(result, None);
}

/// size が None（未指定）で position.x = "left" のみの場合も None
#[test]
fn test_resolve_tile_keyword_none_when_size_is_none() {
    // 目的: size 引数自体が None の場合、width_is_half は常に false と評価されるため
    //       いかなる position 指定でも None になることを確認
    let position = Position {
        x: json!("left"),
        y: json!(null),
    };

    let result = resolve_tile_keyword(Some(&position), None);
    assert_eq!(result, None);
}

/// position が None で size.width = "half" のみの場合も None
#[test]
fn test_resolve_tile_keyword_none_when_position_is_none_width_half() {
    // 目的: position 引数自体が None の場合、x_is_left/x_is_right は常に false と
    //       評価されるため、size だけが half でも Left/Right と判定されないことを確認
    let size = Size {
        width: json!("half"),
        height: json!(null),
    };

    let result = resolve_tile_keyword(None, Some(&size));
    assert_eq!(result, None);
}

// =============================================================================
// TileKeyword::menu_item_name() / is_submenu_item() Tests
// =============================================================================

/// 全バリアントで menu_item_name() が対応する日本語メニュー項目名を返すことを確認
#[test]
fn test_tile_keyword_menu_item_name_all_variants() {
    // 目的: 9つのバリアントすべてで、AppleScript側のメニュー項目名探索に使用する
    //       日本語文字列が正しく返されることを確認（表記が1文字でも違うとメニュー項目が
    //       見つからずフォールバックが発生してしまうため、厳密一致で検証する）
    assert_eq!(TileKeyword::Left.menu_item_name(), "左");
    assert_eq!(TileKeyword::Right.menu_item_name(), "右");
    assert_eq!(TileKeyword::Top.menu_item_name(), "上");
    assert_eq!(TileKeyword::Bottom.menu_item_name(), "下");
    assert_eq!(TileKeyword::TopLeft.menu_item_name(), "左上");
    assert_eq!(TileKeyword::TopRight.menu_item_name(), "右上");
    assert_eq!(TileKeyword::BottomLeft.menu_item_name(), "左下");
    assert_eq!(TileKeyword::BottomRight.menu_item_name(), "右下");
    assert_eq!(TileKeyword::FullScreen.menu_item_name(), "画面全体に表示");
}

/// is_submenu_item() が FullScreen のみ false、他の8バリアントは true を返すことを確認
#[test]
fn test_tile_keyword_is_submenu_item_all_variants() {
    // 目的: 「移動とサイズ変更」サブメニュー配下かどうかの判定が全バリアントで
    //       正しいことを確認。FullScreen だけが「ウインドウ」メニュー直下のため false。

    // サブメニュー配下の8項目はすべて true
    assert!(TileKeyword::Left.is_submenu_item());
    assert!(TileKeyword::Right.is_submenu_item());
    assert!(TileKeyword::Top.is_submenu_item());
    assert!(TileKeyword::Bottom.is_submenu_item());
    assert!(TileKeyword::TopLeft.is_submenu_item());
    assert!(TileKeyword::TopRight.is_submenu_item());
    assert!(TileKeyword::BottomLeft.is_submenu_item());
    assert!(TileKeyword::BottomRight.is_submenu_item());

    // FullScreen のみサブメニュー配下ではない（false）
    assert!(!TileKeyword::FullScreen.is_submenu_item());
}

/// TileKeyword が Copy/Clone/PartialEq/Eq/Debug を実装していることを確認
///
/// これらのトレイトは resolve_tile_keyword() の戻り値を assert_eq! で比較したり、
/// process_window() 内で `&TileKeyword` から値をコピーして再利用したりするために必要
#[test]
fn test_tile_keyword_traits() {
    // 目的: 派生トレイト（Debug, Clone, Copy, PartialEq, Eq）が正しく機能することを確認
    let original = TileKeyword::TopLeft;

    // Copy: 代入してもムーブされず、元の値がそのまま使える
    let copied = original;
    assert_eq!(original, copied);

    // Clone: 明示的な clone() も同じ値を返す
    let cloned = original;
    assert_eq!(original, cloned);

    // PartialEq: 異なるバリアント同士は等しくない
    assert_ne!(TileKeyword::Left, TileKeyword::Right);

    // Debug: フォーマット結果にバリアント名が含まれる
    let debug_str = format!("{:?}", TileKeyword::FullScreen);
    assert!(debug_str.contains("FullScreen"));
}
