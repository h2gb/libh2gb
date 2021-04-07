use simple_error::{SimpleResult, bail};
use serde::{Serialize, Deserialize};

/// A simple class to abstract-out differently-sized keys.
///
/// Users shouldn't need this directly.
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub enum KeyOrIV {
    Bits64([u8; 8]),
    Bits128([u8; 16]),
    Bits192([u8; 24]),
    Bits256([u8; 32]),
}

impl KeyOrIV {
    pub fn new(key: Vec<u8>) -> SimpleResult<Self> {
        Ok(match key.len() {
            8 => {
                let mut a = [0; 8];
                a.copy_from_slice(&key);
                KeyOrIV::Bits64(a)
            }
            16 => {
                let mut a = [0; 16];
                a.copy_from_slice(&key);
                KeyOrIV::Bits128(a)
            }
            24 => {
                let mut a = [0; 24];
                a.copy_from_slice(&key);
                KeyOrIV::Bits192(a)
            }
            32 => {
                let mut a = [0; 32];
                a.copy_from_slice(&key);
                KeyOrIV::Bits256(a)
            }
            _  => bail!("Invalid BlockCipher key or iv length: {} bytes / {} bits", key.len(), key.len() * 8),
        })
    }

    /// Get the value as a 64-bit key, or throw an error.
    ///
    /// This simplifies validating a DES IV.
    pub fn get64(self) -> SimpleResult<[u8; 8]> {
        match self {
            KeyOrIV::Bits64(v) => Ok(v),
            _ => bail!("Invalid IV length"),
        }
    }

    /// Get the value as a 128-bit key, or throw an error.
    ///
    /// This simplifies validating an AES IV.
    pub fn get128(self) -> SimpleResult<[u8; 16]> {
        match self {
            KeyOrIV::Bits128(v) => Ok(v),
            _ => bail!("Invalid IV length"),
        }
    }
}

