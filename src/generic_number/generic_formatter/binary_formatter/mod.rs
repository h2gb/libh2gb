use simple_error::{SimpleResult, bail};
use serde::{Serialize, Deserialize};

use crate::generic_number::{GenericNumber, GenericFormatter, GenericFormatterImpl};

/// Render a [`GenericNumber`] as a binary value.
///
/// # Example
///
/// ```
/// use libh2gb::generic_number::*;
///
/// // Create a GenericNumber directly - normally you'd use a GenericReader
/// let number = GenericNumber::from(15u8);
///
/// // Default 'pretty' formatter
/// assert_eq!("0b00001111", BinaryFormatter::pretty().render(number).unwrap());
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BinaryFormatter {
    /// Prefix binary strings with `0b`
    pub prefix: bool,

    /// Zero-pad binary strings to the full width - `00000001` vs `1`
    pub padded: bool,
}

impl BinaryFormatter {
    pub fn new(prefix: bool, padded: bool) -> GenericFormatter {
        GenericFormatter::Binary(Self {
            prefix: prefix,
            padded: padded,
        })
    }

    pub fn pretty() -> GenericFormatter {
        Self::new(true, true)
    }
}

impl GenericFormatterImpl for BinaryFormatter {
    fn render(&self, number: GenericNumber) -> SimpleResult<String> {
        let mut s = match (self.padded, number) {
            (true, GenericNumber::U8(v))   => format!("{:08b}", v),
            (true, GenericNumber::U16(v))  => format!("{:016b}", v),
            (true, GenericNumber::U32(v))  => format!("{:032b}", v),
            (true, GenericNumber::U64(v))  => format!("{:064b}", v),
            (true, GenericNumber::U128(v)) => format!("{:0128b}", v),
            (true, GenericNumber::I8(v))   => format!("{:08b}", v),
            (true, GenericNumber::I16(v))  => format!("{:016b}", v),
            (true, GenericNumber::I32(v))  => format!("{:032b}", v),
            (true, GenericNumber::I64(v))  => format!("{:064b}", v),
            (true, GenericNumber::I128(v)) => format!("{:0128b}", v),

            (false, GenericNumber::U8(v))   => format!("{:b}", v),
            (false, GenericNumber::U16(v))  => format!("{:b}", v),
            (false, GenericNumber::U32(v))  => format!("{:b}", v),
            (false, GenericNumber::U64(v))  => format!("{:b}", v),
            (false, GenericNumber::U128(v)) => format!("{:b}", v),
            (false, GenericNumber::I8(v))   => format!("{:b}", v),
            (false, GenericNumber::I16(v))  => format!("{:b}", v),
            (false, GenericNumber::I32(v))  => format!("{:b}", v),
            (false, GenericNumber::I64(v))  => format!("{:b}", v),
            (false, GenericNumber::I128(v)) => format!("{:b}", v),

            (_, GenericNumber::F32(_))      => bail!("Cannot display floating point as binary"),
            (_, GenericNumber::F64(_))      => bail!("Cannot display floating point as binary"),
            (_, GenericNumber::Char(_, _))  => bail!("Cannot display character as binary"),
        };

        // Add the prefix after for simplicity
        if self.prefix {
            s = format!("0b{}", s);
        }

        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use simple_error::SimpleResult;
    use crate::generic_number::{Context, GenericReader};

    #[test]
    fn test_binary_i8() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\xab\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index    prefix      padded   expected
            (   0,      true,       true,    "0b00000000"),
            (   1,      true,       true,    "0b00000000"),
            (   2,      true,       true,    "0b00010010"),
            (   3,      true,       true,    "0b10101011"),
            (   4,      true,       true,    "0b11111111"),
            (   5,      true,       true,    "0b11111111"),

            (   0,      false,      false,   "0"),
            (   1,      false,      false,   "0"),
            (   2,      false,      false,   "10010"),
            (   3,      false,      false,   "10101011"),
            (   4,      false,      false,   "11111111"),
            (   5,      false,      false,   "11111111"),
        ];

        for (index, padded, prefix, expected) in tests {
            let context = Context::new_at(&data, index);
            let number = GenericReader::U8.read(context)?;

            assert_eq!(
                expected,
                BinaryFormatter::new(prefix, padded).render(number)?,
            );
        }

        Ok(())
    }
}
