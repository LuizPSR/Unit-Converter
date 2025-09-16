use std::env;
use unit_converter::{AreaUnit, LengthUnit, MassUnit, TempUnit, Unit, VolUnit};
use unit_converter::{convert_and_print_to, convert_and_print_all, convert};
use unit_converter::TempUnit::{Celsius, Kelvin};

enum Task {
    Error(String),
    Help,
    DisplayUnits,
    ConvertTo(f32, Unit, Unit),
    ConvertAll(f32, Unit),
}

fn print_help() {
    println!("USAGE:");
    println!("  -h, --help              Display this help message");
    println!("  units                   Display all available units");
    println!("  [unit]                  Convert 1.0 in an unit to all other possible units");
    println!("  [value] [unit]          Convert a value in an unit to all other possible units");
    println!("  [unit] [unit]           Convert a 1.0 in unit A to unit B");
    println!("  [value] [unit] [unit]   Convert a value in unit A to unit B");
}

fn display_units() {
    println!("TEMPERATURE");
    println!("    K, kelvin");
    println!("    C, celsius");
    println!("    F, fahrenheit");

    println!("LENGTH");
    println!("    mm, millimeters");
    println!("    cm, centimeters");
    println!("    m, meters");
    println!("    km, kilometers");
    println!("    in, inches");
    println!("    ft, feet");
    println!("    yd, yards");
    println!("    mi, miles");

    println!("AREA");
    println!("    mm2, square millimeters");
    println!("    cm2, square centimeters");
    println!("    m2, square meters");
    println!("    km2, square kilometers");
    println!("    in2, square inches");
    println!("    ft2, sqft, square feet");
    println!("    yd2, square yards");
    println!("    mi2, square miles");
    println!("    ac, acres");
    println!("    ha, hectare");

    println!("VOLUME");
    println!("    ml, milliliter");
    println!("    l, liter");
    println!("    mm3, cubic millimeters");
    println!("    cm3, cubic centimeters");
    println!("    m3, cubic meters");
    println!("    teaspoons");
    println!("    tablespoons");
    println!("    cups");
    println!("    pt, pints");
    println!("    gal, gallons");

    println!("WEIGHT");
    println!("    mg, milligrams");
    println!("    g, grams");
    println!("    kg, kilograms");
    println!("    oz, ounces");
    println!("    lb, pounds");
    println!("    st, stones");
}

pub fn parse_unit(token: &str) -> Option<Unit> {
    match token.to_lowercase().as_str() {
        // Temperature
        "k" | "kelvin" => Some(Unit::Temperature(TempUnit::Kelvin)),
        "c" | "celsius" => Some(Unit::Temperature(TempUnit::Celsius)),
        "f" | "fahrenheit" => Some(Unit::Temperature(TempUnit::Fahrenheit)),

        // Length
        "mm" => Some(Unit::Length(LengthUnit::Meter(-3))),
        "cm" => Some(Unit::Length(LengthUnit::Meter(-2))),
        "m"  => Some(Unit::Length(LengthUnit::Meter(0))),
        "km" => Some(Unit::Length(LengthUnit::Meter(3))),
        "in" | "inch" | "inches" => Some(Unit::Length(LengthUnit::Inch)),
        "ft" | "feet" => Some(Unit::Length(LengthUnit::Feet)),
        "yd" | "yard" | "yards" => Some(Unit::Length(LengthUnit::Yard)),
        "mi" | "mile" | "miles" => Some(Unit::Length(LengthUnit::Mile)),

        // Area
        "mm2" => Some(Unit::Area(AreaUnit::Meter2(-3))),
        "cm2" => Some(Unit::Area(AreaUnit::Meter2(-2))),
        "m2"  => Some(Unit::Area(AreaUnit::Meter2(0))),
        "km2" => Some(Unit::Area(AreaUnit::Meter2(3))),
        "in2" => Some(Unit::Area(AreaUnit::Inch2)),
        "ft2" | "sqft" => Some(Unit::Area(AreaUnit::Feet2)),
        "yd2" => Some(Unit::Area(AreaUnit::Yard2)),
        "mi2" => Some(Unit::Area(AreaUnit::Mile2)),
        "ac" | "acre" | "acres" => Some(Unit::Area(AreaUnit::Acre)),
        "ha" | "hectare" | "hectares" => Some(Unit::Area(AreaUnit::Hectare)),

        // Volume
        "ml" => Some(Unit::Volume(VolUnit::Liter(-3))),
        "l" | "liter" | "liters" => Some(Unit::Volume(VolUnit::Liter(0))),
        "mm3" => Some(Unit::Volume(VolUnit::Meter3(-3))),
        "cm3" => Some(Unit::Volume(VolUnit::Meter3(-2))),
        "m3" => Some(Unit::Volume(VolUnit::Meter3(0))),
        "teaspoon" | "teaspoons" => Some(Unit::Volume(VolUnit::TeaSpoon)),
        "tablespoon" | "tablespoons" => Some(Unit::Volume(VolUnit::TableSpoon)),
        "cup" | "cups" => Some(Unit::Volume(VolUnit::Cup)),
        "pt" | "pint" | "pints" => Some(Unit::Volume(VolUnit::Pint)),
        "gal" | "gallon" | "gallons" => Some(Unit::Volume(VolUnit::Gallon)),

        // Mass
        "mg" => Some(Unit::Mass(MassUnit::Gram(-3))),
        "g"  | "gram" | "grams" => Some(Unit::Mass(MassUnit::Gram(0))),
        "kg" => Some(Unit::Mass(MassUnit::Gram(3))),
        "oz" | "ounce" | "ounces" => Some(Unit::Mass(MassUnit::Ounce)),
        "lb" | "pound" | "pounds" => Some(Unit::Mass(MassUnit::Pound)),
        "st" | "stone" | "stones" => Some(Unit::Mass(MassUnit::Stone)),

        _ => None,
    }
}

fn parser(tokens: Vec<String>) -> Task {
    if tokens.is_empty() {
        return Task::Help;
    }

    // Handle help flags
    if tokens[0] == "-h" || tokens[0] == "--help" {
        return Task::Help;
    }

    // Handle units
    if tokens[0] == "units" {
        return Task::DisplayUnits;
    }

    match tokens.len() {
        1 => {
            if let Some(unit) = parse_unit(&tokens[0]) {
                Task::ConvertAll(1.0, unit)
            } else {
                Task::Error(format!("Unknown unit '{}'", tokens[0]))
            }
        }
        2 => {
            // Case A: number + unit
            if let Ok(val) = tokens[0].parse::<f32>() {
                if let Some(unit) = parse_unit(&tokens[1]) {
                    Task::ConvertAll(val, unit)
                } else {
                    Task::Error(format!("Unknown unit '{}'", tokens[1]))
                }
            } else {
                // Case B: unit + unit
                if let (Some(a), Some(b)) = (
                    parse_unit(&tokens[0]),
                    parse_unit(&tokens[1])
                ) {
                    Task::ConvertTo(1.0, a, b)
                } else {
                    Task::Error("Invalid unit(s)".to_string())
                }
            }
        }
        3 => {
            if let Ok(val) = tokens[0].parse::<f32>() {
                if let (Some(a), Some(b)) = (
                    parse_unit(&tokens[1]),
                    parse_unit(&tokens[2])
                ) {
                    Task::ConvertTo(val, a, b)
                } else {
                    Task::Error("Invalid unit(s)".to_string())
                }
            } else {
                Task::Error("First argument must be a number".to_string())
            }
        }
        _ => Task::Error("Too many arguments".to_string()),
    }
}

fn main() {
    let args = env::args().skip(1);
    let task = parser(args.collect());
    match task {
        Task::Error(msg) => {
            println!("{msg}");
        }
        Task::Help => print_help(),
        Task::DisplayUnits => display_units(),
        Task::ConvertTo(value, a, b) => convert_and_print_to(value, a, b),
        Task::ConvertAll(value, a) => convert_and_print_all(value, a),
    }
    println!("{}", convert(0.0, Unit::Temperature(Celsius), Unit::Temperature(Kelvin)));
}

#[cfg(test)]
mod tests {
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
    fn test_parse_invalid_units() {
        assert_eq!(parse_unit("ledsago"), None);
        assert_eq!(parse_unit(""), None);
        assert_eq!(parse_unit("newtons"), None);
    }

    #[test]
    fn test_parser_no_arg() {
        // No args -> Help
        let args = vec![];
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
            Task::Error(msg) => assert!(msg.contains("Unknown unit")),
            _ => panic!("Expected Task::Error for unknown unit"),
        }
    }

    #[test]
    fn test_parser_non_numeric_value() {
        let args = vec!["abc".to_string(), "m".to_string(), "ft".to_string()];
        match parser(args) {
            Task::Error(msg) => assert!(msg.contains("First argument must be a number")),
            _ => panic!("Expected Task::Error for non-numeric value"),
        }
    }

    #[test]
    fn test_parser_invalid_units_pair() {
        let args = vec!["m".to_string(), "foobar".to_string()];
        match parser(args) {
            Task::Error(msg) => assert!(msg.contains("Invalid unit")),
            _ => panic!("Expected Task::Error for invalid unit pair"),
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
            _ => panic!("Expected Task::Error for too many args"),
        }
    }
}
