## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_QUEST_CONFIRM_ACCEPT = 0x019C {
    u32 quest_id;    
    CString quest_title;    
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
| Offset | Size / Endianness | Type | Name | Description |
| ------ | ----------------- | ---- | ---- | ----------- |
| 0x04 | 4 / Little | u32 | quest_id |  |
| 0x08 | - / - | CString | quest_title |  |
| - | 8 / Little | Guid | guid |  |
