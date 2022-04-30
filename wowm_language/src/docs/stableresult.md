## Client Version 1.12

### Wowm Representation
```rust,ignore
enum StableResult : u8 {
    ERR_MONEY = 0x01;
    ERR_STABLE = 0x06;
    SUCCESS_STABLE = 0x08;
    SUCCESS_UNSTABLE = 0x09;
    SUCCESS_BUY_SLOT = 0x0A;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `ERR_MONEY` | 1 (0x01) |  | you don't have enough money |
| `ERR_STABLE` | 6 (0x06) |  | currently used in most fail cases |
| `SUCCESS_STABLE` | 8 (0x08) |  | table success |
| `SUCCESS_UNSTABLE` | 9 (0x09) |  | unstable/swap success |
| `SUCCESS_BUY_SLOT` | 10 (0x0A) |  | buy slot success |
