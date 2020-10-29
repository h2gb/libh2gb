use serde::{Serialize, Deserialize};
use simple_error::SimpleResult;
use std::ops::Range;

use sized_number::Context;

pub mod basic_type;
pub mod complex_type;
// pub mod dynamic_type;

pub mod helpers;

// Allow us to resolve either statically or dynamically, depending on what's
// needed. One or the other might throw an error, though.
pub enum ResolveOffset<'a> {
    Static(u64),
    Dynamic(Context<'a>),
}

impl<'a> From<u64> for ResolveOffset<'a> {
    fn from(o: u64) -> ResolveOffset<'a> {
        ResolveOffset::Static(o)
    }
}

impl<'a> From<Context<'a>> for ResolveOffset<'a> {
    fn from(o: Context<'a>) -> ResolveOffset<'a> {
        ResolveOffset::Dynamic(o)
    }
}

impl<'a> ResolveOffset<'a> {
    pub fn position(&self) -> u64 {
        match self {
            Self::Static(n) => *n,
            Self::Dynamic(c) => c.position(),
        }
    }

    pub fn at(&self, offset: u64) -> ResolveOffset {
        match self {
            Self::Static(_) => Self::Static(offset),
            Self::Dynamic(c) => Self::Dynamic(c.at(offset)),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Align {
    Yes,
    No,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum H2Types {
    // Basic
    H2Number(basic_type::h2number::H2Number),
    H2Pointer(basic_type::h2pointer::H2Pointer),
    Character(basic_type::character::Character),
    IPv4(basic_type::ipv4::IPv4),
    IPv6(basic_type::ipv6::IPv6),
    Unicode(basic_type::unicode::Unicode),

    // Complex
    H2Array(complex_type::h2array::H2Array),

    // Dynamic
    // NTString(dynamic_type::ntstring::NTString),
}

pub trait H2TypeTrait {
    // Is the size known ahead of time?
    fn is_static(&self) -> bool;

    // Get the static size, if possible
    fn size(&self, offset: &ResolveOffset) -> SimpleResult<u64>;

    // Get "child" nodes (array elements, struct body, etc), if possible
    // Empty vector = a leaf node
    fn children(&self, _offset: &ResolveOffset) -> SimpleResult<Vec<PartiallyResolvedType>> {
        Ok(vec![])
    }

    // Get the user-facing name of the type
    fn to_string(&self, offset: &ResolveOffset) -> SimpleResult<String>;

    // Get "related" nodes - ie, what a pointer points to
    fn related(&self, _offset: &ResolveOffset) -> SimpleResult<Vec<(u64, H2Type)>> {
        Ok(vec![])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartiallyResolvedType {
    offset: Range<u64>,
    field_name: Option<String>,
    field_type: H2Type,
}

impl PartiallyResolvedType {
    // This is a simpler way to display the type for the right part of the
    // context
    pub fn to_string(&self, offset: &ResolveOffset) -> SimpleResult<String> {
        self.field_type.to_string(&offset.at(self.offset.start))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct H2Type {
    field: H2Types,
    byte_alignment: Option<u64>,
}

impl H2Type {
    pub fn new(field: H2Types) -> Self {
        Self {
            field: field,
            byte_alignment: None,
        }
    }

    pub fn new_aligned(byte_alignment: Option<u64>, field: H2Types) -> Self {
        Self {
            byte_alignment: byte_alignment,
            field: field,
        }
    }

    pub fn field_type(&self) -> &dyn H2TypeTrait {
        match &self.field {
            // Basic
            H2Types::H2Number(t)  => t,
            H2Types::H2Pointer(t) => t,
            H2Types::Character(t) => t,
            H2Types::IPv4(t)      => t,
            H2Types::IPv6(t)      => t,
            H2Types::Unicode(t)   => t,

            // Complex
            H2Types::H2Array(t)   => t,

            // Dynamic
            // H2Types::NTString(t)  => t,
        }
    }

    // Is the size known ahead of time?
    fn is_static(&self) -> bool {
        self.field_type().is_static()
    }

    fn size(&self, offset: &ResolveOffset, align: Align) -> SimpleResult<u64> {
        match align {
            Align::Yes  => Ok(helpers::maybe_round_up(self.field_type().size(offset)?, self.byte_alignment)),
            Align::No   => Ok(self.field_type().size(offset)?),
        }
        // match self.field_type().static_size() {
        //     Ok(s)   => Ok(helpers::maybe_round_up(s, self.byte_alignment)),
        //     Err(e)  => Err(e),
        // }
    }

    fn children(&self, offset: &ResolveOffset) -> SimpleResult<Vec<PartiallyResolvedType>> {
        self.field_type().children(offset)
    }

    // Render as a string
    fn to_string(&self, offset: &ResolveOffset) -> SimpleResult<String> {
        self.field_type().to_string(offset)
    }

    // Get "related" nodes - ie, what a pointer points to
    fn related(&self, offset: &ResolveOffset) -> SimpleResult<Vec<(u64, H2Type)>> {
        self.field_type().related(offset)
    }

    pub fn fully_resolve(&self, offset: &ResolveOffset) -> SimpleResult<Vec<PartiallyResolvedType>> {
        let children = self.children(offset)?;
        let mut result: Vec<PartiallyResolvedType> = Vec::new();

        if children.len() == 0 {
            // No children? Return ourself!
            result.push(PartiallyResolvedType {
                offset: offset.position()..(offset.position() + self.size(offset, Align::No)?),
                field_name: None,
                field_type: self.clone(),
            });
        } else {
            // Children? Gotta get 'em all!
            for child in children.iter() {
                result.append(&mut child.field_type.fully_resolve(&offset.at(child.offset.start))?);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_error::SimpleResult;
    use sized_number::Context;
    use basic_type::character::Character;

    #[test]
    fn test_character() -> SimpleResult<()> {
        let t = H2Type::from(Character::new());
        let data = b"ABCD".to_vec();
        let s_offset = ResolveOffset::Static(0);
        let d_offset = ResolveOffset::Dynamic(Context::new(&data));

        assert_eq!(1, t.size(&s_offset, Align::No)?);
        assert_eq!(1, t.size(&d_offset, Align::No)?);

        assert_eq!("A", t.to_string(&d_offset.at(0))?);
        assert_eq!("B", t.to_string(&d_offset.at(1))?);
        assert_eq!("C", t.to_string(&d_offset.at(2))?);
        assert_eq!("D", t.to_string(&d_offset.at(3))?);

        assert_eq!(0, t.children(&s_offset)?.len());
        assert_eq!(0, t.children(&d_offset)?.len());

        let resolved = t.fully_resolve(&s_offset)?;
        assert_eq!(1, resolved.len());
        assert_eq!(0..1, resolved[0].offset);
        assert_eq!("Character", resolved[0].to_string(&s_offset)?);

        let resolved = t.fully_resolve(&s_offset.at(1))?;
        assert_eq!(1, resolved.len());
        assert_eq!(1..2, resolved[0].offset);
        assert_eq!("Character", resolved[0].to_string(&s_offset)?);

        let resolved = t.fully_resolve(&d_offset)?;
        assert_eq!(1, resolved.len());
        assert_eq!(0..1, resolved[0].offset);
        assert_eq!("A", resolved[0].to_string(&d_offset)?);

        let resolved = t.fully_resolve(&d_offset.at(1))?;
        assert_eq!(1, resolved.len());
        assert_eq!(1..2, resolved[0].offset);
        assert_eq!("B", resolved[0].to_string(&d_offset)?);

        Ok(())
    }

    // #[test]
    // fn test_align() -> SimpleResult<()> {
    //     // Align to 4-byte boundaries
    //     let t = H2Type::from((4, Character::new()));
    //     let data = b"ABCD".to_vec();
    //     let context = Context::new(&data);

    //     assert_eq!(1, t.size()?);
    //     assert_eq!(1, t.size(&Context::new(&data).at(0))?);
    //     assert_eq!("A", t.to_string(&Context::new(&data).at(0))?);
    //     assert_eq!("B", t.to_string(&Context::new(&data).at(1))?);
    //     assert_eq!("C", t.to_string(&Context::new(&data).at(2))?);
    //     assert_eq!("D", t.to_string(&Context::new(&data).at(3))?);

    //     assert_eq!(0, t.children_static(0)?.len());
    //     assert_eq!(0, t.children(&Context::new(&data).at(0))?.len());

    //     let resolved = t.resolve(&Context::new(&data).at(0))?;
    //     assert_eq!(1, resolved.len());
    //     assert_eq!(0..1, resolved[0].offset);
    //     assert_eq!("A", resolved[0].to_string(&Context::new(&data))?);

    //     let resolved = t.resolve(&Context::new(&data).at(1))?;
    //     assert_eq!(1, resolved.len());
    //     assert_eq!(1..2, resolved[0].offset);
    //     assert_eq!("B", resolved[0].to_string(&Context::new(&data))?);

    //     Ok(())
    // }

    fn test_pointer() -> SimpleResult<()> {
        Ok(())
    }

    #[test]
    fn test_static_array() -> SimpleResult<()> {
        Ok(())
    }

    #[test]
    fn test_dynamic_array() -> SimpleResult<()> {
        Ok(())
    }

    #[test]
    fn test_aligned_array() -> SimpleResult<()> {
        Ok(())
    }

    #[test]
    fn test_static_struct() -> SimpleResult<()> {
        Ok(())
    }

    #[test]
    fn test_dynamic_struct() -> SimpleResult<()> {
        Ok(())
    }

    #[test]
    fn test_enum() -> SimpleResult<()> {
        Ok(())
    }

    #[test]
    fn test_ntstring() -> SimpleResult<()> {
        Ok(())
    }
}
