## Client Version 1.12

### Wowm Representation
```rust,ignore
enum GmTicketStatus : u32 {
    DBERROR = 0x00;    
    HASTEXT = 0x06;    
    DEFAULT = 0x0A;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `DBERROR` | 0 (0x00) |  |  |
| `HASTEXT` | 6 (0x06) |  |  |
| `DEFAULT` | 10 (0x0A) |  |  |
