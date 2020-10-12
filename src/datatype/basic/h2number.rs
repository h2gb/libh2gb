use serde::{Serialize, Deserialize};
use simple_error::SimpleResult;

use sized_number::{Context, SizedDefinition, SizedDisplay};

use crate::datatype::H2Type;
use crate::datatype::basic::H2BasicType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct H2Number {
    definition: SizedDefinition,
    display: SizedDisplay,
}

impl From<H2Number> for H2Type {
    fn from(o: H2Number) -> H2Type {
        H2Type::from(H2BasicType::Number(o))
    }
}

impl H2Number {
    pub fn new(definition: SizedDefinition, display: SizedDisplay) -> Self {
        Self {
            definition: definition,
            display: display,
        }
    }

    pub fn to_string(&self, context: &Context) -> SimpleResult<String> {
        self.definition.to_string(context, self.display)
    }

    pub fn size(&self) -> u64 {
        self.definition.size()
    }

    pub fn related(&self, _context: &Context) -> SimpleResult<Vec<(u64, H2Type)>> {
        Ok(vec![])
    }
}
