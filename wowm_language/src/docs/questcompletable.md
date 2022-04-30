## Client Version 1.12

### Wowm Representation
```rust,ignore
enum QuestCompletable : u32 {
    NOT_COMPLETABLE = 0;
    COMPLETEABLE = 3;
}
```
### Type
The basic type is `u32`, a 4 byte (32 bit) little endian integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NOT_COMPLETABLE` | 0 (0x00) |  |  |
| `COMPLETEABLE` | 3 (0x03) |  |  |
