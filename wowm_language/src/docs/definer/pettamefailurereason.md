## Client Version 1.12

### Wowm Representation
```rust,ignore
enum PetTameFailureReason : u8 {
    INVALIDCREATURE = 1;    
    TOOMANY = 2;    
    CREATUREALREADYOWNED = 3;    
    NOTTAMEABLE = 4;    
    ANOTHERSUMMONACTIVE = 5;    
    UNITSCANTTAME = 6;    
    NOPETAVAILABLE = 7;    
    INTERNALERROR = 8;    
    TOOHIGHLEVEL = 9;    
    DEAD = 10;    
    NOTDEAD = 11;    
    UNKNOWNERROR = 12;    
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `INVALIDCREATURE` | 1 (0x01) |  |  |
| `TOOMANY` | 2 (0x02) |  |  |
| `CREATUREALREADYOWNED` | 3 (0x03) |  |  |
| `NOTTAMEABLE` | 4 (0x04) |  |  |
| `ANOTHERSUMMONACTIVE` | 5 (0x05) |  |  |
| `UNITSCANTTAME` | 6 (0x06) |  |  |
| `NOPETAVAILABLE` | 7 (0x07) |  | not used in taming |
| `INTERNALERROR` | 8 (0x08) |  |  |
| `TOOHIGHLEVEL` | 9 (0x09) |  |  |
| `DEAD` | 10 (0x0A) |  | not used in taming |
| `NOTDEAD` | 11 (0x0B) |  | not used in taming |
| `UNKNOWNERROR` | 12 (0x0C) |  |  |
