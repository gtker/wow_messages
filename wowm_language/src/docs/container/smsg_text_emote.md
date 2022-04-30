## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_TEXT_EMOTE = 0x0105 {
    Guid guid;
    u32 text_emote;
    Emote emote;
    u32 name_length;
    CString name;
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
| 0x04 | 8 / Little | Guid | guid |  |
| 0x0C | 4 / Little | u32 | text_emote |  |
| 0x10 | ? / - | Emote | emote |  |
| - | 4 / Little | u32 | name_length |  |
| - | - / - | CString | name |  |
