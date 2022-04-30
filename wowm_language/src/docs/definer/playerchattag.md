## Client Version 1.12

## Wowm Representation
```rust,ignore
enum PlayerChatTag : u8 {
    NONE = 0;    
    AFK = 1;    
    DND = 2;    
    GM = 3;    
}

```
## Type
The basic type is `u8`, a 1 byte (8 bit) integer.
## Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `AFK` | 1 (0x01) |  |  |
| `DND` | 2 (0x02) |  |  |
| `GM` | 3 (0x03) |  |  |
