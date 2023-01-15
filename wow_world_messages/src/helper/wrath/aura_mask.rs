use std::io;
use std::io::Read;

#[derive(Debug, Hash, Copy, Default, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Aura {
    pub aura: u32,
    pub unknown: u8,
}

#[derive(Debug, Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AuraMask {
    auras: [Option<Aura>; Self::MAX_CAPACITY],
}

impl AuraMask {
    const MAX_CAPACITY: usize = 64;

    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        let mut auras = [None; Self::MAX_CAPACITY];
        let bit_pattern: u64 = crate::util::read_u64_le(r)?;

        for (i, aura) in auras.iter_mut().enumerate() {
            if (bit_pattern & (1 << i)) != 0 {
                let spell = crate::util::read_u32_le(r)?;
                let unknown = crate::util::read_u8_le(r)?;
                *aura = Some(Aura {
                    aura: spell,
                    unknown,
                });
            }
        }

        Ok(Self { auras })
    }

    pub(crate) fn write_into_vec(&self, mut v: &mut Vec<u8>) -> Result<(), io::Error> {
        let mut bit_pattern: u64 = 0;
        for (i, &b) in self.auras().iter().enumerate() {
            if b.is_some() {
                bit_pattern |= 1 << i;
            }
        }

        std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice())?;

        for &i in self.auras() {
            if let Some(Aura {aura, unknown}) = i {
                std::io::Write::write_all(&mut v, aura.to_le_bytes().as_slice())?;
                std::io::Write::write_all(&mut v, unknown.to_le_bytes().as_slice())?;
            }
        }

        Ok(())
    }

    pub const fn auras(&self) -> &[Option<Aura>] {
        self.auras.as_slice()
    }

    pub fn auras_mut(&mut self) -> &mut [Option<Aura>] {
        self.auras.as_mut_slice()
    }

    pub const fn size(&self) -> usize {
        std::mem::size_of::<u64>() + 5 * self.auras.len()
    }
}

impl Default for AuraMask {
    fn default() -> Self {
        Self {
            auras: [None; Self::MAX_CAPACITY],
        }
    }
}
