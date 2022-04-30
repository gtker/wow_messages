## Client Version 1.12

### Wowm Representation
```rust,ignore
enum NewItemCreationType : u32 {
    RECEIVED = 0;    
    CREATED = 1;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `RECEIVED` | 0 (0x00) |  |  |
| `CREATED` | 1 (0x01) |  |  |
