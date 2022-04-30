## Client Version 1.12

### Wowm Representation
```rust,ignore
enum QuestGiverStatus : u8 {
    NONE = 0;
    UNAVAILABLE = 1;
    CHAT = 2;
    INCOMPLETE = 3;
    REWARD_REP = 4;
    AVAILABLE = 5;
    REWARD_OLD = 6;
    REWARD2 = 7;
}
```
### Type
The basic type is `u8`, a 1 byte (8 bit) integer.
### Enumerators
| Enumerator | Value  | Description | Comment |
| --------- | -------- | ----------- | ------- |
| `NONE` | 0 (0x00) |  |  |
| `UNAVAILABLE` | 1 (0x01) |  |  |
| `CHAT` | 2 (0x02) |  |  |
| `INCOMPLETE` | 3 (0x03) |  |  |
| `REWARD_REP` | 4 (0x04) |  |  |
| `AVAILABLE` | 5 (0x05) |  |  |
| `REWARD_OLD` | 6 (0x06) |  | red dot on minimap |
| `REWARD2` | 7 (0x07) |  | yellow dot on minimap |
