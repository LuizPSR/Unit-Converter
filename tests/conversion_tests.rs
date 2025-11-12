use unit_converter::{
    convert,
    units::{Unit::*,
     TempUnit::*,
     LengthUnit::*,
     AreaUnit::*,
     VolUnit::*,
     MassUnit::*, }
};

#[test]
fn test_conversion_temperature() {
    // 0 °C = 273.15 K
    let c_to_k = convert(0.0, Temperature(Celsius), Temperature(Kelvin)).ok().unwrap();
    assert!((c_to_k - 273.15).abs() < 1e-3);

    // 100 °C = 212 °F
    let c_to_f = convert(100.0, Temperature(Celsius), Temperature(Fahrenheit)).ok().unwrap();
    assert!((c_to_f - 212.0).abs() < 1e-3);

    // 32 °F = 0 °C
    let f_to_c = convert(32.0, Temperature(Fahrenheit), Temperature(Celsius)).ok().unwrap();
    assert!((f_to_c - 0.0).abs() < 1e-3);
}

#[test]
fn test_conversion_length() {
    // 1 meter = 100 cm
    let cm = convert(1.0, Length(Meter(0)), Length(Meter(-2))).ok().unwrap();
    assert!((cm - 100.0).abs() < 1e-3);

    // 1 kilometer = 1000 meters
    let m = convert(1.0, Length(Meter(3)), Length(Meter(0))).ok().unwrap();
    assert!((m - 1000.0).abs() < 1e-3);

    // 1 mile ≈ 1609.344 meters
    let m = convert(1.0, Length(Mile), Length(Meter(0))).ok().unwrap();
    assert!((m - 1609.344).abs() < 1e-3);
}

#[test]
fn test_conversion_length_imperial() {
    // 1 foot = 12 inches
    let inches = convert(1.0, Length(Feet), Length(Inch)).ok().unwrap();
    assert!((inches - 12.0).abs() < 1e-3);

    // 1 yard = 3 feet
    let feet = convert(1.0, Length(Yard), Length(Feet)).ok().unwrap();
    assert!((feet - 3.0).abs() < 1e-3);

    // 1 inch ≈ 2.54 cm
    let cm = convert(1.0, Length(Inch), Length(Meter(-2))).ok().unwrap();
    assert!((cm - 2.54).abs() < 1e-3);
}

#[test]
fn test_conversion_area() {
    // 1 m² = 10,000 cm²
    let cm2 = convert(1.0, Area(Meter2(0)), Area(Meter2(-2))).ok().unwrap();
    assert!((cm2 - 10_000.0).abs() < 1e-3);

    // 1 km² = 1,000,000 m²
    let m2 = convert(1.0, Area(Meter2(3)), Area(Meter2(0))).ok().unwrap();
    assert!((m2 - 1_000_000.0).abs() < 1e-3);

    // 1 acre ≈ 4046.8564224 m²
    let m2 = convert(1.0, Area(Acre), Area(Meter2(0))).ok().unwrap();
    assert!((m2 - 4046.8564224).abs() < 1e-3);

    // 1 hectare = 10,000 m²
    let m2 = convert(1.0, Area(Hectare), Area(Meter2(0))).ok().unwrap();
    assert!((m2 - 10_000.0).abs() < 1e-3);
}

#[test]
fn test_conversion_area_milli_centi() {
    // 1 m² = 1,000,000 mm²
    let mm2 = convert(1.0, Area(Meter2(0)), Area(Meter2(-3))).ok().unwrap();
    assert!((mm2 - 1_000_000.0).abs() < 1e-3);

    // 1 cm² = 100 mm²
    let mm2_from_cm2 = convert(1.0, Area(Meter2(-2)), Area(Meter2(-3))).ok().unwrap();
    assert!((mm2_from_cm2 - 100.0).abs() < 1e-3);
}

#[test]
fn test_conversion_volume() {
    // 1 liter = 1000 ml
    let ml = convert(1.0, Volume(Liter(0)), Volume(Liter(-3))).ok().unwrap();
    assert!((ml - 1000.0).abs() < 1e-3);

    // 1 m³ = 1000 liters
    let l = convert(1.0, Volume(Meter3(0)), Volume(Liter(0))).ok().unwrap();
    assert!((l - 1000.0).abs() < 1e-3);

    // 1 cup ≈ 0.236588 liters
    let l = convert(1.0, Volume(Cup), Volume(Liter(0))).ok().unwrap();
    assert!((l - 0.2365882365).abs() < 1e-3);

    // 1 gallon ≈ 3.78541 liters
    let l = convert(1.0, Volume(Gallon), Volume(Liter(0))).ok().unwrap();
    assert!((l - 3.785411784).abs() < 1e-3);
}

#[test]
fn test_conversion_volume_cooking() {
    // 1 tablespoon = 3 teaspoons
    let teaspoons = convert(1.0, Volume(TableSpoon), Volume(TeaSpoon)).ok().unwrap();
    assert!((teaspoons - 3.0).abs() < 1e-3);

    // 1 cup ≈ 15.77 tablespoons (based on defined conversion factors)
    let tablespoons = convert(1.0, Volume(Cup), Volume(TableSpoon)).ok().unwrap();
    assert!((tablespoons - 15.772549).abs() < 1e-3);
}

#[test]
fn test_conversion_mass() {
    // 1 kg = 1000 g
    let g = convert(1.0, Mass(Gram(3)), Mass(Gram(0))).ok().unwrap();
    assert!((g - 1000.0).abs() < 1e-3);

    // 1 g = 1000 mg
    let mg = convert(1.0, Mass(Gram(0)), Mass(Gram(-3))).ok().unwrap();
    assert!((mg - 1000.0).abs() < 1e-3);

    // 1 pound ≈ 453.59237 g
    let g = convert(1.0, Mass(Pound), Mass(Gram(0))).ok().unwrap();
    assert!((g - 453.59237).abs() < 1e-3);

    // 1 stone = 14 pounds = 6350.29318 g
    let g = convert(1.0, Mass(Stone), Mass(Gram(0))).ok().unwrap();
    assert!((g - 6350.29318).abs() < 1e-3);
}

#[test]
fn test_conversion_mass_imperial() {
    // 1 pound = 16 ounces
    let oz = convert(1.0, Mass(Pound), Mass(Ounce)).ok().unwrap();
    assert!((oz - 16.0).abs() < 1e-3);

    // 1 stone = 14 pounds
    let lb = convert(1.0, Mass(Stone), Mass(Pound)).ok().unwrap();
    assert!((lb - 14.0).abs() < 1e-3);
}

#[test]
fn test_converting_incompatible_units() {
    let g = convert(1.0, Mass(Stone), Temperature(Kelvin)).is_ok();
    assert_eq!(g, false);

    let g = convert(1.0, Mass(Stone), Length(Meter(3))).is_ok();
    assert_eq!(g, false);

    let g = convert(1.0, Area(Meter2(3)), Mass(Gram(0))).is_ok();
    assert_eq!(g, false);

    let g = convert(1.0, Volume(Liter(0)), Mass(Gram(0))).is_ok();
    assert_eq!(g, false);
}

#[test]
fn test_converting_compatible_units() {
    let g = convert(1.0, Temperature(Celsius), Temperature(Kelvin)).is_ok();
    assert_eq!(g, true);

    let g = convert(1.0, Length(Feet), Length(Meter(3))).is_ok();
    assert_eq!(g, true);

    let g = convert(1.0, Area(Meter2(3)), Area(Feet2)).is_ok();
    assert_eq!(g, true);

    let g = convert(1.0, Volume(Liter(0)), Volume(Meter3(0))).is_ok();
    assert_eq!(g, true);

    let g = convert(1.0, Mass(Stone), Mass(Gram(0))).is_ok();
    assert_eq!(g, true);
}