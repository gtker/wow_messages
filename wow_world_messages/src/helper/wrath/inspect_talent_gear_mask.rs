use crate::wrath::InspectTalentGear;
use std::io;
use std::io::Read;

#[derive(Debug, Hash, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
pub struct InspectTalentGearMask {
    inspect_talent_gears: [Option<InspectTalentGear>; Self::MAX_CAPACITY],
}

impl InspectTalentGearMask {
    const MAX_CAPACITY: usize = 32;

    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        let mut auras = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None,
        ];
        let bit_pattern: u32 = crate::util::read_u32_le(r)?;

        for (i, aura) in auras.iter_mut().enumerate() {
            if (bit_pattern & (1 << i)) != 0 {
                *aura = Some(InspectTalentGear::read(r)?);
            }
        }

        Ok(Self {
            inspect_talent_gears: auras,
        })
    }

    pub(crate) fn write_into_vec(&self, mut v: &mut Vec<u8>) -> Result<(), io::Error> {
        let mut bit_pattern: u16 = 0;
        for (i, b) in self.inspect_talent_gears().iter().enumerate() {
            if b.is_some() {
                bit_pattern |= 1 << i;
            }
        }

        std::io::Write::write_all(&mut v, bit_pattern.to_le_bytes().as_slice())?;

        for i in self.inspect_talent_gears().iter().flatten() {
            i.write_into_vec(v)?;
        }

        Ok(())
    }

    pub const fn inspect_talent_gears(&self) -> &[Option<InspectTalentGear>] {
        self.inspect_talent_gears.as_slice()
    }

    pub fn inspect_talent_gears_mut(&mut self) -> &mut [Option<InspectTalentGear>] {
        self.inspect_talent_gears.as_mut_slice()
    }

    pub fn size(&self) -> usize {
        const MASK_VARIABLE_SIZE: usize = std::mem::size_of::<u16>();
        let mut auras = 0;

        let mut index = 0;
        while index < self.inspect_talent_gears.len() {
            if let Some(m) = &self.inspect_talent_gears[index] {
                auras += m.size();
            }
            index += 1;
        }

        MASK_VARIABLE_SIZE + auras
    }
}
