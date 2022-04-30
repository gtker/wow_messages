## Client Version 1.12

### Wowm Representation
```rust,ignore
enum GroupLootSetting : u8 {
    FREE_FOR_ALL = 0;
    ROUND_ROBIN = 1;
    MASTER_LOOT = 2;
    GROUP_LOOT = 3;
    NEED_BEFORE_GREED = 4;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Original  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `FREE_FOR_ALL` | 0 (0x00) |  |  |
| `ROUND_ROBIN` | 1 (0x01) |  |  |
| `MASTER_LOOT` | 2 (0x02) |  |  |
| `GROUP_LOOT` | 3 (0x03) |  |  |
| `NEED_BEFORE_GREED` | 4 (0x04) |  |  |
