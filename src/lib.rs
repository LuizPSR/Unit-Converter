use std::io::Write;

pub mod units;
mod parser;

use units::*;
use parser::*;

pub fn print_help<W: Write>(writer: &mut W) {
    writeln!(writer, "USAGE:").unwrap();
    writeln!(writer, "  -h, --help              Display this help message").unwrap();
    writeln!(writer, "  units                   Display all available units").unwrap();
    writeln!(writer, "  [unit]                  Convert 1.0 in an unit to all other possible units").unwrap();
    writeln!(writer, "  [value] [unit]          Convert a value in an unit to all other possible units").unwrap();
    writeln!(writer, "  [unit] [unit]           Convert a 1.0 in unit A to unit B").unwrap();
    writeln!(writer, "  [value] [unit] [unit]   Convert a value in unit A to unit B").unwrap();
}

fn display_all_units<W: Write>(writer: &mut W) {
    writeln!(writer, "TEMPERATURE").unwrap();
    writeln!(writer, "    K, kelvin").unwrap();
    writeln!(writer, "    C, celsius").unwrap();
    writeln!(writer, "    F, fahrenheit").unwrap();

    writeln!(writer, "LENGTH").unwrap();
    writeln!(writer, "    mm, millimeter, millimeters").unwrap();
    writeln!(writer, "    cm, centimeter, centimeters").unwrap();
    writeln!(writer, "    m, meter, meters").unwrap();
    writeln!(writer, "    km, kilometer, kilometers").unwrap();
    writeln!(writer, "    in, inch, inches").unwrap();
    writeln!(writer, "    ft, foot, feet").unwrap();
    writeln!(writer, "    yd, yard, yards").unwrap();
    writeln!(writer, "    mi, mile, miles").unwrap();

    writeln!(writer, "AREA").unwrap();
    writeln!(writer, "    mm2, square_millimeter, square_millimeters, mm^2, mmˆ2").unwrap();
    writeln!(writer, "    cm2, square_centimeter, square_centimeters, cm^2, cmˆ2").unwrap();
    writeln!(writer, "    m2, square_meter, square_meters, m^2, mˆ2").unwrap();
    writeln!(writer, "    km2, square_kilometer, square_kilometers, km^2, kmˆ2").unwrap();
    writeln!(writer, "    in2, square_inch, square_inches, in^2, inˆ2").unwrap();
    writeln!(writer, "    ft2, sqft, square_foot, square_feet, ft^2, ftˆ2").unwrap();
    writeln!(writer, "    yd2, square_yard, square_yards, yd^2, ydˆ2").unwrap();
    writeln!(writer, "    mi2, square_mile, square_miles, mi^2, miˆ2").unwrap();
    writeln!(writer, "    ac, acre, acres").unwrap();
    writeln!(writer, "    ha, hectare, hectares").unwrap();

    writeln!(writer, "VOLUME").unwrap();
    writeln!(writer, "    ml, milliliter, milliliters").unwrap();
    writeln!(writer, "    l, liter, liters").unwrap();
    writeln!(writer, "    mm3, cubic_millimeter, cubic_millimeters, mm^3, mmˆ3").unwrap();
    writeln!(writer, "    cm3, cubic_centimeter, cubic_centimeters, cm^3, cmˆ3").unwrap();
    writeln!(writer, "    m3, cubic_meter, cubic_meters, m^3, mˆ3").unwrap();
    writeln!(writer, "    teaspoon, teaspoons").unwrap();
    writeln!(writer, "    tablespoon, tablespoons").unwrap();
    writeln!(writer, "    cup, cups").unwrap();
    writeln!(writer, "    pt, pint, pints").unwrap();
    writeln!(writer, "    gal, gallon, gallons").unwrap();

    writeln!(writer, "WEIGHT").unwrap();
    writeln!(writer, "    mg, milligram, milligrams").unwrap();
    writeln!(writer, "    g, gram, grams").unwrap();
    writeln!(writer, "    kg, kilogram, kilograms").unwrap();
    writeln!(writer, "    oz, ounce, ounces").unwrap();
    writeln!(writer, "    lb, pound, pounds").unwrap();
    writeln!(writer, "    st, stone, stones").unwrap();
}

pub fn run<W: Write>(args: Vec<String>, writer: &mut W) {
    let task = parser(args);

    match task {
        Task::Error(msg) => {
            writeln!(writer, "{msg}").unwrap();
        }
        Task::Help => print_help(writer),
        Task::DisplayUnits => display_all_units(writer),
        Task::ConvertTo(value, a, b) => convert_and_print_to(writer, value, a, b),
        Task::ConvertAll(value, a) => convert_and_print_all(writer, value, a),
    }
}

fn scale_to_string(scale: i8) -> String {
    match scale {
        -9 => "nano".to_string(),
        -6 => "micro".to_string(),
        -3 => "milli".to_string(),
        -2 => "centi".to_string(),
        0 => "".to_string(),
        3 => "kilo".to_string(),
        6 => "".to_string(),
        9 => "giga".to_string(),
        12 => "tera".to_string(),
        15 => "penta".to_string(),
        _ => format!("[10 to the power of {scale} of a] ").to_string(),
    }
}

fn unit_to_string(unit: Unit) -> String {
    match unit {
        Unit::Temperature(temp) => match temp {
            TempUnit::Kelvin => "kelvin".to_string(),
            TempUnit::Celsius => "celsius".to_string(),
            TempUnit::Fahrenheit => "fahrenheit".to_string(),
        }

        Unit::Length(length) => match length {
            LengthUnit::Meter(i) => {
                let prefix = scale_to_string(i);
                format!("{}meters", prefix)
            }
            LengthUnit::Inch => "inches".to_string(),
            LengthUnit::Feet => "feet".to_string(),
            LengthUnit::Yard => "yards".to_string(),
            LengthUnit::Mile => "miles".to_string(),
        }

        Unit::Area(area) => match area {
            AreaUnit::Meter2(i) => {
                let prefix = scale_to_string(i);
                format!("square {}meters", prefix)
            }
            AreaUnit::Inch2 => "square inches".to_string(),
            AreaUnit::Feet2 => "square feet".to_string(),
            AreaUnit::Yard2 => "square yards".to_string(),
            AreaUnit::Mile2 => "square miles".to_string(),
            AreaUnit::Acre => "acres".to_string(),
            AreaUnit::Hectare => "hectares".to_string(),
        }

        Unit::Volume(vol) => match vol {
            VolUnit::Liter(i) => {
                let prefix = scale_to_string(i);
                format!("{}liters", prefix)
            }
            VolUnit::Meter3(i) => {
                let prefix = scale_to_string(i);
                format!("cubic {}meters", prefix)
            }
            VolUnit::TeaSpoon => "teaspoons".to_string(),
            VolUnit::TableSpoon => "tablespoons".to_string(),
            VolUnit::Cup => "cups".to_string(),
            VolUnit::Pint => "pints".to_string(),
            VolUnit::Gallon => "gallons".to_string()
        }

        Unit::Mass(mass) => match mass {
            MassUnit::Gram(i) => {
                let prefix = scale_to_string(i);
                format!("{}grams", prefix)
            }
            MassUnit::Ounce => "ounces".to_string(),
            MassUnit::Pound => "pounds".to_string(),
            MassUnit::Stone => "stones".to_string(),
        }
    }
}

fn power_of(mut i: i8) -> f32 {
    let mut scale = 1.0;
    if i < 0 {
        while i < 0 {
            scale *= 0.1;
            i += 1;
        }
    } else if i > 0 {
        while i > 0 {
            scale *= 10.0;
            i -= 1;
        }
    }

    scale
}

fn convert_to_standard(value: f32, unit: Unit) -> f32 {
    match unit {
        Unit::Temperature(temp) => match temp {
            TempUnit::Kelvin => value,
            TempUnit::Celsius => value + 273.15,
            TempUnit::Fahrenheit => (value - 32.0) / 1.8 + 273.15,
        }

        Unit::Length(length) => match length {
            LengthUnit::Meter(i) => {
                let scale = power_of(i);
                value * scale
            }

            LengthUnit::Inch => value * 0.0254,
            LengthUnit::Feet => value * 0.3048,
            LengthUnit::Yard => value * 0.9144,
            LengthUnit::Mile => value * 1609.344,
        }

        Unit::Area(area) => match area {
            AreaUnit::Meter2(i) => {
                let scale = power_of(i*2);
                value * scale
            }

            AreaUnit::Inch2 => value * 0.0254 * 0.0254,
            AreaUnit::Feet2 => value * 0.3048 * 0.3048,
            AreaUnit::Yard2 => value * 0.9144 * 0.9144,
            AreaUnit::Mile2 => value * 1609.344 * 1609.344,
            AreaUnit::Acre => value * 4046.8564224,
            AreaUnit::Hectare => value * 10000.0,
        }

        Unit::Volume(vol) => match vol {
            VolUnit::Liter(i) => {
                let scale = power_of(i);
                value * scale
            }
            VolUnit::Meter3(i) => {
                let scale = power_of(i*3);
                value * scale * 1000.0
            }
            VolUnit::TeaSpoon => value * 0.005,
            VolUnit::TableSpoon => value * 0.015,
            VolUnit::Cup => value * 0.2365882365,
            VolUnit::Pint => value * 0.473176473,
            VolUnit::Gallon => value * 3.785411784,
        }

        Unit::Mass(mass) => match mass {
            MassUnit::Gram(i) => {
                let scale = power_of(i);
                value * scale
            }
            MassUnit::Ounce => value * 28.34952,
            MassUnit::Pound => value * 453.59237,
            MassUnit::Stone => value * 453.59237 * 14.0,
        }
    }
}

fn convert_from_standard(value: f32, unit: Unit) -> f32 {
    match unit {
        Unit::Temperature(temp) => match temp {
            TempUnit::Kelvin => value,
            TempUnit::Celsius => value - 273.15,
            TempUnit::Fahrenheit => (value * 1.8) - 459.67,
        }

        Unit::Length(length) => match length {
            LengthUnit::Meter(i) => {
                let scale = power_of(-i);
                value * scale
            }

            LengthUnit::Inch => value / 0.0254,
            LengthUnit::Feet => value / 0.3048,
            LengthUnit::Yard => value / 0.9144,
            LengthUnit::Mile => value / 1609.344,
        }

        Unit::Area(area) => match area {
            AreaUnit::Meter2(i) => {
                let scale = power_of(-i*2);
                value * scale
            }

            AreaUnit::Inch2 => value / (0.0254 * 0.0254),
            AreaUnit::Feet2 => value / (0.3048 * 0.3048),
            AreaUnit::Yard2 => value / (0.9144 * 0.9144),
            AreaUnit::Mile2 => value / (1609.344 * 1609.344),
            AreaUnit::Acre => value / 4046.8564224,
            AreaUnit::Hectare => value / 10000.0,
        }

        Unit::Volume(vol) => match vol {
            VolUnit::Liter(i) => {
                let scale = power_of(-i);
                value * scale
            }
            VolUnit::Meter3(i) => {
                let scale = power_of(-i*3);
                value * scale * 0.001
            }
            VolUnit::TeaSpoon => value / 0.005,
            VolUnit::TableSpoon => value / 0.015,
            VolUnit::Cup => value / 0.2365882365,
            VolUnit::Pint => value / 0.473176473,
            VolUnit::Gallon => value / 3.785411784,
        }

        Unit::Mass(mass) => match mass {
            MassUnit::Gram(i) => {
                let scale = power_of(-i);
                value * scale
            }
            MassUnit::Ounce => value / 28.34952,
            MassUnit::Pound => value / 453.59237,
            MassUnit::Stone => value / (453.59237 * 14.0),
        }
    }
}

fn check_compatibility(a: Unit, b: Unit) -> bool {
    match (a, b) {
        (Unit::Temperature(_), Unit::Temperature(_)) => true,
        (Unit::Length(_), Unit::Length(_)) => true,
        (Unit::Area(_), Unit::Area(_)) => true,
        (Unit::Volume(_), Unit::Volume(_)) => true,
        (Unit::Mass(_), Unit::Mass(_)) => true,

        _ => false
    }
}

pub fn convert(value: f32, a: Unit, b: Unit) -> Result<f32, String> {
    // check compatibility
    if !check_compatibility(a, b) {
        return Err("Trying to convert incompatible units".to_string());
    };


    if a == b {
        return Err("Trying to convert a unit into itself".to_string());
    }

    // convert
    Ok(convert_from_standard(
        convert_to_standard(value, a),
        b
    ))
}

pub fn convert_and_print_to<W: Write>(writer: &mut W, value: f32, a: Unit, b: Unit) {
    match convert(value, a, b) {
        Ok(converted) => {
            let str_a = unit_to_string(a);
            writeln!(writer, "{value:.6} {str_a} equals to...").unwrap();

            let str_b = unit_to_string(b);
            writeln!(writer, "\t {converted:.6} {str_b}").unwrap();
        },
        Err(msg) => {
            writeln!(writer, "{msg}").unwrap();
        }
    }
}

fn fetch_all_units(unit: Unit) -> Vec<Unit> {
    match unit {
        Unit::Temperature(_) => vec![
            Unit::Temperature(TempUnit::Kelvin),
            Unit::Temperature(TempUnit::Celsius),
            Unit::Temperature(TempUnit::Fahrenheit)
        ],
        Unit::Length(_) => vec![
            Unit::Length(LengthUnit::Meter(-3)),
            Unit::Length(LengthUnit::Meter(-2)),
            Unit::Length(LengthUnit::Meter( 0)),
            Unit::Length(LengthUnit::Meter( 3)),

            Unit::Length(LengthUnit::Inch),
            Unit::Length(LengthUnit::Feet),
            Unit::Length(LengthUnit::Yard),
            Unit::Length(LengthUnit::Mile),
        ],
        Unit::Area(_) => vec![
            Unit::Area(AreaUnit::Meter2(-2)),
            Unit::Area(AreaUnit::Meter2(0)),
            Unit::Area(AreaUnit::Meter2(3)),

            Unit::Area(AreaUnit::Inch2),
            Unit::Area(AreaUnit::Feet2),
            Unit::Area(AreaUnit::Yard2),
            Unit::Area(AreaUnit::Mile2),

            Unit::Area(AreaUnit::Acre),
            Unit::Area(AreaUnit::Hectare)
        ],
        Unit::Volume(_) => vec![
            Unit::Volume(VolUnit::Liter(0)),
            Unit::Volume(VolUnit::Liter(-3)),

            Unit::Volume(VolUnit::Meter3(0)),
            Unit::Volume(VolUnit::Meter3(-2)),

            Unit::Volume(VolUnit::TeaSpoon),
            Unit::Volume(VolUnit::TableSpoon),
            Unit::Volume(VolUnit::Cup),
            Unit::Volume(VolUnit::Pint),
            Unit::Volume(VolUnit::Gallon),
        ],
        Unit::Mass(_) => vec![
            Unit::Mass(MassUnit::Gram(-3)),
            Unit::Mass(MassUnit::Gram( 0)),
            Unit::Mass(MassUnit::Gram( 3)),

            Unit::Mass(MassUnit::Ounce),
            Unit::Mass(MassUnit::Pound),
            Unit::Mass(MassUnit::Stone),
        ]
    }
}

pub fn convert_and_print_all<W: Write>(writer: &mut W, value: f32, a: Unit) {
    let str_a = unit_to_string(a);
    writeln!(writer, "{value:.6} {str_a} equals to...").unwrap();

    let units = fetch_all_units(a);
    for unit in units {
        if unit == a {
            continue;
        } else {
            let str_b = unit_to_string(unit);

            let converted = convert(value, a, unit).ok().unwrap();

            writeln!(writer, "\t {converted:.6} {str_b}").unwrap();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{
        *,
        Unit::*,
        TempUnit::*,
        LengthUnit::*,
        AreaUnit::*,
        VolUnit::*,
        MassUnit::*,
    };
    use std::io::Cursor; // Helper for reading the buffer

    #[test]
    fn test_convert_and_print_to_output() {
        let mut output = Cursor::new(Vec::new());

        // Test conversion: 1 meter to feet
        let value = 1.0;
        let from_unit = Length(Meter(0));
        let to_unit = Length(Feet);

        convert_and_print_to(&mut output, value, from_unit, to_unit);

        let output_string = String::from_utf8(output.into_inner()).unwrap();

        // Expected: 1 meter ≈ 3.280839895 feet (1 / 0.3048)
        let expected_converted = 3.280840; // The value rounded to 6 decimal places

        assert!(output_string.contains("1.000000 meters equals to..."));
        assert!(output_string.contains(&format!("\t {:.6} feet", expected_converted)));
    }

    #[test]
    fn test_convert_and_print_all_output() {
        let mut output = Cursor::new(Vec::new());

        let value = 1.0;
        let from_unit = Mass(Gram(3));

        convert_and_print_all(&mut output, value, from_unit);

        let output_string = String::from_utf8(output.into_inner()).unwrap();

        assert!(output_string.contains("1.000000 kilograms equals to..."));
        // 1 kg = 1000 g
        assert!(output_string.contains("\t 1000.000000 grams"));
    }

    #[test]
    fn test_convert_and_print_incompatible_error_output() {
        let mut output = Cursor::new(Vec::new());

        let value = 1.0;
        let from_unit = Mass(Gram(3));
        let to_unit = Temperature(Celsius);

        convert_and_print_to(&mut output, value, from_unit, to_unit);

        let output_string = String::from_utf8(output.into_inner()).unwrap();

        assert_eq!(output_string.trim(), "Trying to convert incompatible units");
    }

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

        // defaults to...
        assert_eq!(unit_to_string(Length(Meter(4))), "[10 to the power of 4 of a] meters".to_string());
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

    #[test]
    fn test_display_all_units_to_output() {
        let mut output = Cursor::new(Vec::new());
        display_all_units(&mut output);
        let output_string = String::from_utf8(output.into_inner()).unwrap();

        // Check for section headers and a few specific units to confirm structure
        assert!(output_string.contains("TEMPERATURE\n"));
        assert!(output_string.contains("kelvin"));
        assert!(output_string.contains("LENGTH\n"));
        assert!(output_string.contains("feet"));
        assert!(output_string.contains("AREA\n"));
        assert!(output_string.contains("ha"));
        assert!(output_string.contains("VOLUME\n"));
        assert!(output_string.contains("gal"));
        assert!(output_string.contains("WEIGHT\n"));
        assert!(output_string.contains("kg"));
    }
}