use std::io;
use std::io::Read;

#[derive(Debug, Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EnchantMask {
    enchants: [Option<u16>; Self::MAX_CAPACITY],
}

impl EnchantMask {
    const MAX_CAPACITY: usize = 16;

    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        let mut auras = [None; Self::MAX_CAPACITY];
        let bit_pattern: u16 = crate::util::read_u16_le(r)?;

        for (i, aura) in auras.iter_mut().enumerate() {
            if (bit_pattern & (1 << i)) != 0 {
                *aura = Some(crate::util::read_u16_le(r)?);
            }
        }

        Ok(Self { enchants: auras })
    }

    pub(crate) fn write_into_vec(&self, mut v: &mut impl std::io::Write) -> Result<(), io::Error> {
        let mut bit_pattern: u16 = 0;
        for (i, &b) in self.enchants().iter().enumerate() {
            if b.is_some() {
                bit_pattern |= 1 << i;
            }
        }

        std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice())?;

        for &i in self.enchants() {
            if let Some(m) = i {
                std::io::Write::write_all(&mut v, m.to_le_bytes().as_slice())?;
            }
        }

        Ok(())
    }

    pub const fn enchants(&self) -> &[Option<u16>] {
        self.enchants.as_slice()
    }

    pub fn enchants_mut(&mut self) -> &mut [Option<u16>] {
        self.enchants.as_mut_slice()
    }

    pub const fn size(&self) -> usize {
        const MASK_VARIABLE_SIZE: usize = std::mem::size_of::<u16>();
        const AURA_SIZE: usize = core::mem::size_of::<u16>();
        let mut auras = 0;

        let mut index = 0;
        while index < self.enchants.len() {
            if self.enchants[index].is_some() {
                auras += AURA_SIZE;
            }
            index += 1;
        }

        MASK_VARIABLE_SIZE + auras
    }
}

impl Default for EnchantMask {
    fn default() -> Self {
        Self {
            enchants: [None; Self::MAX_CAPACITY],
        }
    }
}
