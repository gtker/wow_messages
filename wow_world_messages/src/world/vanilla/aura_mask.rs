#[derive(Debug, Hash, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AuraMask {
    inners: [Option<u16>; Self::MAX_CAPACITY],
}

impl AuraMask {
    const MAX_CAPACITY: usize = 32;

    pub(crate) fn read(mut r: impl std::io::Read) -> Result<Self, std::io::Error> {
        let mut inners = [None; Self::MAX_CAPACITY];
        let bit_pattern: u32 = crate::util::read_u32_le(&mut r)?;

        for (i, inner) in inners.iter_mut().enumerate() {
            if (bit_pattern & (1 << i)) != 0 {
                *inner = Some(crate::util::read_u16_le(&mut r)?);
            }
        }

        Ok(Self { inners })
    }

    pub(crate) fn write_into_vec(&self, mut v: impl std::io::Write) -> Result<(), std::io::Error> {
        let mut bit_pattern: u32 = 0;
        for (i, &b) in self.auras().iter().enumerate() {
            if b.is_some() {
                bit_pattern |= 1 << i;
            }
        }

        std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice())?;

        for &i in self.auras() {
            if let Some(b) = i {
                std::io::Write::write_all(&mut v, b.to_le_bytes().as_slice())?;
            }
        }

        Ok(())
    }

    pub const fn auras(&self) -> &[Option<u16>] {
        self.inners.as_slice()
    }

    pub const fn auras_mut(&mut self) -> &mut [Option<u16>] {
        self.inners.as_mut_slice()
    }

    pub(crate) const fn size(&self) -> usize {
        const MASK_VARIABLE_SIZE: usize = core::mem::size_of::<u32>();
        let mut auras = 0;
        let mut index = 0;
        while index < self.inners.len() {
            if let Some(i) = self.inners[index] {
                auras += 2;
            }
            index += 1;
        }

        MASK_VARIABLE_SIZE + auras
    }
}
