## Client Version 1.12

### Wowm Representation
```rust,ignore
enum PetTalkReason : u32 {
    SPECIAL_SPELL = 0;    
    ATTACK = 1;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SPECIAL_SPELL` | 0 (0x00) |  |  |
| `ATTACK` | 1 (0x01) |  |  |
