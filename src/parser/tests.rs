
use super::*;

#[test]
fn test_parse_units() {
    assert_eq!(parse_unit("cm"), Some(Unit::Length(LengthUnit::Meter(-2))));
    assert_eq!(parse_unit("km"), Some(Unit::Length(LengthUnit::Meter(3))));
    assert_eq!(parse_unit("m"), Some(Unit::Length(LengthUnit::Meter(0))));

    assert_eq!(parse_unit("teaspoon"), Some(Unit::Volume(VolUnit::TeaSpoon)));
    assert_eq!(parse_unit("gal"), Some(Unit::Volume(VolUnit::Gallon)));

    assert_eq!(parse_unit("F"), Some(Unit::Temperature(TempUnit::Fahrenheit)));
}

#[test]
fn test_nonsensical_unit() {
    assert_eq!(parse_unit("ledsago"), None);
    assert_eq!(parse_unit(""), None);
}

#[test]
fn test_parse_unavailable_units() {
    assert_eq!(parse_unit("newtons"), None); // force
    assert_eq!(parse_unit("mph"), None);     // speed
    assert_eq!(parse_unit("seconds"), None); // time
    assert_eq!(parse_unit("feet3"), None);   // unimplemented volume
}

#[test]
fn test_parser_no_arg() {
    // No args -> Help
    let args = vec![];
    assert!(matches!(parser(args), Task::Help));
}

#[test]
fn test_parser_help_flags() {
    // -h
    let args = vec!["-h".to_string()];
    assert!(matches!(parser(args), Task::Help));

    // --help
    let args = vec!["--help".to_string()];
    assert!(matches!(parser(args), Task::Help));
}

#[test]
fn test_parser_display_units() {
    // Units
    let args = vec!["units".to_string()];
    assert!(matches!(parser(args), Task::DisplayUnits));
}

#[test]
fn test_parser_convert_single_unit() {
    // Single unit
    let args = vec!["m".to_string()];
    if let Task::ConvertAll(val, unit) = parser(args) {
        assert_eq!(val, 1.0);
        assert_eq!(unit, Unit::Length(LengthUnit::Meter(0)));
    } else {
        panic!("Expected ConvertAll");
    }

    // Value + unit
    let args = vec!["42".to_string(), "km".to_string()];
    if let Task::ConvertAll(val, unit) = parser(args) {
        assert_eq!(val, 42.0);
        assert_eq!(unit, Unit::Length(LengthUnit::Meter(3)));
    } else {
        panic!("Expected ConvertAll");
    }
}

#[test]
fn test_parser_convert_from_a_to_b() {
    // Unit + unit
    let args = vec!["m".to_string(), "ft".to_string()];
    if let Task::ConvertTo(val, a, b) = parser(args) {
        assert_eq!(val, 1.0);
        assert_eq!(a, Unit::Length(LengthUnit::Meter(0)));
        assert_eq!(b, Unit::Length(LengthUnit::Feet));
    } else {
        panic!("Expected ConvertTo");
    }

    // Value + unit + unit
    let args = vec!["100".to_string(), "c".to_string(), "f".to_string()];
    if let Task::ConvertTo(val, a, b) = parser(args) {
        assert_eq!(val, 100.0);
        assert_eq!(a, Unit::Temperature(TempUnit::Celsius));
        assert_eq!(b, Unit::Temperature(TempUnit::Fahrenheit));
    } else {
        panic!("Expected ConvertTo");
    }
}

#[test]
fn test_parser_unknown_unit() {
    let args = vec!["foobar".to_string()];
    match parser(args) {
        Task::Error(msg) => assert!(msg.contains("Invalid unit 'foobar'")),
        _ => panic!("Expected Task::Error for unknown unit"),
    }
}

#[test]
fn test_parser_invalid_unit_single_arg() {
    let args = vec!["foobar".to_string()];
    match parser(args) {
        Task::Error(msg) => assert!(msg.contains("Invalid unit 'foobar'")),
        _ => panic!("Expected Task::Error for unknown unit"),
    }
}

#[test]
fn test_parser_invalid_unit_value_unit() {
    let args = vec!["10".to_string(), "foobar".to_string()];
    match parser(args) {
        Task::Error(msg) => assert!(msg.contains("Invalid unit 'foobar'")),
        _ => panic!("Expected Task::Error for unknown unit"),
    }
}

#[test]
fn test_parser_non_numeric_value() {
    let args = vec!["abc".to_string(), "m".to_string(), "ft".to_string()];
    match parser(args) {
        Task::Error(msg) => assert!(msg.contains("Invalid format")),
        _ => panic!("Expected Task::Error for non-numeric value in usage [value] [unit] [unit]"),
    }
}

#[test]
fn test_parser_invalid_units_pair() {
    // Unit + invalid unit
    let args = vec!["m".to_string(), "foobar".to_string()];
    match parser(args) {
        Task::Error(msg) => assert!(msg.contains("Invalid unit 'foobar'")),
        _ => panic!("Expected Task::Error for invalid unit pair (target)"),
    }

    // Invalid unit + valid unit
    let args = vec!["foobar".to_string(), "m".to_string()];
    match parser(args) {
        Task::Error(msg) => assert!(msg.contains("Invalid unit 'foobar'")),
        _ => panic!("Expected Task::Error for invalid unit pair (source)"),
    }
}

#[test]
fn test_parser_invalid_unit_value_a_to_b() {
    // Value + valid unit + invalid unit (Target unit error)
    let args = vec!["1.5".to_string(), "m".to_string(), "foobar".to_string()];
    match parser(args) {
        Task::Error(msg) => assert!(msg.contains("Invalid unit 'foobar'")),
        _ => panic!("Expected Task::Error for invalid target unit in 3 args"),
    }
    // Value + invalid unit + valid unit
    let args = vec!["1.5".to_string(), "foobar".to_string(), "m".to_string()];
    match parser(args) {
        Task::Error(msg) => assert!(msg.contains("Invalid unit 'foobar'")),
        _ => panic!("Expected Task::Error for invalid source unit in 3 args"),
    }
}

#[test]
fn test_parser_too_many_args() {
    let args = vec![
        "1".to_string(),
        "m".to_string(),
        "ft".to_string(),
        "extra".to_string(),
    ];
    match parser(args) {
        Task::Error(msg) => assert!(msg.contains("Too many arguments")),
        _ => panic!("Expected Task::Error for too many arguments"),
    }
}
