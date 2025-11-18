#[cfg(test)]
mod tests;

use crate::units::{AreaUnit, LengthUnit, MassUnit, TempUnit, Unit, VolUnit};

pub enum Task {
    Error(String),
    Help,
    DisplayUnits,
    ConvertTo(f32, Unit, Unit),
    ConvertAll(f32, Unit),
}

pub fn parse_unit(token: &str) -> Option<Unit> {
    match token.to_lowercase().as_str() {
        // Temperature
        "k" | "kelvin" => Some(Unit::Temperature(TempUnit::Kelvin)),
        "c" | "celsius" => Some(Unit::Temperature(TempUnit::Celsius)),
        "f" | "fahrenheit" => Some(Unit::Temperature(TempUnit::Fahrenheit)),

        // Length
        "mm" | "millimeter" | "millimeters" => Some(Unit::Length(LengthUnit::Meter(-3))),
        "cm" | "centimeter" | "centimeters" => Some(Unit::Length(LengthUnit::Meter(-2))),
        "m" | "meter" | "meters" => Some(Unit::Length(LengthUnit::Meter(0))),
        "km" | "kilometer" | "kilometers" => Some(Unit::Length(LengthUnit::Meter(3))),

        "in" | "inch" | "inches" => Some(Unit::Length(LengthUnit::Inch)),
        "ft" | "feet" => Some(Unit::Length(LengthUnit::Feet)),
        "yd" | "yard" | "yards" => Some(Unit::Length(LengthUnit::Yard)),
        "mi" | "mile" | "miles" => Some(Unit::Length(LengthUnit::Mile)),

        // Area
        "mm2" | "square_millimeter" | "square_millimeters" | "mm^2" | "mmˆ2" => Some(Unit::Area(AreaUnit::Meter2(-3))),
        "cm2" | "square_centimeter" | "square_centimeters" | "cm^2" | "cmˆ2" => Some(Unit::Area(AreaUnit::Meter2(-2))),
        "m2" | "square_meter" | "square_meters" | "m^2" | "mˆ2" => Some(Unit::Area(AreaUnit::Meter2(0))),
        "km2" | "square_kilometer" | "square_kilometers" | "km^2" | "kmˆ2" => Some(Unit::Area(AreaUnit::Meter2(3))),

        "in2" | "square_inch" | "square_inches" | "in^2" | "inˆ2" => Some(Unit::Area(AreaUnit::Inch2)),
        "ft2" | "sqft" | "square_foot" | "square_feet" | "ft^2" | "ftˆ2" => Some(Unit::Area(AreaUnit::Feet2)),
        "yd2" | "square_yard" | "square_yards"  | "yd^2" | "ydˆ2"=> Some(Unit::Area(AreaUnit::Yard2)),
        "mi2" | "square_mile" | "square_miles" | "mi^2" | "miˆ2" => Some(Unit::Area(AreaUnit::Mile2)),
        "ac" | "acre" | "acres" => Some(Unit::Area(AreaUnit::Acre)),
        "ha" | "hectare" | "hectares" => Some(Unit::Area(AreaUnit::Hectare)),

        // Volume
        "ml" | "milliliter" | "milliliters" => Some(Unit::Volume(VolUnit::Liter(-3))),
        "l" | "liter" | "liters" => Some(Unit::Volume(VolUnit::Liter(0))),

        "mm3" | "cubic_millimeter" | "cubic_millimeters" | "mm^3" | "mmˆ3" => Some(Unit::Volume(VolUnit::Meter3(-3))),
        "cm3" | "cubic_centimeter" | "cubic_centimeters" | "cm^3" | "cmˆ3" => Some(Unit::Volume(VolUnit::Meter3(-2))),
        "m3" | "cubic_meter" | "cubic_meters" | "m^3" | "mˆ3" => Some(Unit::Volume(VolUnit::Meter3(0))),

        "teaspoon" | "teaspoons" => Some(Unit::Volume(VolUnit::TeaSpoon)),
        "tablespoon" | "tablespoons" => Some(Unit::Volume(VolUnit::TableSpoon)),
        "cup" | "cups" => Some(Unit::Volume(VolUnit::Cup)),
        "pt" | "pint" | "pints" => Some(Unit::Volume(VolUnit::Pint)),
        "gal" | "gallon" | "gallons" => Some(Unit::Volume(VolUnit::Gallon)),

        // Mass
        "mg" | "milligram" | "milligrams" => Some(Unit::Mass(MassUnit::Gram(-3))),
        "g"  | "gram" | "grams" => Some(Unit::Mass(MassUnit::Gram(0))),
        "kg" | "kilogram" | "kilograms" => Some(Unit::Mass(MassUnit::Gram(3))),

        "oz" | "ounce" | "ounces" => Some(Unit::Mass(MassUnit::Ounce)),
        "lb" | "pound" | "pounds" => Some(Unit::Mass(MassUnit::Pound)),
        "st" | "stone" | "stones" => Some(Unit::Mass(MassUnit::Stone)),

        _ => None,
    }
}

pub fn parser(tokens: Vec<String>) -> Task {
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
                Task::Error(format!("Invalid unit '{}'", tokens[0]))
            }
        }
        2 => {
            // Case A: number + unit
            if let Ok(val) = tokens[0].parse::<f32>() {
                if let Some(unit) = parse_unit(&tokens[1]) {
                    Task::ConvertAll(val, unit)
                } else {
                    Task::Error(format!("Invalid unit '{}'", tokens[1]))
                }
            } else {
                // Case B: unit + unit
                if let (Some(a), Some(b)) = (
                    parse_unit(&tokens[0]),
                    parse_unit(&tokens[1])
                ) {
                    Task::ConvertTo(1.0, a, b)
                } else {
                    // Check if the first arg was a unit and the second was bad
                    if parse_unit(&tokens[0]).is_some() {
                        Task::Error(format!("Invalid unit '{}'", tokens[1]))
                    } else {
                        Task::Error(format!("Invalid unit '{}'", tokens[0]))
                    }
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
                    // Refined error messages
                    if parse_unit(&tokens[1]).is_none() {
                        Task::Error(format!("Invalid unit '{}'", tokens[1]))
                    } else {
                        Task::Error(format!("Invalid unit '{}'", tokens[2]))
                    }
                }
            } else {
                Task::Error("Invalid format, see --help".to_string())
            }
        }
        _ => Task::Error("Too many arguments".to_string()),
    }
}