## Client Version 1.12

### Wowm Representation
```rust,ignore
enum PetFeedback : u8 {
    PET_DEAD = 1;    
    NOTHING_TO_EAT = 2;    
    CANT_ATTACK_TARGET = 3;    
    NO_PATH_TO = 4;    
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `PET_DEAD` | 1 (0x01) |  |  |
| `NOTHING_TO_EAT` | 2 (0x02) |  |  |
| `CANT_ATTACK_TARGET` | 3 (0x03) |  |  |
| `NO_PATH_TO` | 4 (0x04) |  |  |
