## Client Version 1.12

### Wowm Representation
```rust,ignore
enum CorpseQueryResult : u8 {
    NOT_FOUND = 0;
    FOUND = 1;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NOT_FOUND` | 0 (0x00) |  |  |
| `FOUND` | 1 (0x01) |  |  |
