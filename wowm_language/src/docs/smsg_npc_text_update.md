# SMSG_NPC_TEXT_UPDATE
## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_NPC_TEXT_UPDATE = 0x0180 {
    u32 text_id;
    f32 probability;
    NpcTextUpdate[8] texts;
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
| 0x04 | 4 / Little | u32 | text_id |  |
| 0x08 | 4 / Little | f32 | probability |  |
| 0x0C | ? / - | [NpcTextUpdate](npctextupdate.md)[8] | texts |  |
