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

    pub fn guid(&self) -> u64 {
        self.guid
    }

    pub fn packed_guid(&self) -> Vec<u8> {
        // Worst case capacity
        let mut v = Vec::with_capacity(9);
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

        v[0] = bit_pattern;

        v
    }

    pub fn read(r: &mut impl Read) -> Result<Self, std::io::Error> {
        Ok(Self {
            guid: read_u64_le(r)?,
        })
    }

    pub fn size(&self) -> usize {
        let mut amount_of_bytes = 0;

        for i in 0..8 {
            if (self.guid & (0xFF << (i * 8))) != 0 {
                amount_of_bytes += 1;
            }
        }

        amount_of_bytes
    }

    pub fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.guid)?;

        Ok(())
    }

    pub fn read_packed(r: &mut impl Read) -> Result<Self, std::io::Error> {
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

    pub fn write_packed(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        w.write_all(&self.packed_guid())?;

        Ok(())
    }
}

impl From<u64> for Guid {
    fn from(v: u64) -> Self {
        Self::new(v)
    }
}

#[cfg(test)]
mod test {
    use super::Guid;
    use std::io::Cursor;

    #[test]
    fn packed() {
        const GUID: u64 = 0xDEADBEEFFACADE;

        let guid = Guid::new(GUID);
        assert_eq!(guid.guid(), GUID);

        let mut r = Vec::with_capacity(9);
        guid.write_packed(&mut r).unwrap();

        let mut cursor = Cursor::new(r);
        let guid2 = Guid::read_packed(&mut cursor).unwrap();

        assert_eq!(guid, guid2);

        let mut r = Vec::with_capacity(9);
        guid.write(&mut r).unwrap();

        let mut cursor = Cursor::new(r);
        let guid2 = Guid::read(&mut cursor).unwrap();
        assert_eq!(guid, guid2);
    }
}
