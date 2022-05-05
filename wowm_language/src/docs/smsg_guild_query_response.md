## Client Version 1.12

### Wowm Representation
```rust,ignore
smsg SMSG_GUILD_QUERY_RESPONSE = 0x0055 {
    u32 id;
    CString name;
    CString[10] rank_names;
    u32 emblem_style;
    u32 emblem_color;
    u32 border_style;
    u32 border_color;
    u32 background_color;
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
| 0x04 | 4 / Little | u32 | id |  |
| 0x08 | - / - | CString | name |  |
| - | ? / - | CString[10] | rank_names |  |
| - | 4 / Little | u32 | emblem_style |  |
| - | 4 / Little | u32 | emblem_color |  |
| - | 4 / Little | u32 | border_style |  |
| - | 4 / Little | u32 | border_color |  |
| - | 4 / Little | u32 | background_color |  |
