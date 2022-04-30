## Client Version 1.12

### Wowm Representation
```rust,ignore
enum RaidGroupError : u32 {
    REQUIRED = 1;    
    FULL = 2;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `REQUIRED` | 1 (0x01) |  |  |
| `FULL` | 2 (0x02) |  |  |
