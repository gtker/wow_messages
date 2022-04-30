## Client Version 1.12

### Comment

According to cmangos: 'Default UI handler for this is bugged, args dont match. Gotta do a full update with SMSG_START_MIRROR_TIMER to avoid lua errors.

### Wowm Representation
```rust,ignore
smsg SMSG_PAUSE_MIRROR_TIMER = 0x01DA {
    TimerType timer;
    u8 is_frozen;
}
```
### Header
SMSG have a header of 4 bytes.

#### SMSG Header
| Offset | Size / Endianness | Type   | Name   | Description |
| ------ | ----------------- | ------ | ------ | ----------- |
| 0x00   | 2 / Big           | uint16 | size   | Size of the rest of the message including the opcode field but not including the size field.|
| 0x02   | 2 / Little        | uint16 | opcode | Opcode that determines which fields the message contains.|
### Body
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | ? / - | TimerType | timer |  |
| - | 1 / - | u8 | is_frozen |  |
