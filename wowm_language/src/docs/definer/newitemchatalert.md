## Client Version 1.12

### Wowm Representation
```rust,ignore
enum NewItemChatAlert : u32 {
    DO_NOT_SHOW = 0;    
    SHOW = 1;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `DO_NOT_SHOW` | 0 (0x00) |  |  |
| `SHOW` | 1 (0x01) |  |  |
