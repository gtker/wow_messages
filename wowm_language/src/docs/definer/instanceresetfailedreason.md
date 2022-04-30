## Client Version 1.12

## Wowm Representation
```rust,ignore
enum InstanceResetFailedReason : u8 {
    GENERAL = 0;    
    OFFLINE = 1;    
    ZONING = 2;    
    SILENTLY = 3;    
}

```
## Type
The basic type is `u8`, a 1 byte (8 bit) integer.
## Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `GENERAL` | 0 (0x00) |  | at least one player is in the instance |
| `OFFLINE` | 1 (0x01) |  | at least one player is offline |
| `ZONING` | 2 (0x02) |  | at least one player try to enter the instance (being teleported in) |
| `SILENTLY` | 3 (0x03) |  |  |
