use super::{
    *,
    Unit::*,
    TempUnit::*,
    LengthUnit::*,
    AreaUnit::*,
    VolUnit::*,
    MassUnit::*,
};

#[test]
fn test_convert_incompatible_units_error() {
    let result = convert(1.0, Mass(Gram(0)), Temperature(Kelvin));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Trying to convert incompatible units".to_string());
}

#[test]
fn test_scaled_units_to_string() {
    assert_eq!(unit_to_string(Area(Meter2(-3))), "square millimeters".to_string());
    assert_eq!(unit_to_string(Volume(Liter(0))), "liters".to_string());
    assert_eq!(unit_to_string(Length(Meter(-9))), "nanometers".to_string());
    assert_eq!(unit_to_string(Length(Meter(3))), "kilometers".to_string());
}

#[test]
fn test_non_standard_scaled_unit_to_string() {
    assert_eq!(unit_to_string(Length(Meter(4))), "[10 to the power of 4] meters".to_string());
}

#[test]
fn test_unscaled_units_to_string() {
    assert_eq!(unit_to_string(Temperature(Fahrenheit)), "fahrenheit".to_string());
    assert_eq!(unit_to_string(Length(Inch)), "inches".to_string());
    assert_eq!(unit_to_string(Area(Acre)), "acres".to_string());
    assert_eq!(unit_to_string(Volume(TableSpoon)), "tablespoons".to_string());
    assert_eq!(unit_to_string(Mass(Stone)), "stones".to_string());
}

#[test]
fn test_power_of() {
    // Positive powers
    assert!((power_of(0) - 1.0).abs() < f32::EPSILON);
    assert!((power_of(1) - 10.0).abs() < f32::EPSILON);
    assert!((power_of(3) - 1000.0).abs() < f32::EPSILON);

    // Negative powers
    assert!((power_of(-1) - 0.1).abs() < f32::EPSILON);
    assert!((power_of(-2) - 0.01).abs() < f32::EPSILON);
    assert!((power_of(-3) - 0.001).abs() < f32::EPSILON);
}

#[test]
fn test_compatibility_check() {
    assert_eq!(check_compatibility(Area(Acre), Area(Hectare)), true);
    assert_eq!(check_compatibility(Temperature(Fahrenheit), Mass(Pound)), false);
    assert_eq!(check_compatibility(Length(Feet), Length(Mile)), true); // test two imperial length units
    assert_eq!(check_compatibility(Volume(Liter(0)), Area(Acre)), false); // test volume vs area
}

#[test]
fn test_fetch_units() {
    let unit = Temperature(Kelvin);
    let fetch = fetch_all_units(unit);
    assert_eq!(fetch.len(), 3);
    for x in fetch {
        assert_eq!(check_compatibility(unit, x), true);
    }

    let unit = Length(Meter(0));
    let fetch = fetch_all_units(unit);
    assert_eq!(fetch.len(), 8);
    for x in fetch {
        assert_eq!(check_compatibility(unit, x), true);
    }

    let unit = Area(Feet2);
    let fetch = fetch_all_units(unit);
    assert_eq!(fetch.len(), 9);
    for x in fetch {
        assert_eq!(check_compatibility(unit, x), true);
    }

    let unit = Volume(Cup);
    let fetch = fetch_all_units(unit);
    assert_eq!(fetch.len(), 9);
    for x in fetch {
        assert_eq!(check_compatibility(unit, x), true);
    }

    let unit = Mass(Ounce);
    let fetch = fetch_all_units(unit);
    assert_eq!(fetch.len(), 6);
    for x in fetch {
        assert_eq!(check_compatibility(unit, x), true);
    }
}
