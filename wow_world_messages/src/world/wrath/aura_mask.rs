#[derive(Debug, Hash, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct AuraMask {
    inners: [Option<crate::wrath::Aura>; Self::MAX_CAPACITY],
}

impl AuraMask {
    const MAX_CAPACITY: usize = 64;

    pub(crate) fn read(mut r: impl std::io::Read) -> Result<Self, std::io::Error> {
        let mut inners = [None; Self::MAX_CAPACITY];
        let bit_pattern: u64 = crate::util::read_u64_le(&mut r)?;

        for (i, inner) in inners.iter_mut().enumerate() {
            if (bit_pattern & (1 << i)) != 0 {
                *inner = Some(crate::util::wrath_aura_read(&mut r)?);
            }
        }

        Ok(Self { inners })
    }

    pub(crate) fn write_into_vec(&self, mut v: impl std::io::Write) -> Result<(), std::io::Error> {
        let mut bit_pattern: u64 = 0;
        for (i, &b) in self.auras().iter().enumerate() {
            if b.is_some() {
                bit_pattern |= 1 << i;
            }
        }

        std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice())?;

        for &i in self.auras() {
            if let Some(b) = i {
                crate::util::wrath_aura_write_into_vec(&b, &mut v)?;
            }
        }

        Ok(())
    }

    pub const fn auras(&self) -> &[Option<crate::wrath::Aura>] {
        self.inners.as_slice()
    }

    pub const fn auras_mut(&mut self) -> &mut [Option<crate::wrath::Aura>] {
        self.inners.as_mut_slice()
    }

    pub(crate) const fn size(&self) -> usize {
        const MASK_VARIABLE_SIZE: usize = core::mem::size_of::<u64>();
        let mut auras = 0;
        let mut index = 0;
        while index < self.inners.len() {
            if let Some(i) = self.inners[index] {
                auras += 5;
            }
            index += 1;
        }

        MASK_VARIABLE_SIZE + auras
    }
}

impl Default for AuraMask {
    fn default() -> Self {
        AuraMask {
            inners: [None; Self::MAX_CAPACITY],
        }
    }
}
