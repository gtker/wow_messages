## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_START_MIRROR_TIMER = 0x01D9 {
    TimerType timer;
    u32 time_remaining;
    u32 duration;
    u32 scale;
    u8 is_frozen;
    u32 id;
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

| Offset | Size / Endianness | Type | Name | Description | Comment |
| ------ | ----------------- | ---- | ---- | ----------- | ------- |
| 0x04 | ? / - | [TimerType](timertype.md) | timer |  |  |
| - | 4 / Little | u32 | time_remaining |  |  |
| - | 4 / Little | u32 | duration |  |  |
| - | 4 / Little | u32 | scale |  |  |
| - | 1 / - | u8 | is_frozen |  |  |
| - | 4 / Little | u32 | id |  |  |

