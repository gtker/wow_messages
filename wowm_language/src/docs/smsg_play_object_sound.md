## Client Version 1.12

### Comment

vmangos: Nostalrius: ignored by client if unit is not loaded

### Wowm Representation
```rust,ignore
smsg SMSG_PLAY_OBJECT_SOUND = 0x0278 {
    u32 sound_id;
    Guid guid;
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
| 0x04 | 4 / Little | u32 | sound_id |  |  |
| 0x08 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |

