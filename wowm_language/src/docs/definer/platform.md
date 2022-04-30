## Protocol Version *

### Wowm Representation
```rust,ignore
enum Platform : u32 {
    X86 = "\0x86";    
    PPC = "\0PPC";    
    OTHER = self.value    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `X86` | 7878710 (0x783836) |  |  |
| `PPC` | 5263427 (0x505043) |  |  |
