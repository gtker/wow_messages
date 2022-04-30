## Client Version 1.12

### Wowm Representation
```rust,ignore
enum ServerMessageType : u32 {
    SHUTDOWN_TIME = 1;    
    RESTART_TIME = 2;    
    CUSTOM = 3;    
    SHUTDOWN_CANCELLED = 4;    
    RESTART_CANCELLED = 5;    
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SHUTDOWN_TIME` | 1 (0x01) |  |  |
| `RESTART_TIME` | 2 (0x02) |  |  |
| `CUSTOM` | 3 (0x03) |  |  |
| `SHUTDOWN_CANCELLED` | 4 (0x04) |  |  |
| `RESTART_CANCELLED` | 5 (0x05) |  |  |
