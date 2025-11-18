use std::io::Cursor;
use unit_converter::run;

#[test]
fn test_empty_input() {

    let mut cursor = Cursor::new(Vec::new());
    run(Vec::new(), &mut cursor);
    let out = String::from_utf8(cursor.into_inner()).unwrap();

    // no command should default to --help
    assert!(out.contains("USAGE"))
}

#[test]
fn test_asking_for_help() {

    let mut cursor = Cursor::new(Vec::new());
    run(vec!["-h".to_string()], &mut cursor);
    let out = String::from_utf8(cursor.into_inner()).unwrap();
    assert!(out.contains("USAGE"));

    cursor = Cursor::new(Vec::new());
    run(vec!["--help".to_string()], &mut cursor);
    let out = String::from_utf8(cursor.into_inner()).unwrap();
    assert!(out.contains("USAGE"))
}

#[test]
fn test_units() {

    let mut cursor = Cursor::new(Vec::new());
    run(vec!["units".to_string()], &mut cursor);
    let out = String::from_utf8(cursor.into_inner()).unwrap();

    // should show all implemented units

    // Check for section headers and a few specific units to confirm structure
    assert!(out.contains("TEMPERATURE\n"));
    assert!(out.contains("kelvin"));
    assert!(out.contains("LENGTH\n"));
    assert!(out.contains("feet"));
    assert!(out.contains("AREA\n"));
    assert!(out.contains("ha"));
    assert!(out.contains("VOLUME\n"));
    assert!(out.contains("gal"));
    assert!(out.contains("WEIGHT\n"));
    assert!(out.contains("kg"));
}

#[test]
fn test_number_unit() {

    let mut cursor = Cursor::new(Vec::new());
    run(
        vec![
            "1.5".to_string(),
            "meters".to_string()
        ],
        &mut cursor
    );
    let out = String::from_utf8(cursor.into_inner()).unwrap();

    // should convert to all compatible

    assert!(out.contains("feet"));
    assert!(out.contains("yards"));
    assert!(out.contains("miles"));
}

#[test]
fn test_number_unit_unit() {

    let mut cursor = Cursor::new(Vec::new());
    run(
        vec![
            "0.5".to_string(),
            "Liters".to_string(),
            "cubic_centimeters".to_string()
        ],
        &mut cursor
    );
    let out = String::from_utf8(cursor.into_inner()).unwrap();

    // should convert x in unit to unit b
    assert!(out.contains("cubic centimeters"))
}

#[test]
fn test_convert_between_incompatible_units() {

    let mut cursor = Cursor::new(Vec::new());
    run(
        vec![
            "0.5".to_string(),
            "liters".to_string(),
            "fahrenheit".to_string()
        ],
        &mut cursor
    );
    let out = String::from_utf8(cursor.into_inner()).unwrap();

    // should write an error
    assert!(out.contains("Trying"))
}

#[test]
fn test_convert_to_same_unit() {

    let mut cursor = Cursor::new(Vec::new());
    run(
        vec![
            "0.5".to_string(),
            "Liters".to_string(),
            "Liters".to_string()
        ],
        &mut cursor
    );
    let out = String::from_utf8(cursor.into_inner()).unwrap();

    // should write an error
    assert!(out.contains("Trying"));
}

#[test]
fn test_invalid_units() {

    // case 1: [unit]

    let mut cursor = Cursor::new(Vec::new());
    run(
        vec![
            "Frances_of_floating_thrash".to_string(),
        ],
        &mut cursor
    );
    let out = String::from_utf8(cursor.into_inner()).unwrap();
    assert!(out.contains("Invalid"));

    // case 2: [value] [unit]

    let mut cursor = Cursor::new(Vec::new());
    run(
        vec![
            "Frances_of_floating_thrash".to_string(),
        ],
        &mut cursor
    );
    let out = String::from_utf8(cursor.into_inner()).unwrap();
    assert!(out.contains("Invalid"));

    // case 3: [unit] [unit]

    let mut cursor = Cursor::new(Vec::new());
    run(
        vec![
            "sqft".to_string(),
            "Frances_of_floating_thrash".to_string(),
        ],
        &mut cursor
    );
    let out = String::from_utf8(cursor.into_inner()).unwrap();
    assert!(out.contains("Invalid"));

    // case 4: [value] [unit] [unit]

    let mut cursor = Cursor::new(Vec::new());
    run(
        vec![
            "12480".to_string(),
            "sqft".to_string(),
            "Frances_of_floating_thrash".to_string(),
        ],
        &mut cursor
    );
    let out = String::from_utf8(cursor.into_inner()).unwrap();
    assert!(out.contains("Invalid"));
}

#[test]
fn test_too_many_args() {

    let mut cursor = Cursor::new(Vec::new());
    run(
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ],
        &mut cursor
    );

    let out = String::from_utf8(cursor.into_inner()).unwrap();
    assert!(out.contains("Too many"));
}