use simple_error::SimpleResult;
use serde::{Serialize, Deserialize};

use crate::sized_number::{GenericNumber, SizedOptions, BinaryOptions, DecimalOptions, EnumOptions, HexOptions, OctalOptions, ScientificOptions};

/// Display options with their associated configurations.
///
/// This is the core for configuring the output. It tries to make the best
/// decisions based on the datatype. When displaying a padded hex value, for
/// example, it's padded to the exact width of the field, no matter what that
/// is.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SizedDisplay {
    /// Display in hexadecimal.
    ///
    /// Example:
    /// ```
    /// use libh2gb::sized_number::*;
    ///
    /// let buffer = b"\x00\xab".to_vec();
    /// let d = SizedDefinition::U16(Endian::Big);
    /// let number = d.read(Context::new_at(&buffer, 0)).unwrap();
    ///
    /// assert_eq!("0x00ab", HexOptions::pretty().to_string(number).unwrap());
    /// assert_eq!("00AB", HexOptions::new(true,  false, true ).to_string(number).unwrap());
    /// assert_eq!("0xab", HexOptions::new(false, true,  false).to_string(number).unwrap());
    ///
    /// ```
    Hex(HexOptions),

    /// Display in decimal. Whether the display is signed or not depends on the
    /// `SizedDefinition` type chosen.
    ///
    /// Example:
    /// ```
    /// use libh2gb::sized_number::*;
    ///
    /// let buffer = b"\xFF\xFF".to_vec();
    /// let unsigned = SizedDefinition::U8.read(Context::new_at(&buffer, 0)).unwrap();
    /// assert_eq!("255", DecimalOptions::new().to_string(unsigned).unwrap());
    ///
    /// let signed   = SizedDefinition::I8.read(Context::new_at(&buffer, 0)).unwrap();
    /// assert_eq!("-1", DecimalOptions::new().to_string(signed).unwrap());
    ///
    /// ```
    Decimal(DecimalOptions),

    /// Display in octal.
    ///
    /// Example:
    /// ```
    /// use libh2gb::sized_number::*;
    ///
    /// let buffer = b"\x20".to_vec();
    /// let context = Context::new_at(&buffer, 0);
    /// let number = SizedDefinition::U8.read(context).unwrap();
    ///
    /// assert_eq!("0o40", OctalOptions::pretty().to_string(number).unwrap());
    ///
    /// ```
    Octal(OctalOptions),

    /// Display in binary. Padding can be enabled with `BinaryOptions`
    ///
    /// Example:
    /// ```
    /// use libh2gb::sized_number::*;
    ///
    /// let buffer = b"\x01".to_vec();
    /// let context = Context::new_at(&buffer, 0);
    /// let number = SizedDefinition::U8.read(context).unwrap();
    ///
    /// assert_eq!("0b00000001", BinaryOptions::pretty().to_string(number).unwrap());
    /// ```
    Binary(BinaryOptions),

    /// Display in scientific / exponent notation. The case of `e` can be
    /// changed with `ScientificOptions`.
    ///
    /// Example:
    /// ```
    /// use libh2gb::sized_number::*;
    ///
    /// let buffer = b"\x64".to_vec();
    /// let context = Context::new_at(&buffer, 0);
    /// let number = SizedDefinition::U8.read(context).unwrap();
    ///
    /// assert_eq!("1e2", ScientificOptions::pretty().to_string(number).unwrap());
    /// ```
    Scientific(ScientificOptions),

    /// Display as an 'enum' - a value selected from a list of common values.
    ///
    /// Example: XXX
    ///
    Enum(EnumOptions),
}

impl SizedDisplay {
    pub fn to_options(&self) -> Box<dyn SizedOptions> {
        match self {
            Self::Binary(o)     => Box::new(*o),
            Self::Decimal(o)    => Box::new(*o),
            Self::Enum(o)       => Box::new(*o),
            Self::Hex(o)        => Box::new(*o),
            Self::Octal(o)      => Box::new(*o),
            Self::Scientific(o) => Box::new(*o),
        }
    }

    pub fn to_string(&self, number: GenericNumber) -> SimpleResult<String> {
        self.to_options().to_string(number)
    }
}