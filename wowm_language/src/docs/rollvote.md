## Client Version 1.12

### Wowm Representation
```rust,ignore
enum RollVote : u8 {
    PASS = 0;
    NEED = 1;
    GREED = 2;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `PASS` | 0 (0x00) |  |  |
| `NEED` | 1 (0x01) |  |  |
| `GREED` | 2 (0x02) |  |  |
