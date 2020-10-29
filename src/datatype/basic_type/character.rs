use serde::{Serialize, Deserialize};
use simple_error::SimpleResult;

use crate::datatype::{H2Type, H2Types, H2TypeTrait, ResolveOffset};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
}

impl From<Character> for H2Type {
    fn from(o: Character) -> H2Type {
        H2Type::new(H2Types::Character(o))
    }
}

impl From<(u64, Character)> for H2Type {
    fn from(o: (u64, Character)) -> H2Type {
        H2Type::new_aligned(Some(o.0), H2Types::Character(o.1))
    }
}

impl Character {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl H2TypeTrait for Character {
    fn is_static(&self) -> bool {
        true
    }

    fn size(&self, _offset: &ResolveOffset) -> SimpleResult<u64> {
        Ok(1)
    }

    fn to_string(&self, offset: &ResolveOffset) -> SimpleResult<String> {
        match offset {
            ResolveOffset::Static(_) => Ok("Character".to_string()),
            ResolveOffset::Dynamic(context) => {
                let number = context.read_u8()?;

                match number > 0x1F && number < 0x7F {
                    true  => Ok((number as char).to_string()),
                    false => Ok("<invalid>".to_string()),
                }
            }
        }
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
        let offset = ResolveOffset::Dynamic(Context::new(&data));

        assert_eq!("<invalid>", Character::new().to_string(&offset.at(0))?);
        assert_eq!("<invalid>", Character::new().to_string(&offset.at(1))?);
        assert_eq!(" ",         Character::new().to_string(&offset.at(2))?);
        assert_eq!("A",         Character::new().to_string(&offset.at(3))?);
        assert_eq!("B",         Character::new().to_string(&offset.at(4))?);
        assert_eq!("~",         Character::new().to_string(&offset.at(5))?);
        assert_eq!("<invalid>", Character::new().to_string(&offset.at(6))?);
        assert_eq!("<invalid>", Character::new().to_string(&offset.at(7))?);
        assert_eq!("<invalid>", Character::new().to_string(&offset.at(8))?);

        Ok(())
    }
}
