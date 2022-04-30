## Client Version 1.12

### Wowm Representation
```rust,ignore
enum SimpleSpellCastResult : u8 {
    SUCCESS = 0;    
    FAILURE = 2;    
}

```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `SUCCESS` | 0 (0x00) |  |  |
| `FAILURE` | 2 (0x02) |  |  |
