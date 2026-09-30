#[derive(Debug, Hash, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct InspectTalentGearMask {
    inners: [Option<crate::wrath::InspectTalentGear>; Self::MAX_CAPACITY],
}

impl InspectTalentGearMask {
    const MAX_CAPACITY: usize = 32;

    pub(crate) fn read(mut r: impl std::io::Read) -> Result<Self, std::io::Error> {
        let mut inners = [None; Self::MAX_CAPACITY];
        let bit_pattern: u32 = crate::util::read_u32_le(&mut r)?;

        for (i, inner) in inners.iter_mut().enumerate() {
            if (bit_pattern & (1 << i)) != 0 {
                *inner = Some(crate::wrath::InspectTalentGear::read(&mut r)?);
            }
        }

        Ok(Self { inners })
    }

    pub(crate) fn write_into_vec(&self, mut v: impl std::io::Write) -> Result<(), std::io::Error> {
        let mut bit_pattern: u32 = 0;
        for (i, &b) in self.enchants().iter().enumerate() {
            if b.is_some() {
                bit_pattern |= 1 << i;
            }
        }

        std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice())?;

        for &i in self.enchants() {
            if let Some(b) = i {
                b.write_into_vec(&mut v)?;
            }
        }

        Ok(())
    }

    pub const fn enchants(&self) -> &[Option<crate::wrath::InspectTalentGear>] {
        self.inners.as_slice()
    }

    pub const fn enchants_mut(&mut self) -> &mut [Option<crate::wrath::InspectTalentGear>] {
        self.inners.as_mut_slice()
    }

    pub(crate) const fn size(&self) -> usize {
        const MASK_VARIABLE_SIZE: usize = core::mem::size_of::<u32>();
        let mut auras = 0;
        let mut index = 0;
        while index < self.inners.len() {
            if let Some(i) = self.inners[index] {
                auras += i.size();
            }
            index += 1;
        }

        MASK_VARIABLE_SIZE + auras
    }
}
