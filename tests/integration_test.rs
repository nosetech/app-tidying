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
    // version 2.0 は Issue #121/#122（tilingフィールド）によりサポート対象と
    // なったため、非対応バージョンとして 3.0 を使用する
    let json = r#"{
        "version": "3.0",
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

// =============================================================================
// tiling フィールドのJSON文字列経由デシリアライズテスト（Issue #121/#122）
//
// tests/config_test.rs のテストは AppWindowConfig を直接構築しているため、
// 「JSON上のキー存在で判定する」という設計の要となる serde のデシリアライズ挙動を
// 経由していない。以下はコードレビュー指摘に基づき、実際のJSON文字列を
// parse_layout_from_json() でパースする形で同じ観点を検証する。
// =============================================================================

#[test]
fn test_parse_config_tiling_valid_keyword() {
    // 目的: layout.json の tiling フィールドの文字列値が、実際のJSONパースを経由して
    //      AppWindowConfig::tiling に正しくデシリアライズされることを確認
    // 検証項目: version 2.0 + tiling のみ指定という組み合わせが構文検証を通過し、
    //          tiling フィールドの値が期待通りに読み込まれること

    let json = r#"{
        "version": "2.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Built-in",
                        "windows": [
                            { "app": "Google Chrome", "tiling": "top-left" }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_ok(), "{:?}", config.err());
    let cfg = config.unwrap();
    let window = &cfg.layouts[0].displays[0].windows[0];
    assert_eq!(window.tiling, Some("top-left".to_string()));
    assert!(window.position.is_none());
    assert!(window.size.is_none());
}

#[test]
fn test_parse_config_tiling_explicit_null_treated_as_none() {
    // 目的: JSON上で "tiling": null が明示的に指定された場合、None として
    //      デシリアライズされ、position/size との相互排他エラーの対象に
    //      ならないことを確認
    // 検証項目: tiling キーが存在していても値が null であれば「指定なし」として
    //          扱われること（position/size の x/y/width/height と異なり、
    //          tiling 自体は文字列フィールドのため部分指定という概念がない）

    let json = r#"{
        "version": "2.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Built-in",
                        "windows": [
                            {
                                "app": "Safari",
                                "tiling": null,
                                "position": { "x": "left", "y": "top" }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_ok(), "{:?}", config.err());
    let cfg = config.unwrap();
    let window = &cfg.layouts[0].displays[0].windows[0];
    assert_eq!(window.tiling, None);
}

#[test]
fn test_parse_config_tiling_and_position_with_null_subfields_conflict() {
    // 目的: CLAUDE.md「position/sizeとの相互排他制約」に明記されているエッジケース
    //      （position/size の中身がすべて null であっても、キー自体が存在すれば
    //      「指定あり」として扱われる）を、実際のJSONパース経由で確認する
    // 検証項目: "position": {"x": null, "y": null} のようにキーは存在するが
    //          値がすべて未指定のケースでも、tiling との同時指定エラーになること

    let json = r#"{
        "version": "2.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Built-in",
                        "windows": [
                            {
                                "app": "Safari",
                                "tiling": "left",
                                "position": { "x": null, "y": null }
                            }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config
        .unwrap_err()
        .message
        .contains("'tiling' と 'position'/'size' を同時に指定することはできません"));
}

#[test]
fn test_parse_config_tiling_invalid_keyword_via_json() {
    // 目的: JSON文字列経由で不正な tiling 値が指定された場合にエラーになることを確認
    // 検証項目: エラーメッセージに「無効な tiling 値」が含まれること

    let json = r#"{
        "version": "2.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Built-in",
                        "windows": [
                            { "app": "Safari", "tiling": "diagonal" }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config.unwrap_err().message.contains("無効な tiling 値"));
}

#[test]
fn test_parse_config_tiling_in_version_1_0_via_json() {
    // 目的: JSON文字列経由で version 1.0 の layout.json に tiling フィールドが
    //      混入した場合にエラーになることを確認
    // 検証項目: エラーメッセージに「version 1.0 では 'tiling' フィールドは
    //          サポートされていません」が含まれること

    let json = r#"{
        "version": "1.0",
        "layouts": [
            {
                "displays": [
                    {
                        "name": "Built-in",
                        "windows": [
                            { "app": "Safari", "tiling": "left" }
                        ]
                    }
                ]
            }
        ]
    }"#;

    let config = config::parse_layout_from_json(json);
    assert!(config.is_err());
    assert!(config
        .unwrap_err()
        .message
        .contains("version 1.0 では 'tiling' フィールドはサポートされていません"));
}
