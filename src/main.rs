use std::env;

#[derive(Copy, Clone, PartialEq)]
enum Unit {
    Temperature(TempUnit),
    Length(LengthUnit),
    Area(AreaUnit),
    Volume(VolUnit),
    Mass(MassUnit)
}

#[derive(Copy, Clone, PartialEq)]
enum TempUnit {
    Kelvin,
    Celsius,
    Fahrenheit,
}

#[derive(Copy, Clone, PartialEq)]
enum LengthUnit {
    Meter(i8),

    Inch,
    Feet,
    Yard,
    Mile
}

#[derive(Copy, Clone, PartialEq)]
enum AreaUnit {
    Meter2(i8),

    Inch2,
    Feet2,
    Yard2,
    Mile2,

    Acre,
    Hectare,
}

#[derive(Copy, Clone, PartialEq)]
enum VolUnit {
    Liter(i8),
    Meter3(i8),

    TeaSpoon,
    TableSpoon,
    Cup,
    Pint,
    Gallon
}

#[derive(Copy, Clone, PartialEq)]
enum MassUnit {
    Gram(i8),

    Ounce,
    Pound,
    Stone,
}


fn scale_to_string(scale: i8) -> String {
    match scale {
        -3 => "milli".to_string(),
        -2 => "centi".to_string(),
        3 => "kilo".to_string(),
        _ => "".to_string(),
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
                format!("square {}meter", prefix)
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
            VolUnit::TeaSpoon => "tea spoons".to_string(),
            VolUnit::TableSpoon => "table spoons".to_string(),
            VolUnit::Cup => "cups".to_string(),
            VolUnit::Pint => "pints".to_string(),
            VolUnit::Gallon => "gallons".to_string()
        }

        Unit::Mass(mass) => match mass {
            MassUnit::Gram(i) => {
                let prefix = scale_to_string(i);
                format!("{}grams", prefix)
            }
            MassUnit::Ounce => "ounce".to_string(),
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
                let scale = power_of(i);
                value * scale
            }
            VolUnit::Meter3(i) => {
                let scale = power_of(i*3);
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

fn convert_and_print_to(value: f32, a: Unit, b: Unit) {
    let str_a = unit_to_string(a);
    println!("{value} {str_a} equals to...");

    let str_b = unit_to_string(b);
    let std = convert_from_standard(value, a);
    let converted = convert_from_standard(std, b);
    println!("\t {converted} {str_b}");
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
            Unit::Length(LengthUnit::Meter(0)),
            Unit::Length(LengthUnit::Meter(3)),

            Unit::Length(LengthUnit::Inch),
            Unit::Length(LengthUnit::Feet),
            Unit::Length(LengthUnit::Yard),
            Unit::Length(LengthUnit::Mile),
        ],
        Unit::Area(_) => vec![
            Unit::Area(AreaUnit::Meter2(-3)),
            Unit::Area(AreaUnit::Meter2(-2)),
            Unit::Area(AreaUnit::Meter2(0)),
            Unit::Area(AreaUnit::Meter2(3)),

            Unit::Area(AreaUnit::Inch2),
            Unit::Area(AreaUnit::Feet2),
            Unit::Area(AreaUnit::Yard2),
            Unit::Area(AreaUnit::Mile2),
        ],
        Unit::Volume(_) => vec![
            Unit::Volume(VolUnit::Liter(-3)),
            Unit::Volume(VolUnit::Liter(0)),

            Unit::Volume(VolUnit::Meter3(-3)),
            Unit::Volume(VolUnit::Meter3(-2)),
            Unit::Volume(VolUnit::Meter3(0)),
            Unit::Volume(VolUnit::Meter3(3)),

            Unit::Volume(VolUnit::TeaSpoon),
            Unit::Volume(VolUnit::TableSpoon),
            Unit::Volume(VolUnit::Cup),
            Unit::Volume(VolUnit::Pint),
            Unit::Volume(VolUnit::Gallon),
        ],
        Unit::Mass(_) => vec![
            Unit::Mass(MassUnit::Gram(-3)),
            Unit::Mass(MassUnit::Gram(0)),
            Unit::Mass(MassUnit::Gram(3)),

            Unit::Mass(MassUnit::Ounce),
            Unit::Mass(MassUnit::Pound),
            Unit::Mass(MassUnit::Stone),
        ]
    }
}

fn convert_and_print_all(value: f32, a: Unit) {
    let str_a = unit_to_string(a);
    println!("{value} {str_a} equals to...");

    let units = fetch_all_units(a);
    for unit in units {
        if unit == a {
            continue;
        } else {
            let str_b = unit_to_string(unit);
            let std = convert_to_standard(value, a);
            let converted = convert_from_standard(std, unit);

            println!("\t {converted} {str_b}");
        }
    }
}

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

fn parse_unit(token: &str) -> Option<Unit> {
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

fn parser(mut args: env::Args) -> Task {
    // Drop the program name
    args.next();

    let tokens: Vec<String> = args.collect();
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
                if let (Some(a), Some(b)) = (parse_unit(&tokens[0]), parse_unit(&tokens[1])) {
                    Task::ConvertTo(1.0, a, b)
                } else {
                    Task::Error("Invalid unit(s)".to_string())
                }
            }
        }
        3 => {
            if let Ok(val) = tokens[0].parse::<f32>() {
                if let (Some(a), Some(b)) = (parse_unit(&tokens[1]), parse_unit(&tokens[2])) {
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
    let task = parser(env::args());
    match task {
        Task::Error(msg) => {
            println!("{msg}");
        }
        Task::Help => print_help(),
        Task::DisplayUnits => display_units(),
        Task::ConvertTo(value, a, b) => convert_and_print_to(value, a, b),
        Task::ConvertAll(value, a) => convert_and_print_all(value, a),
    }
}
