use crate::wrath::Aura;
use std::io;
use std::io::Read;

#[derive(Debug, Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AuraMask {
    auras: [Option<Aura>; Self::MAX_CAPACITY],
}

impl AuraMask {
    const MAX_CAPACITY: usize = 64;

    pub fn read<R: Read>(mut r: R) -> Result<Self, io::Error> {
        let mut auras = [None; Self::MAX_CAPACITY];
        let bit_pattern: u64 = crate::util::read_u64_le(&mut r)?;

        for (i, aura) in auras.iter_mut().enumerate() {
            if (bit_pattern & (1 << i)) != 0 {
                *aura = Some(crate::util::wrath_aura_read(&mut r)?);
            }
        }

        Ok(Self { auras })
    }

    pub(crate) fn write_into_vec(&self, mut v: impl std::io::Write) -> Result<(), io::Error> {
        let mut bit_pattern: u64 = 0;
        for (i, &b) in self.auras().iter().enumerate() {
            if b.is_some() {
                bit_pattern |= 1 << i;
            }
        }

        std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice())?;

        for &i in self.auras() {
            if let Some(aura) = i {
                crate::util::wrath_aura_write_into_vec(&aura, &mut v)?;
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
        const MASK_VARIABLE_SIZE: usize = std::mem::size_of::<u64>();
        const AURA_SIZE: usize = core::mem::size_of::<u32>() + core::mem::size_of::<u8>();
        let mut auras = 0;

        let mut index = 0;
        while index < self.auras.len() {
            if self.auras[index].is_some() {
                auras += AURA_SIZE;
            }
            index += 1;
        }

        MASK_VARIABLE_SIZE + auras
    }
}

impl Default for AuraMask {
    fn default() -> Self {
        Self {
            auras: [None; Self::MAX_CAPACITY],
        }
    }
}
