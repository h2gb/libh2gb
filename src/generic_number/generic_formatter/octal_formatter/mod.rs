use serde::{Serialize, Deserialize};
use simple_error::{SimpleResult, bail};

use crate::generic_number::{GenericNumber, GenericFormatter, GenericFormatterImpl};

/// Render a [`GenericNumber`] as an octal value.
///
/// # Example
///
/// ```
/// use libh2gb::generic_number::*;
///
/// // Create a GenericNumber directly - normally you'd use a GenericReader
/// let number = GenericNumber::from(32u8);
///
/// // Default 'pretty' formatter
/// assert_eq!("0o40", OctalFormatter::pretty().render(number).unwrap());
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct OctalFormatter {
    /// Prefix octal strings with `0o`
    pub prefix: bool,

    /// Zero-pad octal strings to the full width - `0001` vs `1`)
    pub padded: bool,
}

impl OctalFormatter {
    pub fn new(prefix: bool, padded: bool) -> GenericFormatter {
        GenericFormatter::Octal(Self {
            prefix: prefix,
            padded: padded,
        })
    }

    pub fn pretty() -> GenericFormatter {
        Self::new(true, false)
    }
}

impl GenericFormatterImpl for OctalFormatter {
    fn render(&self, number: GenericNumber) -> SimpleResult<String> {
        let mut s = match (self.padded, number) {
            (true, GenericNumber::U8(v))   => format!("{:03o}", v),
            (true, GenericNumber::U16(v))  => format!("{:06o}", v),
            (true, GenericNumber::U32(v))  => format!("{:011o}", v),
            (true, GenericNumber::U64(v))  => format!("{:022o}", v),
            (true, GenericNumber::U128(v)) => format!("{:043o}", v),
            (true, GenericNumber::I8(v))   => format!("{:03o}", v),
            (true, GenericNumber::I16(v))  => format!("{:06o}", v),
            (true, GenericNumber::I32(v))  => format!("{:011o}", v),
            (true, GenericNumber::I64(v))  => format!("{:022o}", v),
            (true, GenericNumber::I128(v)) => format!("{:043o}", v),

            (false, GenericNumber::U8(v))   => format!("{:o}", v),
            (false, GenericNumber::U16(v))  => format!("{:o}", v),
            (false, GenericNumber::U32(v))  => format!("{:o}", v),
            (false, GenericNumber::U64(v))  => format!("{:o}", v),
            (false, GenericNumber::U128(v)) => format!("{:o}", v),
            (false, GenericNumber::I8(v))   => format!("{:o}", v),
            (false, GenericNumber::I16(v))  => format!("{:o}", v),
            (false, GenericNumber::I32(v))  => format!("{:o}", v),
            (false, GenericNumber::I64(v))  => format!("{:o}", v),
            (false, GenericNumber::I128(v)) => format!("{:o}", v),

            (_, GenericNumber::F32(_))      => bail!("Cannot display floating point as octal"),
            (_, GenericNumber::F64(_))      => bail!("Cannot display floating point as octal"),
            (_, GenericNumber::Char(_, _))  => bail!("Cannot display character as octal"),
        };

        // Do the prefix after for simplicity
        if self.prefix {
            s = format!("0o{}", s);
        }

        Ok(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use simple_error::SimpleResult;

    use crate::generic_number::{Context, Endian, GenericReader};

    #[test]
    fn test_octal_u8() -> SimpleResult<()> {
        let data = b"\x00\x7F\x80\xFF".to_vec();

        let tests = vec![
            // index  prefix   padded   expected
            (   0,    false,   false,   "0"),
            (   1,    false,   false,   "177"),
            (   2,    false,   false,   "200"),
            (   3,    false,   false,   "377"),

            // index  prefix   padded   expected
            (   0,    false,   true,    "000"),
            (   1,    false,   true,    "177"),
            (   2,    false,   true,    "200"),
            (   3,    false,   true,    "377"),

            // index  prefix   padded   expected
            (   0,    true,    false,   "0o0"),
            (   1,    true,    false,   "0o177"),
            (   2,    true,    false,   "0o200"),
            (   3,    true,    false,   "0o377"),

            // index  prefix   padded   expected
            (   0,    true,    true,    "0o000"),
            (   1,    true,    true,    "0o177"),
            (   2,    true,    true,    "0o200"),
            (   3,    true,    true,    "0o377"),
        ];

        for (index, prefix, padded, expected) in tests {
            let context = Context::new_at(&data, index);
            let number = GenericReader::U8.read(context)?;

            assert_eq!(
                expected,
                OctalFormatter::new(prefix, padded).render(number)?,
            );
        }

        Ok(())
    }

    #[test]
    fn test_octal_u16() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  prefix   padded   expected
            (   0,    false,   false,   "0"),
            (   2,    false,   false,   "11064"),
            (   4,    false,   false,   "177777"),

            // index  prefix   padded   expected
            (   0,    false,   true,    "000000"),
            (   2,    false,   true,    "011064"),
            (   4,    false,   true,    "177777"),

            // index  prefix   padded   expected
            (   0,    true,    false,   "0o0"),
            (   2,    true,    false,   "0o11064"),
            (   4,    true,    false,   "0o177777"),

            // index  prefix   padded   expected
            (   0,    true,    true,    "0o000000"),
            (   2,    true,    true,    "0o011064"),
            (   4,    true,    true,    "0o177777"),
        ];

        for (index, prefix, padded, expected) in tests {
            let context = Context::new_at(&data, index);
            let number = GenericReader::U16(Endian::Big).read(context)?;

            assert_eq!(
                expected,
                OctalFormatter::new(prefix, padded).render(number)?,
            );
        }

        Ok(())
    }

    #[test]
    fn test_octal_u32() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  prefix  padded   expected
            (   0,    false,  false,   "11064"),
            (   2,    false,  false,   "2215177777"),
            (   4,    false,  false,   "37777777777"),

            // index  prefix  padded   expected
            (   0,    false,  true,   "00000011064"),
            (   2,    false,  true,   "02215177777"),
            (   4,    false,  true,   "37777777777"),

            // index  prefix  padded   expected
            (   0,    true,  false,   "0o11064"),
            (   2,    true,  false,   "0o2215177777"),
            (   4,    true,  false,   "0o37777777777"),

            // index  prefix  padded   expected
            (   0,    true,  true,     "0o00000011064"),
            (   2,    true,  true,     "0o02215177777"),
            (   4,    true,  true,     "0o37777777777"),
        ];

        for (index, prefix, padded, expected) in tests {
            let context = Context::new_at(&data, index);
            let number = GenericReader::U32(Endian::Big).read(context)?;

            assert_eq!(
                expected,
                OctalFormatter::new(prefix, padded).render(number)?,
            );
        }

        Ok(())
    }

    #[test]
    fn test_octal_u64() -> SimpleResult<()> {
        let data = b"\x00\x00\x12\x34\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF".to_vec();

        let tests = vec![
            // index  prefix  padded  expected
            (   0,    false,  false,  "443237777777777"),
            (   8,    false,  false,  "1777777777777777777777"),

            // index  prefix  padded  expected
            (   0,    false,  true,   "0000000443237777777777"),
            (   8,    false,  true,   "1777777777777777777777"),

            // index  prefix  padded  expected
            (   0,    true,   false,  "0o443237777777777"),
            (   8,    true,   false,  "0o1777777777777777777777"),

            // index  prefix  padded  expected
            (   0,    true,   true,   "0o0000000443237777777777"),
            (   8,    true,   true,   "0o1777777777777777777777"),

        ];

        for (index, prefix, padded, expected) in tests {
            let context = Context::new_at(&data, index);
            let number = GenericReader::U64(Endian::Big).read(context)?;

            assert_eq!(
                expected,
                OctalFormatter::new(prefix, padded).render(number)?,
            );
        }

        Ok(())
    }
}
