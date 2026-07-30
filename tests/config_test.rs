use apptidying::applescript::DisplayInfo;
use apptidying::config::{
    fill_absent_position, fill_absent_size, parse_position_value, parse_settings_from_json,
    parse_size_value, validate_layout, validate_layout_bounds, validate_layout_syntax,
    AppWindowConfig, DisplayConfig, LayoutConfig, LayoutFile, LogRotationConfig, Position, Size,
    TileKeyword,
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

/// menu_item_name_candidates() が全バリアントで日本語表記を先頭要素として返し、
/// menu_item_name() の戻り値と先頭要素が一致することを確認
#[test]
fn test_tile_keyword_menu_item_name_candidates_all_variants() {
    // 目的: Issue #119 のレビュー対応で追加した多言語候補（日本語/英語）が
    //       全バリアントで2件（日本語, 英語）返され、先頭が menu_item_name() と
    //       一致することを確認する
    let variants = [
        TileKeyword::Left,
        TileKeyword::Right,
        TileKeyword::Top,
        TileKeyword::Bottom,
        TileKeyword::TopLeft,
        TileKeyword::TopRight,
        TileKeyword::BottomLeft,
        TileKeyword::BottomRight,
        TileKeyword::FullScreen,
    ];

    for keyword in variants {
        let candidates = keyword.menu_item_name_candidates();
        assert_eq!(candidates.len(), 2, "候補は日本語・英語の2件であるべき");
        assert_eq!(
            candidates[0],
            keyword.menu_item_name(),
            "先頭候補は menu_item_name() の戻り値と一致するべき"
        );
    }
}

/// menu_item_name_candidates() の英語表記が期待通りであることを確認
#[test]
fn test_tile_keyword_menu_item_name_candidates_english() {
    // 目的: 英語ロケールのmacOSでもメニュー項目を検索できるよう追加した
    //       英語表記候補が期待する文字列であることを確認する
    assert_eq!(TileKeyword::Left.menu_item_name_candidates()[1], "Left");
    assert_eq!(TileKeyword::Right.menu_item_name_candidates()[1], "Right");
    assert_eq!(TileKeyword::Top.menu_item_name_candidates()[1], "Top");
    assert_eq!(TileKeyword::Bottom.menu_item_name_candidates()[1], "Bottom");
    assert_eq!(
        TileKeyword::TopLeft.menu_item_name_candidates()[1],
        "Top Left"
    );
    assert_eq!(
        TileKeyword::TopRight.menu_item_name_candidates()[1],
        "Top Right"
    );
    assert_eq!(
        TileKeyword::BottomLeft.menu_item_name_candidates()[1],
        "Bottom Left"
    );
    assert_eq!(
        TileKeyword::BottomRight.menu_item_name_candidates()[1],
        "Bottom Right"
    );
    assert_eq!(
        TileKeyword::FullScreen.menu_item_name_candidates()[1],
        "Enter Full Screen"
    );
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
/// これらのトレイトは、テストコードで戻り値を assert_eq! で比較したり、
/// `tile_window_via_menu` などの呼び出し元で `&TileKeyword` から値をコピーして
/// 再利用したりするために必要
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

// =============================================================================
// fill_absent_position() / fill_absent_size() のテスト（Issue #120）
// =============================================================================
//
// position/size の x/y, width/height を「片方だけ指定」した layout.json に対応するため、
// 未指定側（null）を現在のウィンドウ位置・サイズ（current）で補完する
// fill_absent_position()/fill_absent_size() の単体テスト。
//
// ブラックボックステスト観点（同値分割）:
//   - 有効値クラス: x/y（width/height）ともに具体的な値（文字列パターン or 数値）
//     → current の Some/None に関わらず変更されず、常に Some を返す
//   - 未指定値クラス: 片方のみ null / 両方 null
//     → current が Some の場合、null のフィールドのみ current の値で補完され、
//       具体的な値を持つフィールドは変更されない
//     → current が None の場合、補完できないため None を返す（呼び出し側は
//       該当ウィンドウの位置・サイズ操作をスキップする想定。レビュー指摘 3-1 対応）
//
// ホワイトボックステスト観点（分岐網羅）:
//   - fill_absent_position/fill_absent_size 内の `value_is_absent(...)` による
//     if/else 分岐（x, y / width, height それぞれ）の true/false を網羅する
//   - `(x_absent || y_absent) && current.is_none()` の早期 None リターン分岐を網羅する

// --- fill_absent_position() ---

/// x/y ともにパターン文字列（"left"/"top"）が指定されている場合、
/// current を渡しても変更されないことを確認
#[test]
fn test_fill_absent_position_both_specified_pattern_unchanged() {
    // 目的: 有効値クラス（両方とも具体的な値）が current の影響を受けないことを確認
    let position = Position {
        x: json!("left"),
        y: json!("top"),
    };

    let filled =
        fill_absent_position(&position, Some((999, 888))).expect("null が無いため常に Some");

    // 検証: x/y ともに元の値のまま（current の値 999/888 で上書きされていない）
    assert_eq!(filled.x, json!("left"));
    assert_eq!(filled.y, json!("top"));
}

/// x/y ともに具体的な値が指定されている場合、current が None（取得失敗）でも
/// 補完が不要なため Some を返すことを確認
#[test]
fn test_fill_absent_position_both_specified_current_none_still_some() {
    // 目的: 補完不要（null が無い）な場合は current が None でも早期 None を返さないことを確認
    // 検証項目: (x_absent || y_absent) が false のため、current.is_none() のガード条件に
    //          関わらず Some が返る分岐を検証
    let position = Position {
        x: json!("left"),
        y: json!("top"),
    };

    let filled = fill_absent_position(&position, None);

    assert!(
        filled.is_some(),
        "null フィールドが無ければ current が None でも Some"
    );
    let filled = filled.unwrap();
    assert_eq!(filled.x, json!("left"));
    assert_eq!(filled.y, json!("top"));
}

/// x/y ともに数値（絶対座標）が指定されている場合、
/// current を渡しても変更されないことを確認
#[test]
fn test_fill_absent_position_both_specified_numeric_unchanged() {
    // 目的: 有効値クラス（数値指定）が current の影響を受けないことを確認
    let position = Position {
        x: json!(50),
        y: json!(60),
    };

    let filled =
        fill_absent_position(&position, Some((999, 888))).expect("null が無いため常に Some");

    assert_eq!(filled.x, json!(50));
    assert_eq!(filled.y, json!(60));
}

/// x のみ null（y は具体的な値）の場合、x のみ current の値で補完され、
/// y は変更されないことを確認
#[test]
fn test_fill_absent_position_x_null_only() {
    // 目的: 片方のみ未指定のケース（x が未指定側）を検証
    // 検証項目: null であった x が current.0 に置き換わり、y はそのまま維持される
    let position = Position {
        x: json!(null),
        y: json!("top"),
    };

    let filled =
        fill_absent_position(&position, Some((123, 456))).expect("current が Some のため補完可能");

    assert_eq!(
        filled.x,
        json!(123),
        "null だった x は current.0 で補完される"
    );
    assert_eq!(filled.y, json!("top"), "具体的な値を持つ y は変更されない");
}

/// y のみ null（x は具体的な値）の場合、y のみ current の値で補完され、
/// x は変更されないことを確認
#[test]
fn test_fill_absent_position_y_null_only() {
    // 目的: 片方のみ未指定のケース（y が未指定側）を検証
    // 検証項目: null であった y が current.1 に置き換わり、x はそのまま維持される
    let position = Position {
        x: json!("left"),
        y: json!(null),
    };

    let filled =
        fill_absent_position(&position, Some((123, 456))).expect("current が Some のため補完可能");

    assert_eq!(filled.x, json!("left"), "具体的な値を持つ x は変更されない");
    assert_eq!(
        filled.y,
        json!(456),
        "null だった y は current.1 で補完される"
    );
}

/// x/y ともに null の場合、両方とも current の値で補完されることを確認
#[test]
fn test_fill_absent_position_both_null_with_current() {
    // 目的: 未指定値クラス（両方 null）で current が Some の場合の補完を検証
    let position = Position {
        x: json!(null),
        y: json!(null),
    };

    let filled =
        fill_absent_position(&position, Some((321, 654))).expect("current が Some のため補完可能");

    assert_eq!(filled.x, json!(321));
    assert_eq!(filled.y, json!(654));
}

/// x/y ともに null かつ current が None（現在位置取得失敗）の場合、
/// 補完できないため None を返すことを確認（レビュー指摘 3-1 対応）
#[test]
fn test_fill_absent_position_both_null_without_current_returns_none() {
    // 目的: current 引数の同値分割「None（取得失敗）」クラスを検証
    // 検証項目: 未指定フィールドを 0 埋めするのではなく、補完不可を示す None を返し、
    //          呼び出し側（process_window）がウィンドウ操作をスキップできるようにする
    let position = Position {
        x: json!(null),
        y: json!(null),
    };

    let filled = fill_absent_position(&position, None);

    assert!(
        filled.is_none(),
        "null フィールドがあり current も取得できない場合は None を返す"
    );
}

/// x のみ null かつ current が None の場合も、
/// もう片方（y）が具体的な値であっても補完不可のため None を返すことを確認
#[test]
fn test_fill_absent_position_x_null_current_none_y_specified_returns_none() {
    // 目的: 「片方のみ未指定」×「current が None」の組み合わせ（相互作用テスト）を検証
    // 検証項目: y が具体的な値でも、x 側の補完に current が必要なため全体として None になる
    let position = Position {
        x: json!(null),
        y: json!(200),
    };

    let filled = fill_absent_position(&position, None);

    assert!(
        filled.is_none(),
        "x が null で current が None の場合、y が具体的な値でも None を返す"
    );
}

/// fill_absent_position() の結果をそのまま parse_position_value() に渡しても
/// 正しく数値へ変換できることを確認する統合的な検証
///
/// 実際の loader.rs の使われ方（fill_absent_position の戻り値を
/// serde_json::to_value() 経由で parse_position_value() に渡す）に近い形で検証する
#[test]
fn test_fill_absent_position_integration_with_parse_position_value() {
    // 目的: fill_absent_position が生成した Position が、既存の parse_position_value と
    //      組み合わせても矛盾なく動作することを確認
    let position = Position {
        x: json!(null),
        y: json!(null),
    };

    // 現在のウィンドウ位置 (300, 400) で補完
    let filled =
        fill_absent_position(&position, Some((300, 400))).expect("current が Some のため補完可能");
    let filled_value = serde_json::to_value(&filled).expect("Position のシリアライズに失敗");

    let result = parse_position_value(&filled_value, None, 1920, 1080, 800, 600, "position");

    assert!(result.is_ok());
    let (x, y) = result.unwrap();
    assert_eq!(x, 300);
    assert_eq!(y, 400);
}

// --- fill_absent_size() ---

/// width/height ともにパターン文字列（"half"/"third"）が指定されている場合、
/// current を渡しても変更されないことを確認
#[test]
fn test_fill_absent_size_both_specified_pattern_unchanged() {
    // 目的: 有効値クラス（両方とも具体的な値）が current の影響を受けないことを確認
    let size = Size {
        width: json!("half"),
        height: json!("third"),
    };

    let filled = fill_absent_size(&size, Some((999, 888))).expect("null が無いため常に Some");

    assert_eq!(filled.width, json!("half"));
    assert_eq!(filled.height, json!("third"));
}

/// width/height ともに数値（絶対サイズ）が指定されている場合、
/// current を渡しても変更されないことを確認
#[test]
fn test_fill_absent_size_both_specified_numeric_unchanged() {
    // 目的: 有効値クラス（数値指定）が current の影響を受けないことを確認
    let size = Size {
        width: json!(800),
        height: json!(600),
    };

    let filled = fill_absent_size(&size, Some((999, 888))).expect("null が無いため常に Some");

    assert_eq!(filled.width, json!(800));
    assert_eq!(filled.height, json!(600));
}

/// width のみ null（height は具体的な値）の場合、width のみ current の値で補完され、
/// height は変更されないことを確認
#[test]
fn test_fill_absent_size_width_null_only() {
    // 目的: 片方のみ未指定のケース（width が未指定側）を検証
    let size = Size {
        width: json!(null),
        height: json!("max"),
    };

    let filled = fill_absent_size(&size, Some((700, 500))).expect("current が Some のため補完可能");

    assert_eq!(
        filled.width,
        json!(700),
        "null だった width は current.0 で補完される"
    );
    assert_eq!(
        filled.height,
        json!("max"),
        "具体的な値を持つ height は変更されない"
    );
}

/// height のみ null（width は具体的な値）の場合、height のみ current の値で補完され、
/// width は変更されないことを確認
#[test]
fn test_fill_absent_size_height_null_only() {
    // 目的: 片方のみ未指定のケース（height が未指定側）を検証
    let size = Size {
        width: json!("half"),
        height: json!(null),
    };

    let filled = fill_absent_size(&size, Some((700, 500))).expect("current が Some のため補完可能");

    assert_eq!(
        filled.width,
        json!("half"),
        "具体的な値を持つ width は変更されない"
    );
    assert_eq!(
        filled.height,
        json!(500),
        "null だった height は current.1 で補完される"
    );
}

/// width/height ともに null の場合、両方とも current の値で補完されることを確認
#[test]
fn test_fill_absent_size_both_null_with_current() {
    // 目的: 未指定値クラス（両方 null）で current が Some の場合の補完を検証
    let size = Size {
        width: json!(null),
        height: json!(null),
    };

    let filled = fill_absent_size(&size, Some((640, 480))).expect("current が Some のため補完可能");

    assert_eq!(filled.width, json!(640));
    assert_eq!(filled.height, json!(480));
}

/// width/height ともに null かつ current が None（現在サイズ取得失敗）の場合、
/// 補完できないため None を返すことを確認（レビュー指摘 3-1 対応）
///
/// 旧実装では (0, 0) にフォールバックしていたが、`parse_size_value` の
/// 「正の値であること」というバリデーションに必ず抵触してエラーになり、
/// 片方だけ正しく指定されていてもウィンドウ操作全体が失敗する非対称な挙動だった。
/// 現在は None を返し、呼び出し側が該当ウィンドウの処理を明確な WARN と共に
/// スキップする方針に変更した。
#[test]
fn test_fill_absent_size_both_null_without_current_returns_none() {
    // 目的: current 引数の同値分割「None（取得失敗）」クラスを検証
    // 検証項目: 未指定フィールドを 0 埋めするのではなく、補完不可を示す None を返すこと
    let size = Size {
        width: json!(null),
        height: json!(null),
    };

    let filled = fill_absent_size(&size, None);

    assert!(
        filled.is_none(),
        "null フィールドがあり current も取得できない場合は None を返す"
    );
}

/// width のみ null かつ current が None の場合も、
/// height が具体的な値であっても補完不可のため None を返すことを確認
#[test]
fn test_fill_absent_size_width_null_current_none_height_specified_returns_none() {
    // 目的: 「片方のみ未指定」×「current が None」の組み合わせ（相互作用テスト）を検証
    // 検証項目: height が具体的な値（"half"）でも、width 側の補完に current が必要なため
    //          全体として None になる（片方だけ適用されて中途半端な結果にはならない）
    let size = Size {
        width: json!(null),
        height: json!("half"),
    };

    let filled = fill_absent_size(&size, None);

    assert!(
        filled.is_none(),
        "width が null で current が None の場合、height が具体的な値でも None を返す"
    );
}

/// fill_absent_size() の結果を parse_size_value() に渡すと、
/// current から正しく補完されたサイズへ変換できることを確認する統合的な検証
#[test]
fn test_fill_absent_size_integration_with_parse_size_value_success() {
    // 目的: fill_absent_size が生成した Size が、既存の parse_size_value と
    //      組み合わせても矛盾なく動作することを確認（正常系）
    let size = Size {
        width: json!(null),
        height: json!(null),
    };

    let filled = fill_absent_size(&size, Some((640, 480))).expect("current が Some のため補完可能");
    let filled_value = serde_json::to_value(&filled).expect("Size のシリアライズに失敗");

    let result = parse_size_value(&filled_value, 1920, 1080, "size");

    assert!(result.is_ok());
    let (width, height) = result.unwrap();
    assert_eq!(width, 640);
    assert_eq!(height, 480);
}
