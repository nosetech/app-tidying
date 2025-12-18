use apptidying::config::{parse_position_value, parse_size_value};
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
