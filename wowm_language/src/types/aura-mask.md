# Masks

Masks consist of an integer pattern mask and a member for every set bit in the pattern mask.

# Vanilla

AuraMask has a pattern of 32 bits, with `u16` members.

# TBC

AuraMask has a pattern of 64 bits with [`Aura`](../docs/aura.md) struct members.

# Wrath

AuraMask has a pattern of 64 bits with [`Aura`](../docs/aura.md) struct members.

```rust,ignore
pub struct AuraMask {
    auras: [Option<u16>; Self::MAX_CAPACITY],
}

impl AuraMask {
    const MAX_CAPACITY: usize = 32;

    pub fn read(r: &mut impl Read) -> Result<Self, io::Error> {
        let mut auras = [None; Self::MAX_CAPACITY];
        let bit_pattern: u32 = crate::util::read_u32_le(r)?;

        for (i, aura) in auras.iter_mut().enumerate() {
            if (bit_pattern & (1 << i)) != 0 {
                *aura = Some(crate::util::read_u16_le(r)?);
            }
        }

        Ok(Self { auras })
    }

    pub(crate) fn write_into_vec(&self, mut v: &mut Vec<u8>) -> Result<(), std::io::Error> {
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
        self.auras.as_slice()
    }

    pub const fn size(&self) -> usize {
        std::mem::size_of::<u32>() + std::mem::size_of::<u16>() * self.auras.len()
    }
}
```
