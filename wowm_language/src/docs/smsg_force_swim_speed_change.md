## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_FORCE_SWIM_SPEED_CHANGE = 0x00E6 {
    PackedGuid guid;
    u32 move_event;
    f32 speed;
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
| 0x04 | - / - | [PackedGuid](../spec/packed-guid.md) | guid |  |  |
| - | 4 / Little | u32 | move_event |  | cmangos/mangoszero/vmangos: set to 0<br/>cmangos/mangoszero/vmangos: moveEvent, NUM_PMOVE_EVTS = 0x39 |
| - | 4 / Little | f32 | speed |  |  |
