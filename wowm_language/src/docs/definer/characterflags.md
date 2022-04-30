## Client Version 1.12

### Wowm Representation
```rust,ignore
flag CharacterFlags : u32 {
    NONE = 0x00;    
    LOCKED_FOR_TRANSFER = 0x04;    
    HIDE_HELM = 0x400;    
    HIDE_CLOAK = 0x800;    
    GHOST = 0x2000;    
    RENAME = 0x4000;    
}

```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `LOCKED_FOR_TRANSFER` | 4 (0x04) |  |  |
| `HIDE_HELM` | 1024 (0x400) |  |  |
| `HIDE_CLOAK` | 2048 (0x800) |  |  |
| `GHOST` | 8192 (0x2000) |  |  |
| `RENAME` | 16384 (0x4000) |  |  |
