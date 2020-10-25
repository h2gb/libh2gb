use serde::{Serialize, Deserialize};
use simple_error::SimpleResult;

use sized_number::Context;

use crate::datatype::H2Type;
use crate::datatype::basic_type::{H2BasicTrait, H2BasicType, H2BasicTypes};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
}

impl From<Character> for H2BasicType {
    fn from(o: Character) -> H2BasicType {
        H2BasicType::new(H2BasicTypes::Character(o))
    }
}

impl Character {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl H2BasicTrait for Character {
    fn to_string(&self, context: &Context) -> SimpleResult<String> {
        let number = context.read_u8()?;

        match number > 0x1F && number < 0x7F {
            true  => Ok((number as char).to_string()),
            false => Ok("<invalid>".to_string()),
        }
    }

    fn size(&self) -> u64 {
        1
    }

    fn related(&self, _context: &Context) -> SimpleResult<Vec<(u64, H2Type)>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_error::SimpleResult;
    use sized_number::Context;

    #[test]
    fn test_character() -> SimpleResult<()> {
        let data = b"\x00\x1F\x20\x41\x42\x7e\x7f\x80\xff".to_vec();

        assert_eq!("<invalid>", Character::new().to_string(&Context::new(&data).at(0))?);
        assert_eq!("<invalid>", Character::new().to_string(&Context::new(&data).at(1))?);
        assert_eq!(" ",         Character::new().to_string(&Context::new(&data).at(2))?);
        assert_eq!("A",         Character::new().to_string(&Context::new(&data).at(3))?);
        assert_eq!("B",         Character::new().to_string(&Context::new(&data).at(4))?);
        assert_eq!("~",         Character::new().to_string(&Context::new(&data).at(5))?);
        assert_eq!("<invalid>", Character::new().to_string(&Context::new(&data).at(6))?);
        assert_eq!("<invalid>", Character::new().to_string(&Context::new(&data).at(7))?);
        assert_eq!("<invalid>", Character::new().to_string(&Context::new(&data).at(8))?);

        Ok(())
    }
}