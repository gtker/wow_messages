## Client Version 1.12

### Comment

According to cmangos: byte value (UNIT_FIELD_BYTES_2,0)

### Wowm Representation
```rust,ignore
enum SheathState : u8 {
    UNARMED = 0;    
    MELEE = 1;    
    RANGED = 2;    
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `UNARMED` | 0 (0x00) |  |  |
| `MELEE` | 1 (0x01) |  |  |
| `RANGED` | 2 (0x02) |  |  |
