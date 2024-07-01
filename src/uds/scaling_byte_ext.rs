crate::enum_wrapper!(uds, ScalingExtension, ScalingExtensionByte);

/// A macro rule to generate prefix and postfix functions from a single enum
macro_rules! generate_enum {
    (
        $(#[$enum_attr:meta])*
        $enum_vis:vis enum $enum_name:ident {
            $(
                #[doc = $doc:literal]
                $(#[prefix = $prefix:literal])?
                $(#[postfix = $postfix:literal])?
                $variant_name:ident = $variant_value:expr,
            )*
        }
    ) => {
        $(#[$enum_attr])*
        $enum_vis enum $enum_name {
            $(
                #[doc = $doc]
                $(#[doc = concat!(" Prefix `", $prefix, "`")])*
                $(#[doc = concat!(" Postfix `", $postfix, "`")])*
                $variant_name = $variant_value,
            )*
        }

        impl $enum_name {
            /// Returns the optional postfix of the scaling byte
            #[must_use]
            pub fn get_postfix(&self) -> Option<&'static str> {
                #[allow(clippy::match_same_arms)]
                match self {
                    $(
                        $(Self::$variant_name => Some($postfix),)*
                    )*
                    _ => None,
                }
            }

            /// Returns the optional prefix of the scaling byte
            #[must_use]
            pub fn get_prefix(&self) -> Option<&'static str> {
                #[allow(clippy::match_same_arms)]
                match self {
                    $(
                        $(Self::$variant_name => Some($prefix),)*
                    )*
                    _ => None,
                }
            }
        }
    };
}

generate_enum! {
    /// Scaling data byte extensions
    /// This enum is used to represent the following:
    /// 1. Measurement units
    /// 2. Format specifiers
    /// 3. Unit scale prefixes
    ///
    /// Due to this, each value specifies if it will return a Postfix or prefix.
    /// Use [`ScalingExtension::get_postfix`] to return the optional postfix of the scaling byte,
    /// or [`ScalingExtension::get_prefix`] to return the optional prefix of the scaling byte.
    #[repr(u8)]
    #[derive(strum::FromRepr, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "iter", derive(strum::EnumIter))]
    pub enum ScalingExtension {
        /// No unit or presentation
        NoUnit = 0x00,
        /// Meter - measure of length.
        #[postfix = "m"]
        Meter = 0x01,
        /// Foot - measure of length.
        #[postfix = "ft"]
        Foot = 0x02,
        /// Inch - measure of length.
        #[postfix = "in"]
        Inch = 0x03,
        /// Yard - measure of length.
        #[postfix = "yd"]
        Yard = 0x04,
        /// English mile - measure of length.
        #[postfix = "mi"]
        Mile = 0x05,
        /// Gram - measure of mass.
        #[postfix = "g"]
        Gram = 0x06,
        /// Metric tonne - measure of mass.
        #[postfix = "t"]
        Tonne = 0x07,
        /// Second - measure of time.
        #[postfix = "s"]
        Second = 0x08,
        /// Minute - measure of time.
        #[postfix = "min"]
        Minute = 0x09,
        /// Hour - measure of time.
        #[postfix = "h"]
        Hour = 0x0A,
        /// Day - measure of time.
        #[postfix = "d"]
        Day = 0x0B,
        /// Year - measure of time.
        #[postfix = "y"]
        Year = 0x0C,
        /// Ampere - measure of current.
        #[postfix = "A"]
        Ampere = 0x0D,
        /// Volt - measure of voltage.
        #[postfix = "V"]
        Volt = 0x0E,
        /// Coulomb - measure of electric charge.
        #[postfix = "C"]
        Coulomb = 0x0F,
        /// Ohm - measure of resistance.
        #[postfix = "W"]
        Ohm = 0x10,
        /// Farad - measure of capacitance.
        #[postfix = "F"]
        Farad = 0x11,
        /// Henry - measure of inductance.
        #[postfix = "H"]
        Henry = 0x12,
        /// Siemens - measure of electric conductance.
        #[postfix = "S"]
        Siemens = 0x13,
        /// Weber - measure of magnetic flux.
        #[postfix = "Wb"]
        Weber = 0x14,
        /// Tesla - measure of magnetic flux density.
        #[postfix = "T"]
        Tesla = 0x15,
        /// Kelvin - measure of thermodynamic temperature.
        #[postfix = "K"]
        Kelvin = 0x16,
        /// Celsius - measure of thermodynamic temperature.
        #[postfix = "°C"]
        Celsius = 0x17,
        /// Fahrenheit - measure of thermodynamic temperature.
        #[postfix = "°F"]
        Fahrenheit = 0x18,
        /// Candela - measure of luminous intensity.
        #[postfix = "cd"]
        Candela = 0x19,
        /// Radians - measure of plane angle.
        #[postfix = "rad"]
        Radian = 0x1A,
        /// Degrees - measure of plane angle.
        #[postfix = "°"]
        Degree = 0x1B,
        /// Hertz - measure of frequency.
        #[postfix = "Hz"]
        Hertz = 0x1C,
        /// Joule - measure of energy.
        #[postfix = "J"]
        Joule = 0x1D,
        /// Newton - measure of force.
        #[postfix = "N"]
        Newton = 0x1E,
        /// Kilopond - measure of force.
        #[postfix = "kp"]
        Kilopond = 0x1F,
        /// Pound force - measure of force.
        #[postfix = "lbf"]
        PoundForce = 0x20,
        /// Watt - measure of power.
        #[postfix = "W"]
        Watt = 0x21,
        /// Metric horse power - measure of power.
        #[postfix = "hk"]
        HorsePowerMetric = 0x22,
        /// Imperial horse power - measure of power.
        #[postfix = "hp"]
        HorsePowerImperial = 0x23,
        /// Pascal - measure of pressure.
        #[postfix = "Pa"]
        Pascal = 0x24,
        /// Bar - measure of pressure.
        #[postfix = "bar"]
        Bar = 0x25,
        /// Atmosphere - measure of pressure.
        #[postfix = "atm"]
        Atmosphere = 0x26,
        /// Pound force per square inch - measure of pressure.
        #[postfix = "psi"]
        Psi = 0x27,
        /// Becquerel - measure of radioactivity.
        #[postfix = "Bq"]
        Becquerel = 0x28,
        /// Lumen - measure of light flux.
        #[postfix = "lm"]
        Lumen = 0x29,
        /// Lux - measure of illuminance.
        #[postfix = "lx"]
        Lux = 0x2A,
        /// Liter - measure of volume.
        #[postfix = "l"]
        Liter = 0x2B,
        /// British gallon - measure of volume.
        GallonImperial = 0x2C,
        /// US liquid gallon - measure of volume.
        GallonUs = 0x2D,
        /// Cubic inch - measure of volume.
        #[postfix = "cu in"]
        CubicInch = 0x2E,
        /// Meter per second - measure of speed.
        #[postfix = "m/s"]
        MeterPerSecond = 0x2F,
        /// Kilometers per hour - measure of speed.
        #[postfix = "km/h"]
        KilometerPerHour = 0x30,
        /// Miles per hour - measure of speed.
        #[postfix = "mph"]
        MilePerHour = 0x31,
        /// Revolutions per second - measure of angular velocity.
        #[postfix = "rps"]
        RevolutionsPerSecond = 0x32,
        /// Revolutions per minute - measure of angular velocity.
        #[postfix = "rpm"]
        RevolutionsPerMinute = 0x33,
        /// Count.
        Counts = 0x34,
        /// Percent.
        #[postfix = "%"]
        Percent = 0x35,
        /// Milligram per stroke - measure of mass per engine stroke.
        #[postfix = "mg/stroke"]
        MilligramPerStroke = 0x36,
        /// Meter per square second - measure of acceleration.
        #[postfix = "m/s²"]
        MeterPerSquareSecond = 0x37,
        /// Newton meter - measure of moment (e.g. torsion moment).
        #[postfix = "Nm"]
        NewtonMeter = 0x38,
        /// Liter per minute - measure of flow.
        #[postfix = "l/min"]
        LiterPerMinute = 0x39,
        /// Watt per square meter W/m2 Intensity - measure of Intensity.
        #[postfix = "W/m²"]
        WattPerSquareMeter = 0x3A,
        /// Bar per second - measure of pressure change.
        #[postfix = "bar/s"]
        BarPerSecond = 0x3B,
        /// Radians per second - measure of angular velocity.
        #[postfix = "rad/s"]
        RadiansPerSecond = 0x3C,
        /// Radians per square second - measure of angular acceleration.
        #[postfix = "rad/s²"]
        RadiansPerSquareSecond = 0x3D,
        /// Kilograms per square meter.
        #[postfix = "kg/m²"]
        KilogramsPerSquareMeter = 0x3E,
        /// Exa `10^18`
        #[prefix = "E"]
        Exa = 0x40,
        /// Peta `10^15`
        #[prefix = "P"]
        Peta = 0x41,
        /// Tera `10^12`
        #[prefix = "T"]
        Tera = 0x42,
        /// Giga `10^9`
        #[prefix = "G"]
        Giga = 0x43,
        /// Mega `10^6`
        #[prefix = "M"]
        Mega = 0x44,
        /// Kilo `10^3`
        #[prefix = "k"]
        Kilo = 0x45,
        /// Hecto `10^2`
        #[prefix = "h"]
        Hecto = 0x46,
        /// Deca `10^1`
        #[prefix = "da"]
        Deca = 0x47,
        /// Deci `10^-1`
        #[prefix = "d"]
        Deci = 0x48,
        /// Centi `10^-2`
        #[prefix = "c"]
        Centi = 0x49,
        /// Milli `10^-3`
        #[prefix = "m"]
        Milli = 0x4A,
        /// Micro `10^-6`
        #[prefix = "m"]
        Micro = 0x4B,
        /// Nano `10^-9`
        #[prefix = "n"]
        Nano = 0x4C,
        /// Pico `10^-12`
        #[prefix = "p"]
        Pico = 0x4D,
        /// Femto `10^-15`
        #[prefix = "f"]
        Femto = 0x4E,
        /// Atto `10^-18`
        #[prefix = "a"]
        Atto = 0x4F,
        /// Date in `Year-Month-Day` format.
        Date1 = 0x50,
        /// Date in `Day/Month/Year` format.
        Date2 = 0x51,
        /// Date in `Month/Day/Year` format.
        Date3 = 0x52,
        /// Calendar week.
        #[postfix = "W"]
        Week = 0x53,
        /// Time in UTC `Hour/Minute/Second` format.
        Time1 = 0x54,
        /// Time in `Hour/Minute/Second` format.
        Time2 = 0x55,
        /// Date and time1 in `Second/Minute/Hour/Day/Month/Year` format.
        DateAndTime1 = 0x56,
        /// DateAndTime2 in `Second/Minute/Hour/Day/Month/Year/Local minute offset/Local hour offset` format.
        DateAndTime2 = 0x57,
        /// DateAndTime3 in `Second/Minute/Hour/Month/Day/Year` format.
        DateAndTime3 = 0x58,
        /// DateAndTime4 in `Second/Minute/Hour/Month/Day/Year/Local minute offset/Local hour offset` format.
        DateAndTime4 = 0x59,
    }
}
