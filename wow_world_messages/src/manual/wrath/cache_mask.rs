use std::io;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CacheMask {
    data: [Option<u32>; Self::SIZE],
}

impl CacheMask {
    pub const GLOBAL_CACHE: u32 = 0x15;
    pub const PER_CHARACTER_CACHE: u32 = 0xEA;

    const SIZE: usize = 32;

    pub fn read(r: &mut impl io::Read) -> Result<Self, io::Error> {
        let mask = crate::util::read_u32_le(r)?;
        let mut data = [None; Self::SIZE];

        for (i, d) in data.iter_mut().enumerate() {
            if (mask & (1 << i)) != 0 {
                *d = Some(crate::util::read_u32_le(r)?);
            }
        }

        Ok(Self { data })
    }

    pub(crate) fn write_into_vec(&self, mut v: impl io::Write) -> Result<(), io::Error> {
        let mut mask = 0_u32;

        for (i, d) in self.data.iter().enumerate() {
            if d.is_some() {
                mask |= 1 << i;
            }
        }

        io::Write::write_all(&mut v, &mask.to_le_bytes())?;

        for &i in self.data() {
            if let Some(d) = i {
                io::Write::write_all(&mut v, &d.to_le_bytes())?;
            }
        }

        Ok(())
    }

    pub const fn data(&self) -> &[Option<u32>] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [Option<u32>] {
        &mut self.data
    }

    pub(crate) const fn size(&self) -> usize {
        const MASK_VARIABLE_SIZE: usize = 4;
        let mut size = MASK_VARIABLE_SIZE;

        const MEMBER_SIZE: usize = 4;

        let mut index = 0;
        while index < self.data.len() {
            if self.data[index].is_some() {
                size += MEMBER_SIZE;
            }
            index += 1;
        }

        size
    }
}
