## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_ITEM_PUSH_RESULT = 0x0166 {
    Guid guid;
    NewItemSource source;
    NewItemCreationType creation_type;
    NewItemChatAlert alert_chat;
    u8 bag_slot;
    u32 item_slot;
    u32 item_id;
    u32 item_suffix_factor;
    u32 item_random_property_id;
    u32 item_count;
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
| 0x04 | 8 / Little | [Guid](../spec/packed-guid.md) | guid |  |  |
| 0x0C | ? / - | [NewItemSource](newitemsource.md) | source |  |  |
| - | ? / - | [NewItemCreationType](newitemcreationtype.md) | creation_type |  |  |
| - | ? / - | [NewItemChatAlert](newitemchatalert.md) | alert_chat |  |  |
| - | 1 / - | u8 | bag_slot |  |  |
| - | 4 / Little | u32 | item_slot |  | mangoszero: item slot, but when added to stack: 0xFFFFFFFF |
| - | 4 / Little | u32 | item_id |  |  |
| - | 4 / Little | u32 | item_suffix_factor |  | mangoszero: SuffixFactor |
| - | 4 / Little | u32 | item_random_property_id |  | mangoszero: random item property id |
| - | 4 / Little | u32 | item_count |  |  |

