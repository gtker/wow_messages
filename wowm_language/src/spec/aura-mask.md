# AuraMask

```rust,ignore
struct AuraMask {
    pub auras: [Option<u8>; 32],
}

impl AuraMask {
    pub fn read(r: &mut impl Read) -> Result<Self, std::io::Error> {
        let mask = r.read_u8()?;
        let auras = [None; 32];

        for i in 0..32 {
            if mask & (1 << i) () {
                auras[i] = Some(r.read_u32_le()?);   
            }
        }

        Self {
            auras,
        }
    }

    pub fn write(w: &mut impl Write) -> Result<()> {
        let mut mask: u32 = 0;

        for (i, aura) in auras.iter().enumerate() {
            if let Some(_) = aura {
                mask |= (1 << i)
            }
        }

        w.write_all(&mask)?;

        for aura in auras {
            w.write_all(&aura)?;
        }
    }
}
```
