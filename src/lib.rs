use std::io::Write;

pub mod converter;
pub mod units;
mod parser;

use parser::*;
use converter::*;

fn print_help<W: Write>(writer: &mut W) {
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
