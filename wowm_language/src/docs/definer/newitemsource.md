## Client Version 1.12

### Wowm Representation
```rust,ignore
enum NewItemSource : u32 {
    LOOTED = 0;    
    FROM_NPC = 1;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `LOOTED` | 0 (0x00) |  |  |
| `FROM_NPC` | 1 (0x01) |  |  |
