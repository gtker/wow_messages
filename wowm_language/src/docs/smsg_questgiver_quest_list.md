## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_QUESTGIVER_QUEST_LIST = 0x0185 {
    Guid npc;
    CString title;
    u32 emote_delay;
    u32 emote;
    u8 amount_of_entries;
    QuestItem[amount_of_entries] quest_items;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | npc |  |
| 0x0C | - / - | CString | title |  |
| - | 4 / Little | u32 | emote_delay |  |
| - | 4 / Little | u32 | emote |  |
| - | 1 / - | u8 | amount_of_entries |  |
| - | ? / - | [QuestItem](questitem.md)[amount_of_entries] | quest_items |  |
