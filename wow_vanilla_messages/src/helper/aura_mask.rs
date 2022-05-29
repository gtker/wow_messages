#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io;
use std::io::{Read, Write};
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AuraMask {
    auras: [Option<u16>; Self::MAX_CAPACITY],
}

impl AuraMask {
    const MAX_CAPACITY: usize = 32;
    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        let mut auras = [None; Self::MAX_CAPACITY];
        let mut bit_pattern: u32 = crate::util::read_u32_le(r)?;

        for i in 0..Self::MAX_CAPACITY {
            if (bit_pattern & (1 << i)) != 0 {
                auras[i] = Some(crate::util::read_u16_le(r)?);
            }
        }

        Ok(Self { auras })
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(48 * 4);
        let mut bit_pattern: u32 = 0;
        for (i, &b) in self.auras().iter().enumerate() {
            if b.is_some() {
                bit_pattern |= 1 << i;
            }
        }

        std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice());

        for &i in self.auras() {
            if let Some(b) = i {
                std::io::Write::write_all(&mut v, b.to_le_bytes().as_slice());
            }
        }

        v
    }

    pub fn auras(&self) -> &[Option<u16>] {
        self.auras.as_slice()
    }

    pub fn size(&self) -> usize {
        std::mem::size_of::<u32>() + std::mem::size_of::<u16>() * self.auras.len()
    }
}

#[cfg(test)]
mod test {
    use super::AuraMask;
    use std::io::Cursor;

    #[test]
    fn auras() {
        let mut v = Vec::new();
        let bits = 0b00010101001011_u32.to_le_bytes();
        for i in bits {
            v.push(i);
        }
        let auras = [123_u8, 66, 15, 237, 231, 74, 25, 45, 35, 74, 99, 12];
        for i in auras {
            v.push(i);
        }

        let mut cursor = Cursor::new(v.clone());
        let mask = AuraMask::read(&mut cursor).unwrap();

        let mut target = Vec::new();
        target.append(&mut mask.as_bytes());

        assert_eq!(v, target);
    }
}
