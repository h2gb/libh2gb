use serde::{Serialize, Deserialize};

use crate::datatype::H2Type;
use crate::datatype::ResolvedType;
use crate::datatype::basic::H2BasicType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct H2Simple {
    basic_type: Box<H2BasicType>,
}

impl From<H2Simple> for H2Type {
    fn from(o: H2Simple) -> H2Type {
        H2Type::from(H2Type::H2Simple(o))
    }
}

impl H2Simple {
    pub fn new(basic_type: H2BasicType) -> Self {
        Self {
            basic_type: Box::new(basic_type),
        }
    }

    pub fn resolve(&self, starting_offset: usize, field_names: Option<Vec<String>>) -> (Vec<ResolvedType>, usize) {
        let v: Vec<ResolvedType> = vec![
            ResolvedType {
                offset: starting_offset,
                field_names: field_names,
                basic_type: (*self.basic_type).clone(),
            }
        ];

        (v, starting_offset + self.basic_type.size())
    }

    pub fn size(&self) -> usize {
        self.basic_type.size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_error::SimpleResult;

    use crate::datatype::helpers::h2context::{H2Context, NumberDefinition};
    use crate::datatype::basic::h2integer::H2Integer;

    #[test]
    fn test_simple() -> SimpleResult<()> {
        let data = b"AAAABBBBCCCCDDDD".to_vec();
        let context = H2Context::new(&data, 0);

        let t = H2Type::from(H2Integer::new(NumberDefinition::u32_big()));
        assert_eq!(4, t.size());

        let resolved = t.resolve();
        assert_eq!(1, resolved.len());
        assert_eq!(0, resolved[0].offset);
        assert_eq!(None, resolved[0].field_names);

        println!("Type: {:?}", t);
        println!("\nto_strings:\n{:?}", t.to_strings(&context)?);

        Ok(())
    }
}
