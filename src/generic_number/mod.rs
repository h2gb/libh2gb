//! A simple library for reading differently-sized integers and floats.
//!
//! While writing h2gb, I needed a way to dynamically read integers from a
//! Vec of u8 bytes. Libraries like [`byteorder`] and [`io::Cursor`] nearly
//! have the right functionality, but weren't quite flexible enough.
//!
//! This library defines a [`Context`] type, which is just a thin wrapper for a
//! [`std::io::Cursor`]. It's also a super cheap type that can be cloned as
//! needed.
//!
//! To create an instance, you need to first define how the number will be
//! defined. Using the [`GenericReader`] enum, choose a size and a
//! [`Endian`]:
//!
//! ```
//! use libh2gb::generic_number::{GenericReader, Endian};
//!
//! let d = GenericReader::U32(Endian::Big);
//! ```
//!
//! Once you have an instance of [`GenericReader`], it can convert a
//! [`Context`] into a string in a variety of formats - use `GenericFormatter` to
//! configure how it should convert:
//!
//! ```
//! use libh2gb::generic_number::*;
//!
//! let buffer = b"ABCD".to_vec();
//! let context = Context::new_at(&buffer, 0);
//! let d = GenericReader::U32(Endian::Big);
//! let number = d.read(context).unwrap();
//!
//! assert_eq!("0x41424344", HexFormatter::pretty().render(number).unwrap());
//! assert_eq!("1094861636", DecimalFormatter::new().render(number).unwrap());
//! assert_eq!("0o10120441504", OctalFormatter::pretty().render(number).unwrap());
//! assert_eq!("0b01000001010000100100001101000100", BinaryFormatter::pretty().render(number).unwrap());
//! assert_eq!("1.094861636e9", ScientificFormatter::pretty().render(number).unwrap());
//! ```
//!
//! The string conversion is designed to be "stamp"-able - you can define the
//! format once, then apply it in multiple places or with multiple formats! No
//! context or data is stored as part of the type.
//!
//! In addition to formatting a string, 64-bit and smaller unsigned instances
//! of [`GenericReader`] can be converted into [`u64`] (unsigned) integers, and
//! 64-bit and smaller signed instances can be converted into [`i64`] (signed)
//! integers:
//!
//! ```
//! use libh2gb::generic_number::{Context, GenericReader, Endian, GenericFormatter, HexFormatter, BinaryFormatter, ScientificFormatter};
//!
//! let buffer = b"\x01\x02\x03\x04\x05\x06\x07\x08".to_vec();
//! let context = Context::new_at(&buffer, 0);
//! let d = GenericReader::U32(Endian::Big);
//!
//! assert_eq!(0x01,               GenericReader::U8.read(context).unwrap().as_u64().unwrap());
//! assert_eq!(0x0102,             GenericReader::U16(Endian::Big).read(context).unwrap().as_u64().unwrap());
//! assert_eq!(0x01020304,         GenericReader::U32(Endian::Big).read(context).unwrap().as_u64().unwrap());
//! assert_eq!(0x0102030405060708, GenericReader::U64(Endian::Big).read(context).unwrap().as_u64().unwrap());
//! ```

mod helpers;
pub use helpers::*;

mod generic_reader;
pub use generic_reader::*;

mod generic_formatter;
pub use generic_formatter::*;

mod generic_number;
pub use generic_number::*;
