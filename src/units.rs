#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Unit {
    Temperature(TempUnit),
    Length(LengthUnit),
    Area(AreaUnit),
    Volume(VolUnit),
    Mass(MassUnit)
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TempUnit {
    Kelvin,
    Celsius,
    Fahrenheit,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum LengthUnit {
    Meter(i8),

    Inch,
    Feet,
    Yard,
    Mile
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AreaUnit {
    Meter2(i8),

    Inch2,
    Feet2,
    Yard2,
    Mile2,

    Acre,
    Hectare,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum VolUnit {
    Liter(i8),
    Meter3(i8),

    TeaSpoon,
    TableSpoon,
    Cup,
    Pint,
    Gallon
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MassUnit {
    Gram(i8),

    Ounce,
    Pound,
    Stone,
}

