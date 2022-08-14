use std::fmt::{Display, Formatter};
use std::io::{Read, Write};

use crate::util::{read_u64_le, read_u8_le};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Copy, Clone)]
pub struct Guid {
    guid: u64,
}

impl Guid {
    pub fn new(guid: u64) -> Self {
        Self { guid }
    }

    /// Guid with 0 value.
    ///
    /// Client uses this to mean different things, including things like no target selected.
    pub fn zero() -> Self {
        Self::new(0)
    }

    pub fn is_zero(&self) -> bool {
        self.guid == 0
    }

    pub fn guid(&self) -> u64 {
        self.guid
    }

    pub(crate) fn write_packed_guid_into_vec(&self, v: &mut Vec<u8>) {
        let placeholder_index = v.len();
        // Placeholder
        v.push(0);

        let guid = self.guid.to_le_bytes();
        let mut bit_pattern = 0;

        for (i, &b) in guid.iter().enumerate() {
            if b != 0 {
                bit_pattern |= 1 << i;
                v.push(b);
            }
        }

        v[placeholder_index] = bit_pattern;
    }

    pub(crate) fn read(r: &mut impl Read) -> Result<Self, std::io::Error> {
        Ok(Self {
            guid: read_u64_le(r)?,
        })
    }

    pub(crate) fn size(&self) -> usize {
        let mut amount_of_bytes = 1;

        for i in 0..8 {
            if (self.guid & (0xFF << (i * 8))) != 0 {
                amount_of_bytes += 1;
            }
        }

        amount_of_bytes
    }

    pub(crate) fn read_packed(r: &mut impl Read) -> Result<Self, std::io::Error> {
        let bit_pattern = read_u8_le(r)?;
        let mut guid: u64 = 0;

        for index in 0..8 {
            let bit = bit_pattern & (1 << index);

            if bit != 0 {
                let byte = read_u8_le(r)?;
                guid |= ((byte as u64) << (index * 8));
            }
        }

        Ok(Self { guid })
    }
}

impl From<u64> for Guid {
    fn from(v: u64) -> Self {
        Self::new(v)
    }
}

impl Display for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        u64::fmt(&self.guid, f)
    }
}

#[cfg(test)]
mod test {
    use super::Guid;
    use std::io::Cursor;
    use std::io::Read;

    #[test]
    fn packed() {
        const GUID: u64 = 0xDEADBEEFFACADE;

        let guid = Guid::new(GUID);
        assert_eq!(guid.guid(), GUID);

        // Make sure that writing into a vec doesn't clobber existing values
        let mut r = vec![1, 2, 3, 4];
        guid.write_packed_guid_into_vec(&mut r);

        let mut cursor = Cursor::new(r);
        let mut padding = [0_u8; 4];
        cursor.read_exact(&mut padding).unwrap();
        let guid2 = Guid::read_packed(&mut cursor).unwrap();

        assert_eq!(guid, guid2);

        let mut r = vec![1, 2, 3, 4];
        r.append(&mut guid.guid().to_le_bytes().to_vec());

        let mut cursor = Cursor::new(r);
        cursor.read_exact(&mut padding).unwrap();
        let guid2 = Guid::read(&mut cursor).unwrap();
        assert_eq!(guid, guid2);
    }
}
