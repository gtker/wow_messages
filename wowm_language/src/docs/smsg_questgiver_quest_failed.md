# SMSG_QUESTGIVER_QUEST_FAILED
## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_QUESTGIVER_QUEST_FAILED = 0x0192 {
    u32 quest_id;
    QuestFailedReason reason;
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
| 0x04 | 4 / Little | u32 | quest_id |  |
| 0x08 | ? / - | [QuestFailedReason](questfailedreason.md) | reason |  |
