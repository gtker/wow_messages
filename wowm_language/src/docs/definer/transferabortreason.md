## Client Version 1.12

### Wowm Representation
```rust,ignore
enum TransferAbortReason : u8 {
    NONE = 0x00;    
    IS_FULL = 0x01;    
    NOT_FOUND = 0x02;    
    TOO_MANY_INSTANCES = 0x03;    
    ZONE_IS_IN_COMBAT = 0x05;    
}

```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `IS_FULL` | 1 (0x01) |  |  |
| `NOT_FOUND` | 2 (0x02) |  |  |
| `TOO_MANY_INSTANCES` | 3 (0x03) |  |  |
| `ZONE_IS_IN_COMBAT` | 5 (0x05) |  |  |
