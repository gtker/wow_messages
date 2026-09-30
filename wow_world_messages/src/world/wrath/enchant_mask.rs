#[derive(Debug, Hash, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct EnchantMask {
    inners: [Option<u16>; Self::MAX_CAPACITY],
}

impl EnchantMask {
    const MAX_CAPACITY: usize = 16;

    pub(crate) fn read(mut r: impl std::io::Read) -> Result<Self, std::io::Error> {
        let mut inners = [None; Self::MAX_CAPACITY];
        let bit_pattern: u16 = crate::util::read_u16_le(&mut r)?;

        for (i, inner) in inners.iter_mut().enumerate() {
            if (bit_pattern & (1 << i)) != 0 {
                *inner = Some(crate::util::read_u16_le(&mut r)?);
            }
        }

        Ok(Self { inners })
    }

    pub(crate) fn write_into_vec(&self, mut v: impl std::io::Write) -> Result<(), std::io::Error> {
        let mut bit_pattern: u16 = 0;
        for (i, &b) in self.enchants().iter().enumerate() {
            if b.is_some() {
                bit_pattern |= 1 << i;
            }
        }

        std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice())?;

        for &i in self.enchants() {
            if let Some(b) = i {
                std::io::Write::write_all(&mut v, b.to_le_bytes().as_slice())?;
            }
        }

        Ok(())
    }

    pub const fn enchants(&self) -> &[Option<u16>] {
        self.inners.as_slice()
    }

    pub const fn enchants_mut(&mut self) -> &mut [Option<u16>] {
        self.inners.as_mut_slice()
    }

    pub(crate) const fn size(&self) -> usize {
        const MASK_VARIABLE_SIZE: usize = core::mem::size_of::<u16>();
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
