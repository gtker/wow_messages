## Client Version 1.12

### Wowm Representation
```rust,ignore
enum RaidTargetUpdateType : u8 {
    PARTIAL = 0;    
    FULL = 1;    
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `PARTIAL` | 0 (0x00) |  |  |
| `FULL` | 1 (0x01) |  |  |
