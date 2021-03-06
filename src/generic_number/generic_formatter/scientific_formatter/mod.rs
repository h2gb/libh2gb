use serde::{Serialize, Deserialize};
use simple_error::{SimpleResult, bail};

use crate::generic_number::{GenericNumber, GenericFormatter, GenericFormatterImpl};

/// Render a [`GenericNumber`] as a scientific (exponential) value.
///
/// # Example
///
/// ```
/// use libh2gb::generic_number::*;
///
/// // Create a GenericNumber directly - normally you'd use a GenericReader
/// let number = GenericNumber::from(100u64);
///
/// // Default 'pretty' formatter
/// assert_eq!("1e2", ScientificFormatter::pretty().render(number).unwrap());
///
/// // Also works on floating point
/// let number = GenericNumber::from(314.159f32);
/// assert_eq!("3.14159e2", ScientificFormatter::pretty().render(number).unwrap());
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ScientificFormatter {
    /// Print the `e` in the scientific notation will be uppercase (`1E0`
    /// instead of `1e0`).
    pub uppercase: bool,
}

impl ScientificFormatter {
    pub fn new(uppercase: bool) -> GenericFormatter {
        GenericFormatter::Scientific(Self {
            uppercase: uppercase
        })
    }

    pub fn pretty() -> GenericFormatter {
        Self::new(false)
    }
}

impl GenericFormatterImpl for ScientificFormatter {
    fn render(&self, number: GenericNumber) -> SimpleResult<String> {
        Ok(match (self.uppercase, number) {
            (true, GenericNumber::U8(v))   => format!("{:E}", v),
            (true, GenericNumber::U16(v))  => format!("{:E}", v),
            (true, GenericNumber::U32(v))  => format!("{:E}", v),
            (true, GenericNumber::U64(v))  => format!("{:E}", v),
            (true, GenericNumber::U128(v)) => format!("{:E}", v),
            (true, GenericNumber::I8(v))   => format!("{:E}", v),
            (true, GenericNumber::I16(v))  => format!("{:E}", v),
            (true, GenericNumber::I32(v))  => format!("{:E}", v),
            (true, GenericNumber::I64(v))  => format!("{:E}", v),
            (true, GenericNumber::I128(v)) => format!("{:E}", v),
            (true, GenericNumber::F32(v))  => format!("{:E}", v),
            (true, GenericNumber::F64(v))  => format!("{:E}", v),

            (false, GenericNumber::U8(v))   => format!("{:e}", v),
            (false, GenericNumber::U16(v))  => format!("{:e}", v),
            (false, GenericNumber::U32(v))  => format!("{:e}", v),
            (false, GenericNumber::U64(v))  => format!("{:e}", v),
            (false, GenericNumber::U128(v)) => format!("{:e}", v),
            (false, GenericNumber::I8(v))   => format!("{:e}", v),
            (false, GenericNumber::I16(v))  => format!("{:e}", v),
            (false, GenericNumber::I32(v))  => format!("{:e}", v),
            (false, GenericNumber::I64(v))  => format!("{:e}", v),
            (false, GenericNumber::I128(v)) => format!("{:e}", v),
            (false, GenericNumber::F32(v))  => format!("{:e}", v),
            (false, GenericNumber::F64(v))  => format!("{:e}", v),

            (_, GenericNumber::Char(_, _))  => bail!("Cannot display character as scientific"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use simple_error::SimpleResult;

    use crate::generic_number::{Context, Endian, GenericReader};

    #[test]
    fn test_scientific_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x7f\xff\xff\xff\x80\x00\x00\x00\xff\xff\xff\xff".to_vec();

        let tests = vec![
            // index  uppercase  expected
            (   0,    false,     "0e0"),
            (   4,    false,     "2.147483647e9"),
            (   8,    false,     "2.147483648e9"),
            (  12,    false,     "4.294967295e9"),
            (   0,    true,      "0E0"),
            (   4,    true,      "2.147483647E9"),
            (   8,    true,      "2.147483648E9"),
            (  12,    true,      "4.294967295E9"),
        ];

        for (index, uppercase, expected) in tests {
            let context = Context::new_at(&data, index);
            let number = GenericReader::U32(Endian::Big).read(context)?;

            assert_eq!(
                expected,
                ScientificFormatter::new(uppercase).render(number)?,
            );
        }

        Ok(())
    }

    #[test]
    fn test_scientific_i32() -> SimpleResult<()> {
        let data = b"\x00\x00\x00\x00\x7f\xff\xff\xff\x80\x00\x00\x00\xff\xff\xff\xff".to_vec();

        let tests = vec![
            // index  uppercase  expected
            (   0,    false,     "0e0"),
            (   4,    false,     "2.147483647e9"),
            (   8,    false,     "-2.147483648e9"),
            (  12,    false,     "-1e0"),
            (   0,    true,      "0E0"),
            (   4,    true,      "2.147483647E9"),
            (   8,    true,      "-2.147483648E9"),
            (  12,    true,      "-1E0"),
        ];

        for (index, uppercase, expected) in tests {
            let context = Context::new_at(&data, index);
            let number = GenericReader::I32(Endian::Big).read(context)?;

            assert_eq!(
                expected,
                ScientificFormatter::new(uppercase).render(number)?,
            );
        }

        Ok(())
    }

    #[test]
    fn test_exponent_f64() -> SimpleResult<()> {
        // I wrote and disassembled a simple C program to get these strings.. double is hard
        let data = b"\x40\x09\x1e\xb8\x51\xeb\x85\x1f\x40\x09\x33\x33\x33\x33\x33\x33".to_vec();

        let tests = vec![
            // index  uppercase expected
            (   0,    false,    "3.14e0"),
            (   8,    false,    "3.15e0"),
            (   0,    true,     "3.14E0"),
            (   8,    true,     "3.15E0"),
        ];

        for (index, uppercase, expected) in tests {
            let context = Context::new_at(&data, index);
            let number = GenericReader::F64(Endian::Big).read(context)?;

            assert_eq!(
                expected,
                ScientificFormatter::new(uppercase).render(number)?,
            );
        }

        Ok(())
    }

}
