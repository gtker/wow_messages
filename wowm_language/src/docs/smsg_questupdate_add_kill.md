## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_QUESTUPDATE_ADD_KILL = 0x0199 {
    u32 quest_id;
    u32 create_id;
    u32 kill_count;
    u32 required_kill_count;
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
| 0x04 | 4 / Little | u32 | quest_id |  |  |
| 0x08 | 4 / Little | u32 | create_id |  | Unsure of name |
| 0x0C | 4 / Little | u32 | kill_count |  |  |
| 0x10 | 4 / Little | u32 | required_kill_count |  |  |
| 0x14 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
