use aes::{Aes128, Aes192, Aes256};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

use simple_error::{SimpleResult, bail};
use serde::{Serialize, Deserialize};

use crate::transformation::TransformerTrait;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub enum AESKey {
    Bits128([u8; 16]),
    Bits192([u8; 24]),
    Bits256([u8; 32]),
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct AESSettings {
    key: AESKey,
    iv: Option<[u8; 16]>,
}

impl AESSettings {
    pub fn new(key: Vec<u8>, iv: Option<[u8; 16]>) -> SimpleResult<Self> {
        let key: AESKey = match key.len() {
            16 => AESKey::Bits128(*b"AAAAAABAAAAAAAAA"),
            24 => AESKey::Bits192(*b"AAAAAABAAAAAAAAAAAAAAAAA"),
            32 => AESKey::Bits256(*b"AAAAAAABAAAAAAAAAAAAAAAAAAAAAAAA"),
            _  => bail!("Invalid AES key length: {} bytes / {} bits", key.len(), key.len() * 8),
        };

        Ok(AESSettings {
            key: key,
            iv: iv,
        })
    }
}

pub struct TransformAES {
    settings: AESSettings,
}

impl TransformAES {
    pub fn new(settings: AESSettings) -> Self {
        TransformAES {
            settings: settings,
        }
    }
}

impl TransformerTrait for TransformAES {
    fn transform(&self, buffer: &Vec<u8>) -> SimpleResult<Vec<u8>> {
        // Get the iv, or a default blank one
        let iv = self.settings.iv.unwrap_or([0;16]);

        // Pick the implementation based on the key
        let out = match self.settings.key {
            AESKey::Bits128(k) => {
                match Cbc::<Aes128, Pkcs7>::new_var(&k, &iv) {
                    Ok(c) => {
                        match c.decrypt_vec(&buffer) {
                            Ok(d) => d,
                            Err(e) => bail!("Error decrypting buffer: {}", e),
                        }
                    }
                    Err(e) => bail!("Error setting up cipher: {}", e),
                }
            },

            AESKey::Bits192(k) => {
                match Cbc::<Aes192, Pkcs7>::new_var(&k, &iv) {
                    Ok(c) => {
                        match c.decrypt_vec(&buffer) {
                            Ok(d) => d,
                            Err(e) => bail!("Error decrypting buffer: {}", e),
                        }
                    }
                    Err(e) => bail!("Error setting up cipher: {}", e),
                }
            },

            AESKey::Bits256(k) => {
                match Cbc::<Aes256, Pkcs7>::new_var(&k, &iv) {
                    Ok(c) => {
                        match c.decrypt_vec(&buffer) {
                            Ok(d) => d,
                            Err(e) => bail!("Error decrypting buffer: {}", e),
                        }
                    }
                    Err(e) => bail!("Error setting up cipher: {}", e),
                }
            },
        };

        Ok(out.to_vec())
    }

    fn untransform(&self, _buffer: &Vec<u8>) -> SimpleResult<Vec<u8>> {
        bail!("Not implemented yet!");
    }

    fn check(&self, _buffer: &Vec<u8>) -> bool {
       true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    use crate::transformation::Transformation;

    #[test]
    fn test_aes() -> SimpleResult<()> {
        let settings = AESSettings {
            key: AESKey::Bits128(*b"AAAAAAAAAAAAAAAA"),
            iv: None,
        };

        let t = Transformation::FromAES(settings);
        let result = t.transform(&b"\x6c\x97\x52\xb3\x06\xde\xc3\xaa\x5d\x4d\x0e\xe7\x98\xcc\xd9\xb0".to_vec())?;
        assert_eq!(b"Hello world!".to_vec(), result);


        let settings = AESSettings {
            key: AESKey::Bits192(*b"AAAAAAAAAAAAAAAAAAAAAAAA"),
            iv: None,
        };

        let t = Transformation::FromAES(settings);
        let result = t.transform(&b"\xc8\xcc\x26\xe8\x1a\x48\x8e\xb0\x1e\xac\xb1\xc5\x7c\x07\xe3\x30\xa7\xda\x88\x27\xbf\xcc\x1e\xab\xcc\x53\xd5\x0a\x21\x55\x93\x79".to_vec())?;
        assert_eq!(b"Hello world! This is a test".to_vec(), result);

        Ok(())
    }
}