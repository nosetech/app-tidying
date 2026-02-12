use apptidying::config;

#[test]
fn test_parse_valid_layout() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Built-in",
                        "windows": [
                            {
                                "app": "Google Chrome",
                                "position": { "x": 0, "y": 25 },
                                "size": { "width": 1440, "height": 900 }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let layout = config::parse_layout_from_json(json);
    assert!(layout.is_ok());
    let lyt = layout.unwrap();
    assert_eq!(lyt.version, "1.0");
    assert_eq!(lyt.layouts.len(), 1);
    assert_eq!(lyt.layouts[0].displays[0].name, "Built-in");
}

#[test]
fn test_parse_config_with_pattern_values() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": [
                            {
                                "app": "Finder",
                                "position": { "x": "left", "y": "top" },
                                "size": { "width": "half", "height": "max" }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_ok());
}

#[test]
fn test_parse_config_missing_version() {
    let json = r#"{
        "layouts": [
            {
                "displays": []
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
}

#[test]
fn test_parse_config_unsupported_version() {
    let json = r#"{
        "version": "2.0",
        "layouts": []
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config
        .unwrap_err()
        .message
        .contains("サポートされていないバージョン"));
}

#[test]
fn test_parse_config_empty_layouts() {
    let json = r#"{
        "version": "1.0",
        "layouts": []
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config
        .unwrap_err()
        .message
        .contains("layouts フィールドが空"));
}

#[test]
fn test_parse_config_empty_displays() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": []
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config.unwrap_err().message.contains("ディスプレイが空"));
}

#[test]
fn test_parse_config_empty_windows() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "name": "layout",
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": []
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config.unwrap_err().message.contains("ウィンドウが空"));
}

#[test]
fn test_parse_config_invalid_position_x() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": [
                            {
                                "app": "Finder",
                                "position": { "x": "invalid", "y": "top" }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config.unwrap_err().message.contains("無効な x 値"));
}

#[test]
fn test_parse_config_invalid_position_y() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": [
                            {
                                "app": "Finder",
                                "position": { "x": "left", "y": "invalid" }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config.unwrap_err().message.contains("無効な y 値"));
}

#[test]
fn test_parse_config_invalid_size_width() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": [
                            {
                                "app": "Finder",
                                "size": { "width": "invalid" }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config.unwrap_err().message.contains("無効な width 値"));
}

#[test]
fn test_parse_config_invalid_size_height() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": [
                            {
                                "app": "Finder",
                                "size": { "height": "invalid" }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config.unwrap_err().message.contains("無効な height 値"));
}

#[test]
fn test_parse_config_negative_coordinates() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": [
                            {
                                "app": "Finder",
                                "position": { "x": -10, "y": 0 }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config.unwrap_err().message.contains("が負です"));
}

#[test]
fn test_parse_config_zero_or_negative_size() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": [
                            {
                                "app": "Finder",
                                "size": { "width": 0 }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config.unwrap_err().message.contains("正の数値"));
}

#[test]
fn test_parse_config_multiple_layouts_and_displays() {
    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": [
                            {
                                "app": "App1"
                            }
                        ]
                    },
                    {
                        "name": "Display 2",
                        "windows": [
                            {
                                "app": "App2"
                            }
                        ]
                    }
                ]
            },
            {
                "displays": [
                    {
                        "name": "Display 1",
                        "windows": [
                            {
                                "app": "App3"
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_ok());
    let cfg = config.unwrap();
    assert_eq!(cfg.layouts.len(), 2);
    assert_eq!(cfg.layouts[0].displays.len(), 2);
    assert_eq!(cfg.layouts[1].displays.len(), 1);
}
